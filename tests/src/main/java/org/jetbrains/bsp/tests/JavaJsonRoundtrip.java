package org.jetbrains.bsp.tests;

import ch.epfl.scala.bsp4j.*;
import com.google.gson.*;
import org.eclipse.lsp4j.jsonrpc.json.adapters.EitherTypeAdapterFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;

public class JavaJsonRoundtrip {

    private static final Map<String, Class<?>> serializersMap = new HashMap<>();
    private static final String TERMINAL_STRING = "exit";

    static {
        serializersMap.put("Range", Range.class);
        serializersMap.put("CompileResult", CompileResult.class);
        serializersMap.put("WorkspaceBuildTargetsResult", WorkspaceBuildTargetsResult.class);
        serializersMap.put("TestFinish", TestFinish.class);
        serializersMap.put("DependencySourcesResult", DependencySourcesResult.class);
        serializersMap.put("Position", Position.class);
        serializersMap.put("BspConnectionDetails", BspConnectionDetails.class);
        serializersMap.put("TestProvider", TestProvider.class);
        serializersMap.put("RunParams", RunParams.class);
        serializersMap.put("InverseSourcesParams", InverseSourcesParams.class);
        serializersMap.put("TaskProgressParams", TaskProgressParams.class);
        serializersMap.put("PrintParams", PrintParams.class);
        serializersMap.put("TaskStartParams", TaskStartParams.class);
        serializersMap.put("OutputPathsResult", OutputPathsResult.class);
        serializersMap.put("CompileReport", CompileReport.class);
        serializersMap.put("BuildTargetIdentifier", BuildTargetIdentifier.class);
        serializersMap.put("ResourcesItem", ResourcesItem.class);
        serializersMap.put("OutputPathsItem", OutputPathsItem.class);
        serializersMap.put("RunResult", RunResult.class);
        serializersMap.put("ResourcesResult", ResourcesResult.class);
        serializersMap.put("CompileTask", CompileTask.class);
        serializersMap.put("CleanCacheResult", CleanCacheResult.class);
        serializersMap.put("TestReport", TestReport.class);
        serializersMap.put("TaskId", TaskId.class);
        serializersMap.put("BuildClientCapabilities", BuildClientCapabilities.class);
        serializersMap.put("InverseSourcesResult", InverseSourcesResult.class);
        serializersMap.put("SourcesResult", SourcesResult.class);
        serializersMap.put("BuildTargetCapabilities", BuildTargetCapabilities.class);
        serializersMap.put("OutputPathsParams", OutputPathsParams.class);
        serializersMap.put("DependencyModulesItem", DependencyModulesItem.class);
        serializersMap.put("DebugProvider", DebugProvider.class);
        serializersMap.put("CleanCacheParams", CleanCacheParams.class);
        serializersMap.put("SourcesParams", SourcesParams.class);
        serializersMap.put("TestTask", TestTask.class);
        serializersMap.put("SourceItem", SourceItem.class);
        serializersMap.put("BuildTargetEvent", BuildTargetEvent.class);
        serializersMap.put("ResourcesParams", ResourcesParams.class);
        serializersMap.put("DependencyModulesResult", DependencyModulesResult.class);
        serializersMap.put("DiagnosticRelatedInformation", DiagnosticRelatedInformation.class);
        serializersMap.put("CodeDescription", CodeDescription.class);
        serializersMap.put("SourcesItem", SourcesItem.class);
        serializersMap.put("TextDocumentIdentifier", TextDocumentIdentifier.class);
        serializersMap.put("BuildTarget", BuildTarget.class);
        serializersMap.put("InitializeBuildParams", InitializeBuildParams.class);
        serializersMap.put("RunProvider", RunProvider.class);
        serializersMap.put("DependencyModule", DependencyModule.class);
        serializersMap.put("DidChangeBuildTarget", DidChangeBuildTarget.class);
        serializersMap.put("BuildServerCapabilities", BuildServerCapabilities.class);
        serializersMap.put("TaskFinishParams", TaskFinishParams.class);
        serializersMap.put("LogMessageParams", LogMessageParams.class);
        serializersMap.put("TestResult", TestResult.class);
        serializersMap.put("OutputPathItem", OutputPathItem.class);
        serializersMap.put("DebugSessionParams", DebugSessionParams.class);
        serializersMap.put("DependencySourcesParams", DependencySourcesParams.class);
        serializersMap.put("CompileProvider", CompileProvider.class);
        serializersMap.put("CompileParams", CompileParams.class);
        serializersMap.put("InitializeBuildResult", InitializeBuildResult.class);
        serializersMap.put("DependencyModulesParams", DependencyModulesParams.class);
        serializersMap.put("ReadParams", ReadParams.class);
        serializersMap.put("Location", Location.class);
        serializersMap.put("DependencySourcesItem", DependencySourcesItem.class);
        serializersMap.put("TestStart", TestStart.class);
        serializersMap.put("PublishDiagnosticsParams", PublishDiagnosticsParams.class);
        serializersMap.put("TestParams", TestParams.class);
        serializersMap.put("DebugSessionAddress", DebugSessionAddress.class);
        serializersMap.put("Diagnostic", Diagnostic.class);
        serializersMap.put("ShowMessageParams", ShowMessageParams.class);
        serializersMap.put("CancelRequestParams", CancelRequestParams.class);
        serializersMap.put("SetCargoFeaturesResult", SetCargoFeaturesResult.class);
        serializersMap.put("SetCargoFeaturesParams", SetCargoFeaturesParams.class);
        serializersMap.put("PackageFeatures", PackageFeatures.class);
        serializersMap.put("CargoBuildTarget", CargoBuildTarget.class);
        serializersMap.put("CargoFeaturesStateResult", CargoFeaturesStateResult.class);
        serializersMap.put("CppOptionsResult", CppOptionsResult.class);
        serializersMap.put("CppOptionsParams", CppOptionsParams.class);
        serializersMap.put("CppOptionsItem", CppOptionsItem.class);
        serializersMap.put("CppBuildTarget", CppBuildTarget.class);
        serializersMap.put("JavacOptionsResult", JavacOptionsResult.class);
        serializersMap.put("JavacOptionsItem", JavacOptionsItem.class);
        serializersMap.put("JavacOptionsParams", JavacOptionsParams.class);
        serializersMap.put("JvmTestEnvironmentResult", JvmTestEnvironmentResult.class);
        serializersMap.put("JvmBuildTarget", JvmBuildTarget.class);
        serializersMap.put("JvmTestEnvironmentParams", JvmTestEnvironmentParams.class);
        serializersMap.put("JvmRunEnvironmentResult", JvmRunEnvironmentResult.class);
        serializersMap.put("JvmMainClass", JvmMainClass.class);
        serializersMap.put("JvmRunEnvironmentParams", JvmRunEnvironmentParams.class);
        serializersMap.put("JvmEnvironmentItem", JvmEnvironmentItem.class);
        serializersMap.put("MavenDependencyModuleArtifact", MavenDependencyModuleArtifact.class);
        serializersMap.put("MavenDependencyModule", MavenDependencyModule.class);
        serializersMap.put("PythonBuildTarget", PythonBuildTarget.class);
        serializersMap.put("PythonOptionsResult", PythonOptionsResult.class);
        serializersMap.put("PythonOptionsItem", PythonOptionsItem.class);
        serializersMap.put("PythonOptionsParams", PythonOptionsParams.class);
        serializersMap.put("RustPackage", RustPackage.class);
        serializersMap.put("RustWorkspaceParams", RustWorkspaceParams.class);
        serializersMap.put("RustDepKindInfo", RustDepKindInfo.class);
        serializersMap.put("RustTarget", RustTarget.class);
        serializersMap.put("RustRawDependency", RustRawDependency.class);
        serializersMap.put("RustDependency", RustDependency.class);
        serializersMap.put("RustWorkspaceResult", RustWorkspaceResult.class);
        serializersMap.put("SbtBuildTarget", SbtBuildTarget.class);
        serializersMap.put("ScalaTestSuiteSelection", ScalaTestSuiteSelection.class);
        serializersMap.put("ScalaMainClassesParams", ScalaMainClassesParams.class);
        serializersMap.put("ScalaTextEdit", ScalaTextEdit.class);
        serializersMap.put("ScalaBuildTarget", ScalaBuildTarget.class);
        serializersMap.put("ScalacOptionsResult", ScalacOptionsResult.class);
        serializersMap.put("ScalaMainClass", ScalaMainClass.class);
        serializersMap.put("ScalaMainClassesResult", ScalaMainClassesResult.class);
        serializersMap.put("ScalacOptionsItem", ScalacOptionsItem.class);
        serializersMap.put("ScalaWorkspaceEdit", ScalaWorkspaceEdit.class);
        serializersMap.put("ScalacOptionsParams", ScalacOptionsParams.class);
        serializersMap.put("ScalaTestClassesResult", ScalaTestClassesResult.class);
        serializersMap.put("ScalaMainClassesItem", ScalaMainClassesItem.class);
        serializersMap.put("ScalaTestParams", ScalaTestParams.class);
        serializersMap.put("ScalaTestSuites", ScalaTestSuites.class);
        serializersMap.put("ScalaTestClassesParams", ScalaTestClassesParams.class);
        serializersMap.put("ScalaAction", ScalaAction.class);
        serializersMap.put("ScalaTestClassesItem", ScalaTestClassesItem.class);
        serializersMap.put("ScalaAttachRemote", ScalaAttachRemote.class);
        serializersMap.put("ScalaDiagnostic", ScalaDiagnostic.class);
    }

    public static void main(String[] args) {
        Gson gson = new GsonBuilder()
                .registerTypeAdapterFactory(new EitherTypeAdapterFactory())
                .create();
        Scanner scanner = new Scanner(System.in);

        while (true) {
            try {
                String inputLine = scanner.nextLine();
                if (inputLine == null || inputLine.equals(TERMINAL_STRING)) {
                    break;
                }

                DataWrapper wrapper = gson.fromJson(inputLine, DataWrapper.class);

                Class<?> clazz = serializersMap.get(wrapper.name);
                if (clazz == null) {
                    System.out.println("Type '" + wrapper.name + "' not known");
                    continue;
                }

                Object instance = gson.fromJson(wrapper.jsonData, clazz);

                JsonElement encodedInstance = gson.toJsonTree(instance);
                System.out.println(encodedInstance.toString());
            } catch (JsonSyntaxException | IllegalStateException e) {
                System.out.println("Error processing input: " + e.getMessage());
            }
        }
    }

    private static class DataWrapper {
        public String name;
        public JsonObject jsonData;
    }
}
