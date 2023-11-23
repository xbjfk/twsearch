// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TWSearch",
    products: [
        .library(name: "TWSearch", targets: ["TWSearchLibSwift"]),
    ],
    targets: [
        .target(
            name: "TWSearchLibSwift",
            dependencies: ["TWSearchLib"],
            path: "./src/rs-swift/SwiftGenerated"
	),
        .binaryTarget(
            name: "TWSearchLib",
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.998/TWSearch.xcframework.zip",
            checksum: "f81915ac353b0c31e485af0eba84f369d20b6c669590f3544c11c1c609f0004d"
        )
    ]
)
