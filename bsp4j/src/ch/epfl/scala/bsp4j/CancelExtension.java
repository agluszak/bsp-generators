package ch.epfl.scala.bsp4j;

import org.eclipse.lsp4j.jsonrpc.services.JsonNotification;

public interface CancelExtension {
  @JsonNotification("$/cancelRequest")
  void cancelRequest(CancelRequestParams params);
}
