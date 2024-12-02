// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterCalculator",
    products: [
        .library(name: "TreeSitterCalculator", targets: ["TreeSitterCalculator"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterCalculator",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                // NOTE: if your language has an external scanner, add it here.
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterCalculatorTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterCalculator",
            ],
            path: "bindings/swift/TreeSitterCalculatorTests"
        )
    ],
    cLanguageStandard: .c11
)
