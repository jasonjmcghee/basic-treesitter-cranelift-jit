/**
 * @file Calculator grammar for tree-sitter
 * @author Jason McGhee <mcghee.j@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
module.exports = grammar({
    name: 'calculator',

    extras: $ => [
        /\s/
    ],

    rules: {
        source: $ => $.expression,

        expression: $ => choice(
            $.number,
            $.float,
            $.parenthesized_expression,
            $.binary_expression,
        ),

        parenthesized_expression: $ => seq(
            '(',
            field('inner', $.expression),
            ')'
        ),

        number: $ => choice(
            /[0-9]+/,
            seq('-', /[0-9]+/),
        ),

        float: $ => choice(
            /[0-9]*\.[0-9]+/,
            seq('-', /[0-9]*\.[0-9]+/),
        ),

        binary_expression: $ => choice(
            ..."!@#$%^&,.=_~".split("").map((op) =>
                prec.left(0, seq(
                    field('left', $.expression),
                    field('operator', op),
                    field('right', $.expression)
                ))
            ),
            prec.left(1, seq(
                field('left', $.expression),
                field('operator', "-"),
                field('right', $.expression)
            )),
            prec.left(2, seq(
                field('left', $.expression),
                field('operator', "+"),
                field('right', $.expression)
            )),
            prec.left(3, seq(
                field('left', $.expression),
                field('operator', "/"),
                field('right', $.expression)
            )),
            prec.left(4, seq(
                field('left', $.expression),
                field('operator', "*"),
                field('right', $.expression)
            )),
        )
    }
});