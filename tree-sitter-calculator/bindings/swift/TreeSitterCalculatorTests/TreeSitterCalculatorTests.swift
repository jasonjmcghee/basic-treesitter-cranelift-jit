import XCTest
import SwiftTreeSitter
import TreeSitterCalculator

final class TreeSitterCalculatorTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_calculator())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Calculator grammar")
    }
}
