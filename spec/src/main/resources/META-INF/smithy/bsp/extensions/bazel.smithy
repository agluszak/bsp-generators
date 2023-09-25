$version: "2"

namespace bsp.bazel

use bsp#BuildTargetIdentifier
use bsp#BuildTargetIdentifiers
use traits#jsonRPC
use traits#jsonRequest

@jsonRPC
service BazelBuildServer {
    operations: [
        WorkspaceLibraries
    ]
}

@unstable
@jsonRequest("workspace/libraries")
operation WorkspaceLibraries {
    output: WorkspaceLibrariesResult
}

@unstable
structure WorkspaceLibrariesResult {
    @required
    libraries: LibraryItems
}

list LibraryItems {
    member: LibraryItem
}

@unstable
structure LibraryItem {
    @required
    id: BuildTargetIdentifier
    @required
    dependencies: BuildTargetIdentifiers
    @required
    jars: Jars
}

@unstable
string Jar

list Jars {
    member: Jar
}
