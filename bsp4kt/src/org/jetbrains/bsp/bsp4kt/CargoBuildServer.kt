package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName

interface CargoBuildServer {
  @JsonRequest("workspace/cargoFeaturesState")
  suspend fun cargoFeaturesState(): CargoFeaturesStateResult

  @JsonRequest("workspace/enableCargoFeatures")
  suspend fun enableCargoFeatures(params: EnableCargoFeaturesParams): Unit

  @JsonRequest("workspace/disableCargoFeatures")
  suspend fun disableCargoFeatures(params: DisableCargoFeaturesParams): Unit

}
