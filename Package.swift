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
            url: "https://github.com/xbjfk/twsearch/releases/download/0.0.10/TWSearch.xcframework.tar.gz",
            checksum: "e1ae967edf730d9559c09733e84aaaefc19fd80c18c315df27d882c578d10d2ec88638b9247a348622eadd58737df132c6ac4f53fd584b27c0ac038eaa622301"
        )
    ]
)
