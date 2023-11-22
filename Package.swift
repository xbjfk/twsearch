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
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.50/TWSearch.xcframework.tar.gz",
            checksum: "baa8f085918e7aa730590fc97d1c469f004e500d8a35d546fa50a7d928d27a49"
        )
    ]
)
