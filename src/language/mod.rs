mod error;
mod input_buffer;

use crate::language::error::{CalcErrorKind, CalculatorError};
use crate::language::input_buffer::InputBuffer;
use ahash::AHasher;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use dashmap::DashMap;
use miette::{NamedSource, Result as MietteResult};
use parking_lot::{Mutex, RwLock};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
    time::{Duration, Instant},
};
use tree_sitter::Node;

// ===== AST Structures =====

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOpKind,
        right: Box<Expr>,
    },
    Parenthesized(Box<Expr>),
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Integer(i) => {
                0_u8.hash(state);
                i.hash(state);
            }
            Expr::Float(f) => {
                1_u8.hash(state);
                f.to_bits().hash(state);
            }
            Expr::BinaryOp { left, op, right } => {
                2_u8.hash(state);
                left.hash(state);
                op.hash(state);
                right.hash(state);
            }
            Expr::Parenthesized(inner) => {
                3_u8.hash(state);
                inner.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// ===== Value System =====

#[derive(Debug)]
pub enum CalcValue {
    Integer(i64),
    Float(f64),
}

impl Default for CalcValue {
    fn default() -> Self {
        CalcValue::Integer(0)
    }
}

impl PartialEq for CalcValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CalcValue::Integer(a), CalcValue::Integer(b)) => a == b,
            (CalcValue::Float(a), CalcValue::Float(b)) => (a - b).abs() < f64::EPSILON,
            _ => false,
        }
    }
}

// ===== JIT Compilation =====

pub struct CompilationCache {
    last_tree: Option<tree_sitter::Tree>,
    function_cache: DashMap<u64, Arc<CompiledFunction>>,
}

pub enum CompiledFnPtr {
    Integer(unsafe fn() -> i64),
    Float(unsafe fn() -> f64),
}

pub struct CompiledFunction {
    code_ptr: CompiledFnPtr,
    last_used: Arc<Mutex<Instant>>,
}

impl CompiledFunction {
    fn new_int(code_ptr: unsafe fn() -> i64) -> Self {
        Self {
            code_ptr: CompiledFnPtr::Integer(code_ptr),
            last_used: Arc::new(Mutex::new(Instant::now())),
        }
    }

    fn new_float(code_ptr: unsafe fn() -> f64) -> Self {
        Self {
            code_ptr: CompiledFnPtr::Float(code_ptr),
            last_used: Arc::new(Mutex::new(Instant::now())),
        }
    }

    fn update(&self) {
        *self.last_used.lock() = Instant::now();
    }

    unsafe fn call(&self) -> CalcValue {
        match self.code_ptr {
            CompiledFnPtr::Integer(ptr) => CalcValue::Integer(ptr()),
            CompiledFnPtr::Float(ptr) => CalcValue::Float(ptr()),
        }
    }
}

// ===== Parser Implementation =====

fn collect_error_nodes<'a>(node: Node<'a>, errors: &mut Vec<Node<'a>>) {
    if node.kind() == "ERROR" {
        errors.push(node);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_error_nodes(child, errors);
    }
}

// ===== Calculator Implementation =====

pub struct Calculator {
    pub parser: tree_sitter::Parser,
    source: NamedSource<String>,
    cache: CompilationCache,
    jit_module: Arc<RwLock<JITModule>>,
    builder_context: Arc<Mutex<FunctionBuilderContext>>,
    input_buffer: InputBuffer,
}

impl Calculator {
    pub fn new() -> MietteResult<Self> {
        let mut parser = tree_sitter::Parser::new();
        let source = NamedSource::new("calculator", String::new());

        parser
            .set_language(&tree_sitter_calculator::LANGUAGE.into())
            .map_err(|e| CalculatorError {
                src: source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::CompilationError(e.to_string()),
                help: None,
            })?;

        let mut flag_builder = settings::builder();
        flag_builder
            .set("use_colocated_libcalls", "false")
            .map_err(|e| CalculatorError {
                src: source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        flag_builder
            .set("is_pic", "false")
            .map_err(|e| CalculatorError {
                src: source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        let isa_builder = cranelift_native::builder().map_err(|e| CalculatorError {
            src: source.clone(),
            span: (0, 0).into(),
            kind: CalcErrorKind::JitError(e.to_string()),
            help: None,
        })?;

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| CalculatorError {
                src: source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let jit_module = JITModule::new(builder);

        Ok(Self {
            parser,
            source,
            cache: CompilationCache {
                last_tree: None,
                function_cache: DashMap::new(),
            },
            jit_module: Arc::new(RwLock::new(jit_module)),
            builder_context: Arc::new(Mutex::new(FunctionBuilderContext::new())),
            input_buffer: InputBuffer::new(),
        })
    }

    pub fn update_input(
        &mut self,
        new_input: &str,
        edit_pos: usize,
        old_end: usize,
        new_end: usize,
    ) -> MietteResult<CalcValue> {
        self.input_buffer
            .update(new_input, edit_pos, old_end, new_end);

        let edit = tree_sitter::InputEdit {
            start_byte: edit_pos,
            old_end_byte: old_end,
            new_end_byte: new_end,
            start_position: tree_sitter::Point::new(0, edit_pos),
            old_end_position: tree_sitter::Point::new(0, old_end),
            new_end_position: tree_sitter::Point::new(0, new_end),
        };

        let tree = if let Some(old_tree) = &mut self.cache.last_tree {
            old_tree.edit(&edit);
            self.parser
                .parse(self.input_buffer.as_str(), Some(old_tree))
        } else {
            self.parser.parse(self.input_buffer.as_str(), None)
        }
        .ok_or_else(|| CalculatorError {
            src: self.source.clone(),
            span: (0, new_input.len()).into(),
            kind: CalcErrorKind::ParseError("Failed to parse input".to_string()),
            help: Some("Make sure your expression is syntactically valid".to_string()),
        })?;

        self.cache.last_tree = Some(tree.clone());

        let mut errors = Vec::new();
        collect_error_nodes(tree.root_node(), &mut errors);

        if !errors.is_empty() {
            let error_node = &errors[0];
            let span = (
                error_node.start_byte(),
                error_node.end_byte() - error_node.start_byte(),
            );
            let error_message = error_node
                .utf8_text(self.input_buffer.as_str().as_bytes())
                .unwrap_or("Syntax error")
                .to_string();

            return Err(CalculatorError {
                src: self.source.clone(),
                span: span.into(),
                kind: CalcErrorKind::ParseError(format!("Syntax error near '{}'", error_message)),
                help: Some("Ensure that your expression follows the correct syntax.".into()),
            }
            .into());
        }

        let ast = self.node_to_expr(new_input, tree.root_node())?;
        let ast_hash = self.hash_ast(&ast);

        if let Some(cached_fn) = self.cache.function_cache.get(&ast_hash) {
            let entry = cached_fn.value().clone();
            entry.update();
            return Ok(unsafe { entry.call() });
        }

        let compiled_fn = self.compile_expr(new_input, ast)?;
        let result = unsafe { compiled_fn.call() };
        self.cache
            .function_cache
            .insert(ast_hash, Arc::new(compiled_fn));
        self.cleanup_cache();

        Ok(result)
    }

    pub fn node_to_expr(&self, input: &str, node: Node) -> MietteResult<Expr> {
        let span = node.start_byte()..node.end_byte();
        let node_text = node.utf8_text(input.as_bytes()).unwrap_or("invalid utf8");

        match node.kind() {
            "ERROR" => {
                let error_text = node
                    .utf8_text(input.as_bytes())
                    .unwrap_or("Syntax error")
                    .to_string();
                Err(CalculatorError {
                    src: self.source.clone(),
                    span: span.into(),
                    kind: CalcErrorKind::ParseError(format!("Syntax error near '{}'", error_text)),
                    help: Some("Check the expression syntax.".into()),
                })?
            }
            "source" => {
                let child = node.child(0).ok_or_else(|| CalculatorError {
                    src: self.source.clone(),
                    span: (span.start, span.end - span.start).into(),
                    kind: CalcErrorKind::ParseError("Empty expression".into()),
                    help: Some("Expression cannot be empty".into()),
                })?;
                self.node_to_expr(input, child)
            }
            "expression" => {
                let child = node.child(0).ok_or_else(|| CalculatorError {
                    src: self.source.clone(),
                    span: (span.start, span.end - span.start).into(),
                    kind: CalcErrorKind::ParseError("Empty expression node".into()),
                    help: Some("Expression node must contain a value".into()),
                })?;
                self.node_to_expr(input, child)
            }
            "parenthesized_expression" => {
                // Find the inner expression (skip the parentheses)
                let inner = node
                    .child_by_field_name("inner")
                    .ok_or_else(|| CalculatorError {
                        src: NamedSource::new("calculator", input.to_string()),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::ParseError("Empty parentheses".into()),
                        help: Some("Parentheses cannot be empty".into()),
                    })?;
                let inner_expr = self.node_to_expr(input, inner)?;
                Ok(Expr::Parenthesized(Box::new(inner_expr)))
            }
            "number" => {
                if let Ok(expr) = node_text.parse().map(Expr::Integer) {
                    Ok(expr)
                } else {
                    Err(CalculatorError {
                        src: self.source.clone(),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::NumberError("Failed to parse integer".into()),
                        help: Some("Make sure the number is a valid integer".into()),
                    })?
                }
            }
            "float" => {
                if let Ok(expr) = node_text.parse().map(Expr::Float) {
                    Ok(expr)
                } else {
                    Err(CalculatorError {
                        src: self.source.clone(),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::NumberError("Failed to parse float".into()),
                        help: Some("Make sure the number is a valid floating point number".into()),
                    })?
                }
            }
            "binary_expression" => {
                let left = node
                    .child_by_field_name("left")
                    .ok_or_else(|| CalculatorError {
                        src: self.source.clone(),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::ParseError("Missing left operand".into()),
                        help: Some("Binary expression must have a left operand".into()),
                    })?;
                let left_expr = self.node_to_expr(input, left)?;

                let op = if let Some(op_text) = node
                    .child_by_field_name("operator")
                    .and_then(|n| n.utf8_text(input.as_bytes()).ok())
                {
                    match op_text {
                        "+" => Ok(BinaryOpKind::Add),
                        "-" => Ok(BinaryOpKind::Subtract),
                        "*" => Ok(BinaryOpKind::Multiply),
                        "/" => Ok(BinaryOpKind::Divide),
                        _ => Err(CalculatorError {
                            src: self.source.clone(),
                            span: (span.start, span.end - span.start).into(),
                            kind: CalcErrorKind::InvalidOperator(op_text.to_string()),
                            help: Some("Only +, -, *, and / operators are supported".into()),
                        }),
                    }
                } else {
                    Err(CalculatorError {
                        src: self.source.clone(),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::ParseError("Missing operator".into()),
                        help: Some("Binary expression must have an operator".into()),
                    })
                }?;

                let right = node
                    .child_by_field_name("right")
                    .ok_or_else(|| CalculatorError {
                        src: self.source.clone(),
                        span: (span.start, span.end - span.start).into(),
                        kind: CalcErrorKind::ParseError("Missing right operand".into()),
                        help: Some("Binary expression must have a right operand".into()),
                    })?;
                let right_expr = self.node_to_expr(input, right)?;

                Ok(Expr::BinaryOp {
                    left: Box::new(left_expr),
                    op,
                    right: Box::new(right_expr),
                })
            }
            _ => Err(CalculatorError {
                src: self.source.clone(),
                span: (span.start, span.end - span.start).into(),
                kind: CalcErrorKind::ParseError(format!("Unexpected node type '{}'", node.kind())),
                help: Some("Expression must be a number, float, or binary operation".into()),
            })?,
        }
    }

    pub fn compile_expr(&self, input: &str, expr: Expr) -> MietteResult<CompiledFunction> {
        let mut jit_module = self.jit_module.write();
        let mut ctx = jit_module.make_context();

        let (return_type, is_float) = self.determine_type(&expr)?;
        let signature_type = if is_float { types::F64 } else { types::I64 };

        ctx.func
            .signature
            .returns
            .push(AbiParam::new(signature_type));

        let mut builder_context = self.builder_context.lock();
        let mut func_builder = FunctionBuilder::new(&mut ctx.func, &mut *builder_context);

        let entry_block = func_builder.create_block();
        func_builder.append_block_params_for_function_params(entry_block);
        func_builder.switch_to_block(entry_block);
        func_builder.seal_block(entry_block);

        let (_, result) = self.compile_node(input, &mut func_builder, &expr)?;
        func_builder.ins().return_(&[result]);
        func_builder.finalize();

        let id = jit_module
            .declare_function(
                &format!("calc_{}", fastrand::u64(..)),
                Linkage::Export,
                &ctx.func.signature,
            )
            .map_err(|e| CalculatorError {
                src: self.source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        jit_module
            .define_function(id, &mut ctx)
            .map_err(|e| CalculatorError {
                src: self.source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        jit_module
            .finalize_definitions()
            .map_err(|e| CalculatorError {
                src: self.source.clone(),
                span: (0, 0).into(),
                kind: CalcErrorKind::JitError(e.to_string()),
                help: None,
            })?;

        let fn_ptr = jit_module.get_finalized_function(id);

        Ok(match return_type {
            CalcValue::Integer(_) => {
                CompiledFunction::new_int(unsafe { std::mem::transmute(fn_ptr) })
            }
            CalcValue::Float(_) => {
                CompiledFunction::new_float(unsafe { std::mem::transmute(fn_ptr) })
            }
        })
    }

    fn determine_type(&self, expr: &Expr) -> MietteResult<(CalcValue, bool)> {
        Ok(match expr {
            Expr::Integer(n) => (CalcValue::Integer(*n), false),
            Expr::Float(x) => (CalcValue::Float(*x), true),
            Expr::BinaryOp { left, op, right } => {
                let (_left_type, left_float) = self.determine_type(left)?;
                let (_right_type, right_float) = self.determine_type(right)?;
                if left_float || right_float || *op == BinaryOpKind::Divide {
                    (CalcValue::Float(0.0), true)
                } else {
                    (CalcValue::Integer(0), false)
                }
            }
            Expr::Parenthesized(inner) => self.determine_type(inner)?,
        })
    }

    fn compile_node(
        &self,
        input: &str,
        builder: &mut FunctionBuilder,
        expr: &Expr,
    ) -> MietteResult<(CalcValue, cranelift::prelude::Value)> {
        match expr {
            Expr::Integer(n) => {
                let v = builder.ins().iconst(types::I64, *n);
                Ok((CalcValue::Integer(*n), v))
            }
            Expr::Float(x) => {
                let v = builder.ins().f64const(*x);
                Ok((CalcValue::Float(*x), v))
            }
            Expr::BinaryOp { left, op, right } => {
                let (left_val, left_ir) = self.compile_node(input, builder, left)?;
                let (right_val, right_ir) = self.compile_node(input, builder, right)?;

                let needs_float = matches!(op, BinaryOpKind::Divide)
                    || matches!(&left_val, CalcValue::Float(_))
                    || matches!(&right_val, CalcValue::Float(_));

                let (final_left, final_right) = if needs_float {
                    let float_left = match &left_val {
                        CalcValue::Integer(_) => builder.ins().fcvt_from_sint(types::F64, left_ir),
                        CalcValue::Float(_) => left_ir,
                    };
                    let float_right = match &right_val {
                        CalcValue::Integer(_) => builder.ins().fcvt_from_sint(types::F64, right_ir),
                        CalcValue::Float(_) => right_ir,
                    };
                    (float_left, float_right)
                } else {
                    (left_ir, right_ir)
                };

                let result = match (op, needs_float) {
                    (BinaryOpKind::Add, false) => builder.ins().iadd(final_left, final_right),
                    (BinaryOpKind::Subtract, false) => builder.ins().isub(final_left, final_right),
                    (BinaryOpKind::Multiply, false) => builder.ins().imul(final_left, final_right),
                    (BinaryOpKind::Add, true) => builder.ins().fadd(final_left, final_right),
                    (BinaryOpKind::Subtract, true) => builder.ins().fsub(final_left, final_right),
                    (BinaryOpKind::Multiply, true) => builder.ins().fmul(final_left, final_right),
                    (BinaryOpKind::Divide, _) => builder.ins().fdiv(final_left, final_right),
                };

                Ok((
                    if needs_float {
                        CalcValue::Float(0.0)
                    } else {
                        CalcValue::Integer(0)
                    },
                    result,
                ))
            }
            Expr::Parenthesized(inner) => self.compile_node(input, builder, inner),
        }
    }

    fn cleanup_cache(&self) {
        let cache = Arc::new(self.cache.function_cache.clone());
        std::thread::spawn(move || {
            let now = Instant::now();
            cache.retain(|_, v| now.duration_since(*v.last_used.lock()) < Duration::from_secs(300));
        });
    }

    fn hash_ast(&self, expr: &Expr) -> u64 {
        let mut hasher = AHasher::default();
        expr.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::error::{CalcErrorKind, CalculatorError};
    use crossterm::terminal;
    use miette::GraphicalReportHandler;

    // Helper function to create a controlled test environment
    fn setup_test_calculator() -> Calculator {
        Calculator::new().expect("Failed to create calculator")
    }

    mod parser_tests {
        use super::*;

        #[test]
        fn test_parse_single_number() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("42", 0, 0, 2);
            assert!(matches!(result, Ok(CalcValue::Integer(42))));
        }

        #[test]
        fn test_parse_single_float() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("42.5", 0, 0, 4);
            assert!(matches!(result, Ok(CalcValue::Float(42.5))));
        }

        #[test]
        fn test_parse_negative_number() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("-42", 0, 0, 3);
            assert!(matches!(result, Ok(CalcValue::Integer(-42))));
        }

        #[test]
        fn test_parse_invalid_input() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("abc", 0, 0, 3);
            assert!(matches!(result, Err(_)));
        }
    }

    mod expression_tests {
        use super::*;

        #[test]
        fn test_simple_addition() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("5 + 3", 0, 0, 5);
            assert!(matches!(result, Ok(CalcValue::Integer(8))));
        }

        #[test]
        fn test_simple_multiplication() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("5 * 3", 0, 0, 5);
            assert!(matches!(result, Ok(CalcValue::Integer(15))));
        }

        #[test]
        fn test_mixed_operations() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2 + 3 * 4", 0, 0, 9);
            assert!(matches!(result, Ok(CalcValue::Integer(14))));
        }

        #[test]
        fn test_float_operations() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2.5 + 3.5", 0, 0, 9);
            if let Ok(CalcValue::Float(val)) = result {
                assert!((val - 6.0).abs() < f64::EPSILON);
            } else {
                panic!("Expected float result");
            }
        }

        #[test]
        fn test_mixed_types() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2 + 3.5", 0, 0, 7);
            if let Ok(CalcValue::Float(val)) = result {
                assert!((val - 5.5).abs() < f64::EPSILON);
            } else {
                panic!("Expected float result");
            }
        }
    }

    mod type_determination_tests {
        use super::*;

        #[test]
        fn test_determine_type_integer() {
            let calc = setup_test_calculator();
            let expr = Expr::Integer(42);
            let result = calc.determine_type(&expr);
            assert!(matches!(result, Ok((CalcValue::Integer(42), false))));
        }

        #[test]
        fn test_determine_type_float() {
            let calc = setup_test_calculator();
            let expr = Expr::Float(42.5);
            let result = calc.determine_type(&expr);
            assert!(matches!(result, Ok((CalcValue::Float(42.5), true))));
        }

        #[test]
        fn test_determine_type_mixed_operation() {
            let calc = setup_test_calculator();
            let expr = Expr::BinaryOp {
                left: Box::new(Expr::Integer(2)),
                op: BinaryOpKind::Add,
                right: Box::new(Expr::Float(3.5)),
            };
            let result = calc.determine_type(&expr);
            assert!(matches!(result, Ok((CalcValue::Float(_), true))));
        }
    }

    mod cache_tests {
        use super::*;
        use std::thread;
        use std::time::Duration;

        #[test]
        fn test_cache_hit() {
            let mut calc = setup_test_calculator();
            // First evaluation should compile
            let result1 = calc.update_input("2 + 3", 0, 0, 5);
            assert!(matches!(result1, Ok(CalcValue::Integer(5))));

            // Second evaluation should use cache
            let result2 = calc.update_input("2 + 3", 0, 0, 5);
            assert!(matches!(result2, Ok(CalcValue::Integer(5))));
        }

        #[test]
        fn test_cache_cleanup() {
            let mut calc = setup_test_calculator();
            calc.update_input("2 + 3", 0, 0, 5).unwrap();

            // Wait for cleanup thread to run
            thread::sleep(Duration::from_secs(1));

            // Check if cache was cleaned
            assert!(calc.cache.function_cache.len() <= 1);
        }
    }

    mod error_handling_tests {
        use super::*;

        #[test]
        fn test_syntax_error() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2 +", 0, 0, 3);
            assert!(matches!(result, Err(_)));
        }

        #[test]
        fn test_invalid_operator() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2 / 3", 0, 0, 5);
            assert!(matches!(result, Err(_))); // First check if it's an error

            if let Err(e) = result {
                let diagnostic_msg = format!("{:?}", e);
                assert!(diagnostic_msg.contains("Invalid operator")); // Then check the error message
            } else {
                panic!("Expected error for invalid operator");
            }
        }

        #[test]
        fn test_invalid_number() {
            let mut calc = setup_test_calculator();
            let result = calc.update_input("2.a", 0, 0, 3);
            assert!(matches!(result, Err(_)));
        }
    }

    mod incremental_update_tests {
        use super::*;

        #[test]
        fn test_incremental_input() {
            let mut calc = setup_test_calculator();

            // Type "2"
            let result1 = calc.update_input("2", 0, 0, 1);
            assert!(matches!(result1, Ok(CalcValue::Integer(2))));

            // Type "2 + "
            let result2 = calc.update_input("2 + ", 1, 1, 4);
            assert!(matches!(result2, Err(_))); // Incomplete expression

            // Type "2 + 3"
            let result3 = calc.update_input("2 + 3", 4, 4, 5);
            assert!(matches!(result3, Ok(CalcValue::Integer(5))));
        }

        #[test]
        fn test_backspace() {
            let mut calc = setup_test_calculator();

            // Type "2 + 3"
            calc.update_input("2 + 3", 0, 0, 5).unwrap();

            // Backspace to "2 + "
            let result = calc.update_input("2 + ", 0, 5, 4);
            assert!(matches!(result, Err(_))); // Incomplete expression
        }
    }

    // Helper function to simulate calculator input and get formatted output
    fn simulate_calc_input(input: &str) -> String {
        let mut calculator = Calculator::new().unwrap();
        let result = calculator.update_input(input, 0, 0, input.len());

        match result {
            Ok(value) => format!("= {:?}", value),
            Err(error) => {
                let mut output = String::new();
                GraphicalReportHandler::new()
                    .with_context_lines(2)
                    .with_links(false)
                    .render_report(&mut output, &*error)
                    .unwrap();
                output
            }
        }
    }

    #[test]
    fn test_valid_integer_calculation() {
        let output = simulate_calc_input("1 + 2");
        assert!(output.contains("= Integer(3)"));
    }

    #[test]
    fn test_valid_float_calculation() {
        let output = simulate_calc_input("1.5 + 2.5");
        assert!(output.contains("= Float(4.0)"));
    }

    #[test]
    fn test_syntax_error() {
        let output = simulate_calc_input("1 +");
        assert!(output.contains("Invalid number"));
        assert!(output.contains("Calculator error"));
    }

    #[test]
    fn test_invalid_operator() {
        let output = simulate_calc_input("1 / 2");
        assert!(output.contains("Invalid operator"));
    }

    #[test]
    fn test_mixed_type_calculation() {
        let output = simulate_calc_input("1 + 2.5");
        assert!(output.contains("= Float(3.5)"));
    }

    #[test]
    fn test_complex_expression() {
        let output = simulate_calc_input("1 + 2 * 3");
        assert!(output.contains("= Integer(7)"));
    }

    // Test error formatting specifically
    #[test]
    fn test_error_formatting() {
        let mut handler = GraphicalReportHandler::new()
            .with_context_lines(2)
            .with_links(false);

        let error = CalculatorError {
            src: NamedSource::new("test", "1 +".to_string()),
            span: (0, 2).into(),
            kind: CalcErrorKind::ParseError("Failed to parse input".to_string()),
            help: Some("Complete the expression".to_string()),
        };

        let mut output = String::new();
        handler.render_report(&mut output, &error).unwrap();

        // Verify error components are present
        assert!(output.contains("calculator::error"));
        assert!(output.contains("Invalid syntax"));
        assert!(output.contains("Complete the expression"));
    }

    // Test terminal setup
    #[test]
    fn test_terminal_configuration() {
        // Save current terminal state
        let raw_mode = terminal::is_raw_mode_enabled().unwrap_or(false);

        // Test terminal setup
        assert!(Calculator::new().is_ok());

        // Verify terminal state was preserved
        assert_eq!(terminal::is_raw_mode_enabled().unwrap_or(false), raw_mode);
    }

    #[test]
    fn test_parentheses() {
        let mut calc = Calculator::new().unwrap();

        // Basic parentheses
        let result = calc.update_input("(2 + 3)", 0, 0, 7);
        assert!(matches!(result, Ok(CalcValue::Integer(5))));

        // Nested parentheses
        let result = calc.update_input("(2 + (3 * 4))", 0, 0, 13);
        assert!(matches!(result, Ok(CalcValue::Integer(14))));

        // Multiple parentheses
        let result = calc.update_input("(2 + 3) * (4 + 5)", 0, 0, 17);
        assert!(matches!(result, Ok(CalcValue::Integer(45))));

        // Mixed types in parentheses
        let result = calc.update_input("(2.5 + 1.5) * 3", 0, 0, 15);
        if let Ok(CalcValue::Float(val)) = result {
            assert!((val - 12.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected float result");
        }
    }

    #[test]
    fn test_parentheses_errors() {
        let mut calc = Calculator::new().unwrap();

        // Empty parentheses
        assert!(matches!(calc.update_input("()", 0, 0, 2), Err(_)));

        // Unclosed parentheses
        assert!(matches!(calc.update_input("(2 + 3", 0, 0, 6), Err(_)));

        // Unopened parentheses
        assert!(matches!(calc.update_input("2 + 3)", 0, 0, 6), Err(_)));
    }

    #[test]
    fn test_precedence_with_parentheses() {
        let mut calc = Calculator::new().unwrap();

        // Without parentheses: 2 + 3 * 4 = 14
        let result1 = calc.update_input("2 + 3 * 4", 0, 0, 9);
        assert!(matches!(result1, Ok(CalcValue::Integer(14))));

        // With parentheses: (2 + 3) * 4 = 20
        let result2 = calc.update_input("(2 + 3) * 4", 0, 0, 11);
        assert!(matches!(result2, Ok(CalcValue::Integer(20))));
    }
}
