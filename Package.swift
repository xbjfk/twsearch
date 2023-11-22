// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TWSearch",
    products: [
        .library(name: "TWSearch", targets: ["TWSearchLib", "TWSearchLibSwift"]),
    ],
    targets: [
        .target(
            name: "TWSearchLibSwift",
            sources: ["./src/rs-swift/Swift"]
	),
        .binaryTarget(
            name: "TWSearchLib",
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.9/TWSearch.xcframework.tar.gz",
            checksum: "3a84fc40159bb95683335dfc53358d1be41769033f6847d17e74895121856a83fd40d0e19574dc18d4b738f1745c2c755c211d93f3d5858cd1f5024e4baa4591  TWSearch.xcframework.zip"
        )
    ]
)
