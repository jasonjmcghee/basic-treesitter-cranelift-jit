package tree_sitter_calculator_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_calculator "github.com/tree-sitter/tree-sitter-calculator/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_calculator.Language())
	if language == nil {
		t.Errorf("Error loading Calculator grammar")
	}
}
