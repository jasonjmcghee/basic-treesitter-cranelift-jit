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
    source: $ => $.expression,  // Explicitly mark the root

    expression: $ => choice(
        $.number,
        $.float,
        $.binary_expression,
    ),

    number: $ => choice(
        /[0-9]+/,
        seq('-', /[0-9]+/),  // Handle negative numbers
    ),

    float: $ => choice(
        /[0-9]*\.[0-9]+/,
        seq('-', /[0-9]*\.[0-9]+/),  // Handle negative floats
    ),

    binary_expression: $ => choice(
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