package org.jetbrains.bsp.tests;

import ch.epfl.scala.bsp._
import com.github.plokhotnyuk.jsoniter_scala.core.{JsonValueCodec, readFromString, writeToString}
import com.github.plokhotnyuk.jsoniter_scala.macros.JsonCodecMaker

import scala.io.StdIn

object ScalaJsonRoundtrip {
  private val TERMINAL_STRING = "exit"

  // TODO Kasia: move it to some file and make it generate automatically
  private val serializersMap: Map[String, JsonValueCodec[_]] = Map(
    "Range" -> Range.codec,
    "CompileResult" -> CompileResult.codec,
    "WorkspaceBuildTargetsResult" -> WorkspaceBuildTargetsResult.codec,
    "TestFinish" -> TestFinish.codec,
    "DependencySourcesResult" -> DependencySourcesResult.codec,
    "Position" -> Position.codec,
    "BspConnectionDetails" -> BspConnectionDetails.codec,
    "TestProvider" -> TestProvider.codec,
    "RunParams" -> RunParams.codec,
    "InverseSourcesParams" -> InverseSourcesParams.codec,
    "TaskProgressParams" -> TaskProgressParams.codec,
    "PrintParams" -> PrintParams.codec,
    "TaskStartParams" -> TaskStartParams.codec,
    "OutputPathsResult" -> OutputPathsResult.codec,
    "CompileReport" -> CompileReport.codec,
    "BuildTargetIdentifier" -> BuildTargetIdentifier.codec,
    "ResourcesItem" -> ResourcesItem.codec,
    "OutputPathsItem" -> OutputPathsItem.codec,
    "RunResult" -> RunResult.codec,
    "ResourcesResult" -> ResourcesResult.codec,
    "CompileTask" -> CompileTask.codec,
    "CleanCacheResult" -> CleanCacheResult.codec,
    "TestReport" -> TestReport.codec,
    "TaskId" -> TaskId.codec,
    "BuildClientCapabilities" -> BuildClientCapabilities.codec,
    "InverseSourcesResult" -> InverseSourcesResult.codec,
    "SourcesResult" -> SourcesResult.codec,
    "BuildTargetCapabilities" -> BuildTargetCapabilities.codec,
    "OutputPathsParams" -> OutputPathsParams.codec,
    "DependencyModulesItem" -> DependencyModulesItem.codec,
    "DebugProvider" -> DebugProvider.codec,
    "CleanCacheParams" -> CleanCacheParams.codec,
    "SourcesParams" -> SourcesParams.codec,
    "TestTask" -> TestTask.codec,
    "SourceItem" -> SourceItem.codec,
    "BuildTargetEvent" -> BuildTargetEvent.codec,
    "ResourcesParams" -> ResourcesParams.codec,
    "DependencyModulesResult" -> DependencyModulesResult.codec,
    "DiagnosticRelatedInformation" -> DiagnosticRelatedInformation.codec,
    "CodeDescription" -> CodeDescription.codec,
    "SourcesItem" -> SourcesItem.codec,
    "TextDocumentIdentifier" -> TextDocumentIdentifier.codec,
    "BuildTarget" -> BuildTarget.codec,
    "InitializeBuildParams" -> InitializeBuildParams.codec,
    "RunProvider" -> RunProvider.codec,
    "DependencyModule" -> DependencyModule.codec,
    "DidChangeBuildTarget" -> DidChangeBuildTarget.codec,
    "BuildServerCapabilities" -> BuildServerCapabilities.codec,
    "TaskFinishParams" -> TaskFinishParams.codec,
    "LogMessageParams" -> LogMessageParams.codec,
    "TestResult" -> TestResult.codec,
    "OutputPathItem" -> OutputPathItem.codec,
    "DebugSessionParams" -> DebugSessionParams.codec,
    "DependencySourcesParams" -> DependencySourcesParams.codec,
    "CompileProvider" -> CompileProvider.codec,
    "CompileParams" -> CompileParams.codec,
    "InitializeBuildResult" -> InitializeBuildResult.codec,
    "DependencyModulesParams" -> DependencyModulesParams.codec,
    "ReadParams" -> ReadParams.codec,
    "Location" -> Location.codec,
    "DependencySourcesItem" -> DependencySourcesItem.codec,
    "TestStart" -> TestStart.codec,
    "PublishDiagnosticsParams" -> PublishDiagnosticsParams.codec,
    "TestParams" -> TestParams.codec,
    "DebugSessionAddress" -> DebugSessionAddress.codec,
    "Diagnostic" -> Diagnostic.codec,
    "ShowMessageParams" -> ShowMessageParams.codec,
    "CancelRequestParams" -> CancelRequestParams.codec,
    "SetCargoFeaturesResult" -> SetCargoFeaturesResult.codec,
    "SetCargoFeaturesParams" -> SetCargoFeaturesParams.codec,
    "PackageFeatures" -> PackageFeatures.codec,
    "CargoBuildTarget" -> CargoBuildTarget.codec,
    "CargoFeaturesStateResult" -> CargoFeaturesStateResult.codec,
    "CppOptionsResult" -> CppOptionsResult.codec,
    "CppOptionsParams" -> CppOptionsParams.codec,
    "CppOptionsItem" -> CppOptionsItem.codec,
    "CppBuildTarget" -> CppBuildTarget.codec,
    "JavacOptionsResult" -> JavacOptionsResult.codec,
    "JavacOptionsItem" -> JavacOptionsItem.codec,
    "JavacOptionsParams" -> JavacOptionsParams.codec,
    "JvmTestEnvironmentResult" -> JvmTestEnvironmentResult.codec,
    "JvmBuildTarget" -> JvmBuildTarget.codec,
    "JvmTestEnvironmentParams" -> JvmTestEnvironmentParams.codec,
    "JvmRunEnvironmentResult" -> JvmRunEnvironmentResult.codec,
    "JvmMainClass" -> JvmMainClass.codec,
    "JvmRunEnvironmentParams" -> JvmRunEnvironmentParams.codec,
    "JvmEnvironmentItem" -> JvmEnvironmentItem.codec,
    "MavenDependencyModuleArtifact" -> MavenDependencyModuleArtifact.codec,
    "MavenDependencyModule" -> MavenDependencyModule.codec,
    "PythonBuildTarget" -> PythonBuildTarget.codec,
    "PythonOptionsResult" -> PythonOptionsResult.codec,
    "PythonOptionsItem" -> PythonOptionsItem.codec,
    "PythonOptionsParams" -> PythonOptionsParams.codec,
    "RustPackage" -> RustPackage.codec,
    "RustWorkspaceParams" -> RustWorkspaceParams.codec,
    "RustDepKindInfo" -> RustDepKindInfo.codec,
    "RustTarget" -> RustTarget.codec,
    "RustRawDependency" -> RustRawDependency.codec,
    "RustDependency" -> RustDependency.codec,
    "RustWorkspaceResult" -> RustWorkspaceResult.codec,
    "SbtBuildTarget" -> SbtBuildTarget.codec,
    "ScalaTestSuiteSelection" -> ScalaTestSuiteSelection.codec,
    "ScalaMainClassesParams" -> ScalaMainClassesParams.codec,
    "ScalaTextEdit" -> ScalaTextEdit.codec,
    "ScalaBuildTarget" -> ScalaBuildTarget.codec,
    "ScalacOptionsResult" -> ScalacOptionsResult.codec,
    "ScalaMainClass" -> ScalaMainClass.codec,
    "ScalaMainClassesResult" -> ScalaMainClassesResult.codec,
    "ScalacOptionsItem" -> ScalacOptionsItem.codec,
    "ScalaWorkspaceEdit" -> ScalaWorkspaceEdit.codec,
    "ScalacOptionsParams" -> ScalacOptionsParams.codec,
    "ScalaTestClassesResult" -> ScalaTestClassesResult.codec,
    "ScalaMainClassesItem" -> ScalaMainClassesItem.codec,
    "ScalaTestParams" -> ScalaTestParams.codec,
    "ScalaTestSuites" -> ScalaTestSuites.codec,
    "ScalaTestClassesParams" -> ScalaTestClassesParams.codec,
    "ScalaAction" -> ScalaAction.codec,
    "ScalaTestClassesItem" -> ScalaTestClassesItem.codec,
    "ScalaAttachRemote" -> ScalaAttachRemote.codec,
    "ScalaDiagnostic" -> ScalaDiagnostic.codec
  )

  case class DataWrapper(name: String, jsonData: String)

  def main(args: Array[String]): Unit = {
    while (true) {
      try {
        val inputLine = StdIn.readLine()
        if (inputLine == null || inputLine == TERMINAL_STRING) {
          sys.exit(0)
        }

        val wrapper: DataWrapper = readFromString(inputLine)(JsonCodecMaker.make[DataWrapper])

        val codecOption: Option[JsonValueCodec[_]] = serializersMap.get(wrapper.name)

        codecOption match {
          case Some(codec) =>
            val jsonData = wrapper.jsonData
            val instance = readFromString(jsonData)(codec.asInstanceOf[JsonValueCodec[Any]])
            val encodedInstance = writeToString(instance)(codec.asInstanceOf[JsonValueCodec[Any]])
            println(encodedInstance)

          case None =>
            println(s"Type '${wrapper.name}' not known")
        }

      } catch {
        case e: Exception =>
          println(s"Error processing input: ${e.getMessage}")
      }
    }
  }
}
