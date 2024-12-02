#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 21
#define LARGE_STATE_COUNT 7
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
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
  anon_sym_PLUS = 6,
  anon_sym_SLASH = 7,
  anon_sym_STAR = 8,
  sym_source = 9,
  sym_expression = 10,
  sym_parenthesized_expression = 11,
  sym_number = 12,
  sym_float = 13,
  sym_binary_expression = 14,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [aux_sym_number_token1] = "number_token1",
  [anon_sym_DASH] = "-",
  [aux_sym_float_token1] = "float_token1",
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(2);
      if (lookahead == '(') ADVANCE(3);
      if (lookahead == ')') ADVANCE(4);
      if (lookahead == '*') ADVANCE(10);
      if (lookahead == '+') ADVANCE(8);
      if (lookahead == '-') ADVANCE(6);
      if (lookahead == '.') ADVANCE(1);
      if (lookahead == '/') ADVANCE(9);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 1:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(aux_sym_float_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
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
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [aux_sym_float_token1] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(20),
    [sym_expression] = STATE(17),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [2] = {
    [sym_expression] = STATE(18),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [3] = {
    [sym_expression] = STATE(14),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [4] = {
    [sym_expression] = STATE(15),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [5] = {
    [sym_expression] = STATE(13),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
  [6] = {
    [sym_expression] = STATE(16),
    [sym_parenthesized_expression] = STATE(8),
    [sym_number] = STATE(8),
    [sym_float] = STATE(8),
    [sym_binary_expression] = STATE(8),
    [anon_sym_LPAREN] = ACTIONS(3),
    [aux_sym_number_token1] = ACTIONS(5),
    [anon_sym_DASH] = ACTIONS(7),
    [aux_sym_float_token1] = ACTIONS(9),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(11), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [9] = 1,
    ACTIONS(13), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [18] = 1,
    ACTIONS(15), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [27] = 1,
    ACTIONS(17), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [36] = 1,
    ACTIONS(19), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [45] = 1,
    ACTIONS(21), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [54] = 4,
    ACTIONS(25), 1,
      anon_sym_PLUS,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    ACTIONS(29), 1,
      anon_sym_STAR,
    ACTIONS(23), 3,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
  [69] = 3,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    ACTIONS(29), 1,
      anon_sym_STAR,
    ACTIONS(23), 4,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
  [82] = 2,
    ACTIONS(29), 1,
      anon_sym_STAR,
    ACTIONS(23), 5,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
  [93] = 1,
    ACTIONS(23), 6,
      ts_builtin_sym_end,
      anon_sym_RPAREN,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_SLASH,
      anon_sym_STAR,
  [102] = 5,
    ACTIONS(25), 1,
      anon_sym_PLUS,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    ACTIONS(29), 1,
      anon_sym_STAR,
    ACTIONS(31), 1,
      ts_builtin_sym_end,
    ACTIONS(33), 1,
      anon_sym_DASH,
  [118] = 5,
    ACTIONS(25), 1,
      anon_sym_PLUS,
    ACTIONS(27), 1,
      anon_sym_SLASH,
    ACTIONS(29), 1,
      anon_sym_STAR,
    ACTIONS(33), 1,
      anon_sym_DASH,
    ACTIONS(35), 1,
      anon_sym_RPAREN,
  [134] = 2,
    ACTIONS(37), 1,
      aux_sym_number_token1,
    ACTIONS(39), 1,
      aux_sym_float_token1,
  [141] = 1,
    ACTIONS(41), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(7)] = 0,
  [SMALL_STATE(8)] = 9,
  [SMALL_STATE(9)] = 18,
  [SMALL_STATE(10)] = 27,
  [SMALL_STATE(11)] = 36,
  [SMALL_STATE(12)] = 45,
  [SMALL_STATE(13)] = 54,
  [SMALL_STATE(14)] = 69,
  [SMALL_STATE(15)] = 82,
  [SMALL_STATE(16)] = 93,
  [SMALL_STATE(17)] = 102,
  [SMALL_STATE(18)] = 118,
  [SMALL_STATE(19)] = 134,
  [SMALL_STATE(20)] = 141,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1, 0, 0),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1, 0, 0),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float, 2, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 2, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized_expression, 3, 0, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3, 0, 2),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [41] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
