package ch.epfl.scala.bsp4j;

import java.util.concurrent.CompletableFuture;
import org.eclipse.lsp4j.jsonrpc.services.JsonRequest;

public interface RustBuildServer {
  @JsonRequest("buildTarget/rustWorkspace")
  CompletableFuture<RustWorkspaceResult> rustWorkspace(RustWorkspaceParams params);

  @JsonRequest("buildTarget/rustToolchain")
  CompletableFuture<RustToolchainResult> rustToolchain(RustToolchainParams params);
}
