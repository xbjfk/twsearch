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
            path: "./src/rs-swift/Swift"
	),
        .binaryTarget(
            name: "TWSearchLib",
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.60/TWSearch.xcframework.zip",
            checksum: "6d977471de1bced18b4024ca3f33fcbc0ce31e6799867348d9afcb7b05b70ab3"
        )
    ]
)
