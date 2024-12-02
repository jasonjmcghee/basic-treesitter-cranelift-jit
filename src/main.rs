#![feature(str_as_str)]

use adder_treesitter_cranelift::repl::{run_repl, test};
use miette::{Result as MietteResult};

fn main() -> MietteResult<()> {
    miette::set_panic_hook();
    test()
}