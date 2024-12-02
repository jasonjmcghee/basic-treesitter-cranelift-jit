#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 23
#define LARGE_STATE_COUNT 15
#define SYMBOL_COUNT 28
#define ALIAS_COUNT 0
#define TOKEN_COUNT 22
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 3

enum ts_symbol_identifiers {
  anon_sym_LPAREN = 1,
  anon_sym_RPAREN = 2,
  aux_sym_number_token1 = 3,
  anon_sym_DASH = 4,
  aux_sym_float_token1 = 5,
  anon_sym_BANG = 6,
  anon_sym_AT = 7,
  anon_sym_POUND = 8,
  anon_sym_DOLLAR = 9,
  anon_sym_PERCENT = 10,
  anon_sym_CARET = 11,
  anon_sym_AMP = 12,
  anon_sym_COMMA = 13,
  anon_sym_DOT = 14,
  anon_sym_EQ = 15,
  anon_sym__ = 16,
  anon_sym_TILDE = 17,
  anon_sym_PIPE = 18,
  anon_sym_PLUS = 19,
  anon_sym_SLASH = 20,
  anon_sym_STAR = 21,
  sym_source = 22,
  sym_expression = 23,
  sym_parenthesized_expression = 24,
  sym_number = 25,
  sym_float = 26,
  sym_binary_expression = 27,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [aux_sym_number_token1] = "number_token1",
  [anon_sym_DASH] = "-",
  [aux_sym_float_token1] = "float_token1",
  [anon_sym_BANG] = "!",
  [anon_sym_AT] = "@",
  [anon_sym_POUND] = "#",
  [anon_sym_DOLLAR] = "$",
  [anon_sym_PERCENT] = "%",
  [anon_sym_CARET] = "^",
  [anon_sym_AMP] = "&",
  [anon_sym_COMMA] = ",",
  [anon_sym_DOT] = ".",
  [anon_sym_EQ] = "=",
  [anon_sym__] = "_",
  [anon_sym_TILDE] = "~",
  [anon_sym_PIPE] = "|",
  [anon_sym_PLUS] = "+",
  [anon_sym_SLASH] = "/",
  [anon_sym_STAR] = "*",
  [sym_source] = "source",
  [sym_expression] = "expression",
  [sym_parenthesized_expression] = "parenthesized_expression",
  [sym_number] = "number",
  [sym_float] = "float",
  [sym_binary_expression] = "binary_expression",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [anon_sym_DASH] = anon_sym_DASH,
  [aux_sym_float_token1] = aux_sym_float_token1,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_POUND] = anon_sym_POUND,
  [anon_sym_DOLLAR] = anon_sym_DOLLAR,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym__] = anon_sym__,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [sym_source] = sym_source,
  [sym_expression] = sym_expression,
  [sym_parenthesized_expression] = sym_parenthesized_expression,
  [sym_number] = sym_number,
  [sym_float] = sym_float,
  [sym_binary_expression] = sym_binary_expression,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_number_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_float_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOLLAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym__] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_parenthesized_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_float] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
};

enum ts_field_identifiers {
  field_inner = 1,
  field_left = 2,
  field_operator = 3,
  field_right = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_inner] = "inner",
  [field_left] = "left",
  [field_operator] = "operator",
  [field_right] = "right",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_inner, 1},
  [1] =
    {field_left, 0},
    {field_operator, 1},
    {field_right, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(3);
      ADVANCE_MAP(
        '!', 10,
        '#', 12,
        '$', 13,
        '%', 14,
        '&', 16,
        '(', 4,
        ')', 5,
        '*', 25,
        '+', 23,
        ',', 17,
        '-', 8,
        '.', 18,
        '/', 24,
        '=', 19,
        '@', 11,
        '^', 15,
        '_', 20,
        '|', 22,
        '~', 21,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 1:
      if (lookahead == '(') ADVANCE(4);
      if (lookahead == '-') ADVANCE(8);
      if (lookahead == '.') ADVANCE(2);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 2:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(aux_sym_float_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DOLLAR);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym__);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 1},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [anon_sym_DOLLAR] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym__] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(22),
    [sym_expression] = STATE(13),
    [sym_parenthesized_expression] = STATE(4),
    [sym_number] = STATE(4),
    [sym_float] = STATE(4),
    [sym_binary_expression] = STATE(4),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [2] = {
    [ts_builtin_sym_end] = ACTIONS(11),
    [anon_sym_RPAREN] = ACTIONS(11),
    [anon_sym_DASH] = ACTIONS(11),
    [anon_sym_BANG] = ACTIONS(11),
    [anon_sym_AT] = ACTIONS(11),
    [anon_sym_POUND] = ACTIONS(11),
    [anon_sym_DOLLAR] = ACTIONS(11),
    [anon_sym_PERCENT] = ACTIONS(11),
    [anon_sym_CARET] = ACTIONS(11),
    [anon_sym_AMP] = ACTIONS(11),
    [anon_sym_COMMA] = ACTIONS(11),
    [anon_sym_DOT] = ACTIONS(11),
    [anon_sym_EQ] = ACTIONS(11),
    [anon_sym__] = ACTIONS(11),
    [anon_sym_TILDE] = ACTIONS(11),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_PLUS] = ACTIONS(11),
    [anon_sym_SLASH] = ACTIONS(11),
    [anon_sym_STAR] = ACTIONS(11),
  },
  [3] = {
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_RPAREN] = ACTIONS(13),
    [anon_sym_DASH] = ACTIONS(13),
    [anon_sym_BANG] = ACTIONS(13),
    [anon_sym_AT] = ACTIONS(13),
    [anon_sym_POUND] = ACTIONS(13),
    [anon_sym_DOLLAR] = ACTIONS(13),
    [anon_sym_PERCENT] = ACTIONS(13),
    [anon_sym_CARET] = ACTIONS(13),
    [anon_sym_AMP] = ACTIONS(13),
    [anon_sym_COMMA] = ACTIONS(13),
    [anon_sym_DOT] = ACTIONS(13),
    [anon_sym_EQ] = ACTIONS(13),
    [anon_sym__] = ACTIONS(13),
    [anon_sym_TILDE] = ACTIONS(13),
    [anon_sym_PIPE] = ACTIONS(13),
    [anon_sym_PLUS] = ACTIONS(13),
    [anon_sym_SLASH] = ACTIONS(13),
    [anon_sym_STAR] = ACTIONS(13),
  },
  [4] = {
    [ts_builtin_sym_end] = ACTIONS(15),
    [anon_sym_RPAREN] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(15),
    [anon_sym_BANG] = ACTIONS(15),
    [anon_sym_AT] = ACTIONS(15),
    [anon_sym_POUND] = ACTIONS(15),
    [anon_sym_DOLLAR] = ACTIONS(15),
    [anon_sym_PERCENT] = ACTIONS(15),
    [anon_sym_CARET] = ACTIONS(15),
    [anon_sym_AMP] = ACTIONS(15),
    [anon_sym_COMMA] = ACTIONS(15),
    [anon_sym_DOT] = ACTIONS(15),
    [anon_sym_EQ] = ACTIONS(15),
    [anon_sym__] = ACTIONS(15),
    [anon_sym_TILDE] = ACTIONS(15),
    [anon_sym_PIPE] = ACTIONS(15),
    [anon_sym_PLUS] = ACTIONS(15),
    [anon_sym_SLASH] = ACTIONS(15),
    [anon_sym_STAR] = ACTIONS(15),
  },
  [5] = {
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_RPAREN] = ACTIONS(17),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_BANG] = ACTIONS(17),
    [anon_sym_AT] = ACTIONS(17),
    [anon_sym_POUND] = ACTIONS(17),
    [anon_sym_DOLLAR] = ACTIONS(17),
    [anon_sym_PERCENT] = ACTIONS(17),
    [anon_sym_CARET] = ACTIONS(17),
    [anon_sym_AMP] = ACTIONS(17),
    [anon_sym_COMMA] = ACTIONS(17),
    [anon_sym_DOT] = ACTIONS(17),
    [anon_sym_EQ] = ACTIONS(17),
    [anon_sym__] = ACTIONS(17),
    [anon_sym_TILDE] = ACTIONS(17),
    [anon_sym_PIPE] = ACTIONS(17),
    [anon_sym_PLUS] = ACTIONS(17),
    [anon_sym_SLASH] = ACTIONS(17),
    [anon_sym_STAR] = ACTIONS(17),
  },
  [6] = {
    [ts_builtin_sym_end] = ACTIONS(19),
    [anon_sym_RPAREN] = ACTIONS(19),
    [anon_sym_DASH] = ACTIONS(19),
    [anon_sym_BANG] = ACTIONS(19),
    [anon_sym_AT] = ACTIONS(19),
    [anon_sym_POUND] = ACTIONS(19),
    [anon_sym_DOLLAR] = ACTIONS(19),
    [anon_sym_PERCENT] = ACTIONS(19),
    [anon_sym_CARET] = ACTIONS(19),
    [anon_sym_AMP] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(19),
    [anon_sym_DOT] = ACTIONS(19),
    [anon_sym_EQ] = ACTIONS(19),
    [anon_sym__] = ACTIONS(19),
    [anon_sym_TILDE] = ACTIONS(19),
    [anon_sym_PIPE] = ACTIONS(19),
    [anon_sym_PLUS] = ACTIONS(19),
    [anon_sym_SLASH] = ACTIONS(19),
    [anon_sym_STAR] = ACTIONS(19),
  },
  [7] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_RPAREN] = ACTIONS(21),
    [anon_sym_DASH] = ACTIONS(21),
    [anon_sym_BANG] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(21),
    [anon_sym_DOLLAR] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(21),
    [anon_sym_CARET] = ACTIONS(21),
    [anon_sym_AMP] = ACTIONS(21),
    [anon_sym_COMMA] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(21),
    [anon_sym_EQ] = ACTIONS(21),
    [anon_sym__] = ACTIONS(21),
    [anon_sym_TILDE] = ACTIONS(21),
    [anon_sym_PIPE] = ACTIONS(21),
    [anon_sym_PLUS] = ACTIONS(23),
    [anon_sym_SLASH] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(27),
  },
  [8] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_RPAREN] = ACTIONS(21),
    [anon_sym_DASH] = ACTIONS(29),
    [anon_sym_BANG] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(21),
    [anon_sym_DOLLAR] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(21),
    [anon_sym_CARET] = ACTIONS(21),
    [anon_sym_AMP] = ACTIONS(21),
    [anon_sym_COMMA] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(21),
    [anon_sym_EQ] = ACTIONS(21),
    [anon_sym__] = ACTIONS(21),
    [anon_sym_TILDE] = ACTIONS(21),
    [anon_sym_PIPE] = ACTIONS(21),
    [anon_sym_PLUS] = ACTIONS(23),
    [anon_sym_SLASH] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(27),
  },
  [9] = {
    [ts_builtin_sym_end] = ACTIONS(31),
    [anon_sym_RPAREN] = ACTIONS(31),
    [anon_sym_DASH] = ACTIONS(31),
    [anon_sym_BANG] = ACTIONS(31),
    [anon_sym_AT] = ACTIONS(31),
    [anon_sym_POUND] = ACTIONS(31),
    [anon_sym_DOLLAR] = ACTIONS(31),
    [anon_sym_PERCENT] = ACTIONS(31),
    [anon_sym_CARET] = ACTIONS(31),
    [anon_sym_AMP] = ACTIONS(31),
    [anon_sym_COMMA] = ACTIONS(31),
    [anon_sym_DOT] = ACTIONS(31),
    [anon_sym_EQ] = ACTIONS(31),
    [anon_sym__] = ACTIONS(31),
    [anon_sym_TILDE] = ACTIONS(31),
    [anon_sym_PIPE] = ACTIONS(31),
    [anon_sym_PLUS] = ACTIONS(31),
    [anon_sym_SLASH] = ACTIONS(31),
    [anon_sym_STAR] = ACTIONS(31),
  },
  [10] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_RPAREN] = ACTIONS(21),
    [anon_sym_DASH] = ACTIONS(21),
    [anon_sym_BANG] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(21),
    [anon_sym_DOLLAR] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(21),
    [anon_sym_CARET] = ACTIONS(21),
    [anon_sym_AMP] = ACTIONS(21),
    [anon_sym_COMMA] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(21),
    [anon_sym_EQ] = ACTIONS(21),
    [anon_sym__] = ACTIONS(21),
    [anon_sym_TILDE] = ACTIONS(21),
    [anon_sym_PIPE] = ACTIONS(21),
    [anon_sym_PLUS] = ACTIONS(21),
    [anon_sym_SLASH] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(27),
  },
  [11] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_RPAREN] = ACTIONS(21),
    [anon_sym_DASH] = ACTIONS(21),
    [anon_sym_BANG] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(21),
    [anon_sym_DOLLAR] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(21),
    [anon_sym_CARET] = ACTIONS(21),
    [anon_sym_AMP] = ACTIONS(21),
    [anon_sym_COMMA] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(21),
    [anon_sym_EQ] = ACTIONS(21),
    [anon_sym__] = ACTIONS(21),
    [anon_sym_TILDE] = ACTIONS(21),
    [anon_sym_PIPE] = ACTIONS(21),
    [anon_sym_PLUS] = ACTIONS(21),
    [anon_sym_SLASH] = ACTIONS(21),
    [anon_sym_STAR] = ACTIONS(27),
  },
  [12] = {
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_RPAREN] = ACTIONS(21),
    [anon_sym_DASH] = ACTIONS(21),
    [anon_sym_BANG] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(21),
    [anon_sym_POUND] = ACTIONS(21),
    [anon_sym_DOLLAR] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(21),
    [anon_sym_CARET] = ACTIONS(21),
    [anon_sym_AMP] = ACTIONS(21),
    [anon_sym_COMMA] = ACTIONS(21),
    [anon_sym_DOT] = ACTIONS(21),
    [anon_sym_EQ] = ACTIONS(21),
    [anon_sym__] = ACTIONS(21),
    [anon_sym_TILDE] = ACTIONS(21),
    [anon_sym_PIPE] = ACTIONS(21),
    [anon_sym_PLUS] = ACTIONS(21),
    [anon_sym_SLASH] = ACTIONS(21),
    [anon_sym_STAR] = ACTIONS(21),
  },
  [13] = {
    [ts_builtin_sym_end] = ACTIONS(33),
    [anon_sym_DASH] = ACTIONS(29),
    [anon_sym_BANG] = ACTIONS(35),
    [anon_sym_AT] = ACTIONS(35),
    [anon_sym_POUND] = ACTIONS(35),
    [anon_sym_DOLLAR] = ACTIONS(35),
    [anon_sym_PERCENT] = ACTIONS(35),
    [anon_sym_CARET] = ACTIONS(35),
    [anon_sym_AMP] = ACTIONS(35),
    [anon_sym_COMMA] = ACTIONS(35),
    [anon_sym_DOT] = ACTIONS(35),
    [anon_sym_EQ] = ACTIONS(35),
    [anon_sym__] = ACTIONS(35),
    [anon_sym_TILDE] = ACTIONS(35),
    [anon_sym_PIPE] = ACTIONS(35),
    [anon_sym_PLUS] = ACTIONS(23),
    [anon_sym_SLASH] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(27),
  },
  [14] = {
    [anon_sym_RPAREN] = ACTIONS(37),
    [anon_sym_DASH] = ACTIONS(29),
    [anon_sym_BANG] = ACTIONS(35),
    [anon_sym_AT] = ACTIONS(35),
    [anon_sym_POUND] = ACTIONS(35),
    [anon_sym_DOLLAR] = ACTIONS(35),
    [anon_sym_PERCENT] = ACTIONS(35),
    [anon_sym_CARET] = ACTIONS(35),
    [anon_sym_AMP] = ACTIONS(35),
    [anon_sym_COMMA] = ACTIONS(35),
    [anon_sym_DOT] = ACTIONS(35),
    [anon_sym_EQ] = ACTIONS(35),
    [anon_sym__] = ACTIONS(35),
    [anon_sym_TILDE] = ACTIONS(35),
    [anon_sym_PIPE] = ACTIONS(35),
    [anon_sym_PLUS] = ACTIONS(23),
    [anon_sym_SLASH] = ACTIONS(25),
    [anon_sym_STAR] = ACTIONS(27),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(14), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [22] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(10), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [44] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(8), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [66] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(12), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [88] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(7), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [110] = 6,
    ACTIONS(3), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 1,
      aux_sym_number_token1,
    ACTIONS(7), 1,
      anon_sym_DASH,
    ACTIONS(9), 1,
      aux_sym_float_token1,
    STATE(11), 1,
      sym_expression,
    STATE(4), 4,
      sym_parenthesized_expression,
      sym_number,
      sym_float,
      sym_binary_expression,
  [132] = 2,
    ACTIONS(39), 1,
      aux_sym_number_token1,
    ACTIONS(41), 1,
      aux_sym_float_token1,
  [139] = 1,
    ACTIONS(43), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(15)] = 0,
  [SMALL_STATE(16)] = 22,
  [SMALL_STATE(17)] = 44,
  [SMALL_STATE(18)] = 66,
  [SMALL_STATE(19)] = 88,
  [SMALL_STATE(20)] = 110,
  [SMALL_STATE(21)] = 132,
  [SMALL_STATE(22)] = 139,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float, 1, 0, 0),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1, 0, 0),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 2, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float, 2, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, 0, 2),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_expression, 3, 0, 1),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [43] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_calculator(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
