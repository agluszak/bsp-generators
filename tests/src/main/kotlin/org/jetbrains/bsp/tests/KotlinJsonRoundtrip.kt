package org.jetbrains.bsp.tests

import kotlinx.serialization.InternalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.serializer
import org.jetbrains.bsp.bsp4kt.*

// TODO Kasia: move it to some file and make it generate automatically
@OptIn(InternalSerializationApi::class)
val serializersMap = mapOf<String, KSerializer<*>>(
    "Range" to Range::class.serializer(),
    "CompileResult" to CompileResult::class.serializer(),
    "WorkspaceBuildTargetsResult" to WorkspaceBuildTargetsResult::class.serializer(),
    "TestFinish" to TestFinish::class.serializer(),
    "DependencySourcesResult" to DependencySourcesResult::class.serializer(),
    "Position" to Position::class.serializer(),
    "BspConnectionDetails" to BspConnectionDetails::class.serializer(),
    "TestProvider" to TestProvider::class.serializer(),
    "RunParams" to RunParams::class.serializer(),
    "InverseSourcesParams" to InverseSourcesParams::class.serializer(),
    "TaskProgressParams" to TaskProgressParams::class.serializer(),
    "PrintParams" to PrintParams::class.serializer(),
    "TaskStartParams" to TaskStartParams::class.serializer(),
    "OutputPathsResult" to OutputPathsResult::class.serializer(),
    "CompileReport" to CompileReport::class.serializer(),
    "BuildTargetIdentifier" to BuildTargetIdentifier::class.serializer(),
    "ResourcesItem" to ResourcesItem::class.serializer(),
    "OutputPathsItem" to OutputPathsItem::class.serializer(),
    "RunResult" to RunResult::class.serializer(),
    "ResourcesResult" to ResourcesResult::class.serializer(),
    "CompileTask" to CompileTask::class.serializer(),
    "CleanCacheResult" to CleanCacheResult::class.serializer(),
    "TestReport" to TestReport::class.serializer(),
    "TaskId" to TaskId::class.serializer(),
    "BuildClientCapabilities" to BuildClientCapabilities::class.serializer(),
    "InverseSourcesResult" to InverseSourcesResult::class.serializer(),
    "SourcesResult" to SourcesResult::class.serializer(),
    "BuildTargetCapabilities" to BuildTargetCapabilities::class.serializer(),
    "OutputPathsParams" to OutputPathsParams::class.serializer(),
    "DependencyModulesItem" to DependencyModulesItem::class.serializer(),
    "DebugProvider" to DebugProvider::class.serializer(),
    "CleanCacheParams" to CleanCacheParams::class.serializer(),
    "SourcesParams" to SourcesParams::class.serializer(),
    "TestTask" to TestTask::class.serializer(),
    "SourceItem" to SourceItem::class.serializer(),
    "BuildTargetEvent" to BuildTargetEvent::class.serializer(),
    "ResourcesParams" to ResourcesParams::class.serializer(),
    "DependencyModulesResult" to DependencyModulesResult::class.serializer(),
    "DiagnosticRelatedInformation" to DiagnosticRelatedInformation::class.serializer(),
    "CodeDescription" to CodeDescription::class.serializer(),
    "SourcesItem" to SourcesItem::class.serializer(),
    "TextDocumentIdentifier" to TextDocumentIdentifier::class.serializer(),
    "BuildTarget" to BuildTarget::class.serializer(),
    "InitializeBuildParams" to InitializeBuildParams::class.serializer(),
    "RunProvider" to RunProvider::class.serializer(),
    "DependencyModule" to DependencyModule::class.serializer(),
    "DidChangeBuildTarget" to DidChangeBuildTarget::class.serializer(),
    "BuildServerCapabilities" to BuildServerCapabilities::class.serializer(),
    "TaskFinishParams" to TaskFinishParams::class.serializer(),
    "LogMessageParams" to LogMessageParams::class.serializer(),
    "TestResult" to TestResult::class.serializer(),
    "OutputPathItem" to OutputPathItem::class.serializer(),
    "DebugSessionParams" to DebugSessionParams::class.serializer(),
    "DependencySourcesParams" to DependencySourcesParams::class.serializer(),
    "CompileProvider" to CompileProvider::class.serializer(),
    "CompileParams" to CompileParams::class.serializer(),
    "InitializeBuildResult" to InitializeBuildResult::class.serializer(),
    "DependencyModulesParams" to DependencyModulesParams::class.serializer(),
    "ReadParams" to ReadParams::class.serializer(),
    "Location" to Location::class.serializer(),
    "DependencySourcesItem" to DependencySourcesItem::class.serializer(),
    "TestStart" to TestStart::class.serializer(),
    "PublishDiagnosticsParams" to PublishDiagnosticsParams::class.serializer(),
    "TestParams" to TestParams::class.serializer(),
    "DebugSessionAddress" to DebugSessionAddress::class.serializer(),
    "Diagnostic" to Diagnostic::class.serializer(),
    "ShowMessageParams" to ShowMessageParams::class.serializer(),
    "CancelRequestParams" to CancelRequestParams::class.serializer(),
    "SetCargoFeaturesResult" to SetCargoFeaturesResult::class.serializer(),
    "SetCargoFeaturesParams" to SetCargoFeaturesParams::class.serializer(),
    "PackageFeatures" to PackageFeatures::class.serializer(),
    "CargoBuildTarget" to CargoBuildTarget::class.serializer(),
    "CargoFeaturesStateResult" to CargoFeaturesStateResult::class.serializer(),
    "CppOptionsResult" to CppOptionsResult::class.serializer(),
    "CppOptionsParams" to CppOptionsParams::class.serializer(),
    "CppOptionsItem" to CppOptionsItem::class.serializer(),
    "CppBuildTarget" to CppBuildTarget::class.serializer(),
    "JavacOptionsResult" to JavacOptionsResult::class.serializer(),
    "JavacOptionsItem" to JavacOptionsItem::class.serializer(),
    "JavacOptionsParams" to JavacOptionsParams::class.serializer(),
    "JvmTestEnvironmentResult" to JvmTestEnvironmentResult::class.serializer(),
    "JvmBuildTarget" to JvmBuildTarget::class.serializer(),
    "JvmTestEnvironmentParams" to JvmTestEnvironmentParams::class.serializer(),
    "JvmRunEnvironmentResult" to JvmRunEnvironmentResult::class.serializer(),
    "JvmMainClass" to JvmMainClass::class.serializer(),
    "JvmRunEnvironmentParams" to JvmRunEnvironmentParams::class.serializer(),
    "JvmEnvironmentItem" to JvmEnvironmentItem::class.serializer(),
    "MavenDependencyModuleArtifact" to MavenDependencyModuleArtifact::class.serializer(),
    "MavenDependencyModule" to MavenDependencyModule::class.serializer(),
    "PythonBuildTarget" to PythonBuildTarget::class.serializer(),
    "PythonOptionsResult" to PythonOptionsResult::class.serializer(),
    "PythonOptionsItem" to PythonOptionsItem::class.serializer(),
    "PythonOptionsParams" to PythonOptionsParams::class.serializer(),
    "RustPackage" to RustPackage::class.serializer(),
    "RustWorkspaceParams" to RustWorkspaceParams::class.serializer(),
    "RustDepKindInfo" to RustDepKindInfo::class.serializer(),
    "RustTarget" to RustTarget::class.serializer(),
    "RustRawDependency" to RustRawDependency::class.serializer(),
    "RustDependency" to RustDependency::class.serializer(),
    "RustWorkspaceResult" to RustWorkspaceResult::class.serializer(),
    "SbtBuildTarget" to SbtBuildTarget::class.serializer(),
    "ScalaTestSuiteSelection" to ScalaTestSuiteSelection::class.serializer(),
    "ScalaMainClassesParams" to ScalaMainClassesParams::class.serializer(),
    "ScalaTextEdit" to ScalaTextEdit::class.serializer(),
    "ScalaBuildTarget" to ScalaBuildTarget::class.serializer(),
    "ScalacOptionsResult" to ScalacOptionsResult::class.serializer(),
    "ScalaMainClass" to ScalaMainClass::class.serializer(),
    "ScalaMainClassesResult" to ScalaMainClassesResult::class.serializer(),
    "ScalacOptionsItem" to ScalacOptionsItem::class.serializer(),
    "ScalaWorkspaceEdit" to ScalaWorkspaceEdit::class.serializer(),
    "ScalacOptionsParams" to ScalacOptionsParams::class.serializer(),
    "ScalaTestClassesResult" to ScalaTestClassesResult::class.serializer(),
    "ScalaMainClassesItem" to ScalaMainClassesItem::class.serializer(),
    "ScalaTestParams" to ScalaTestParams::class.serializer(),
    "ScalaTestSuites" to ScalaTestSuites::class.serializer(),
    "ScalaTestClassesParams" to ScalaTestClassesParams::class.serializer(),
    "ScalaAction" to ScalaAction::class.serializer(),
    "ScalaTestClassesItem" to ScalaTestClassesItem::class.serializer(),
    "ScalaAttachRemote" to ScalaAttachRemote::class.serializer(),
    "ScalaDiagnostic" to ScalaDiagnostic::class.serializer(),
)

const val TERMINAL_STRING = "exit"

@Serializable
data class DataWrapper(val name: String, val jsonData: String)

fun main() {
    val json = Json { ignoreUnknownKeys = true }

    while (true) {
        try {
            val inputLine = readlnOrNull() ?: break
            if (inputLine == TERMINAL_STRING) break

            val wrapper = json.decodeFromString<DataWrapper>(inputLine)

            val kClassSerializer = serializersMap[wrapper.name]
            if (kClassSerializer == null) {
                println("Type '" + wrapper.name + "' not known")
                continue
            }

            val instance = Json.decodeFromString(kClassSerializer, wrapper.jsonData)

            val encodedInstance = Json.encodeToJsonElement(kClassSerializer as KSerializer<Any>, instance!!)
            println(encodedInstance.toString())
        } catch (e: Exception) {
            println("Error processing input: ${e.message}")
        }
    }
}
