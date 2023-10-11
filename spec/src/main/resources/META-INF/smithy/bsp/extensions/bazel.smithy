$version: "2"

namespace bsp.bazel

use bsp#BuildTargetIdentifier
use bsp#BuildTargetIdentifiers
use bsp#URI
use traits#jsonRPC
use traits#jsonRequest

@jsonRPC
service BazelBuildServer {
    operations: [
        WorkspaceLibraries
        WorkspaceDirectories
    ]
}

@unstable
@jsonRequest("workspace/libraries")
operation WorkspaceLibraries {
    output: WorkspaceLibrariesResult
}

@unstable
@jsonRequest("workspace/directories")
operation WorkspaceDirectories {
    output: WorkspaceDirectoriesResult
}

@unstable
structure WorkspaceLibrariesResult {
    @required
    libraries: LibraryItems
}

@unstable
structure WorkspaceDirectoriesResult {
    @required
    includedDirectories: DirectoryItems
    @required
    excludedDirectories: DirectoryItems
}

@unstable
structure LibraryItem {
    @required
    id: BuildTargetIdentifier
    @required
    dependencies: BuildTargetIdentifiers
    @required
    jars: Jars
    @required
    sourceJars: Jars

}

list LibraryItems {
    member: LibraryItem
}

@unstable
string Jar

list Jars {
    member: Jar
}

@unstable
structure DirectoryItem {
    @required
    uri: URI
}

list DirectoryItems {
    member: DirectoryItem
}
