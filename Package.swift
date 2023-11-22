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
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.99/TWSearch.xcframework.zip",
            checksum: "9628b88bc2dcd2fbfb638986d5e8527878c314f494dcd6cc6057de83fe72f9ea"
        )
    ]
)
