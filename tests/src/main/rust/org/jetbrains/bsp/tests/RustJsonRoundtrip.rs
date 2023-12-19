use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use bsp4rs::bsp::*;
use bsp4rs::cancel::*;
use bsp4rs::cargo::*;
use bsp4rs::cpp::*;
use bsp4rs::java::*;
use bsp4rs::jvm::*;
use bsp4rs::maven::*;
use bsp4rs::python::*;
use bsp4rs::rust::*;
use bsp4rs::sbt::*;
use bsp4rs::scala::*;
use bsp4rs::*;

// TODO Kasia: move it to some file and make it generate automatically
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", content = "jsonData")]
enum SerializedData {
    Range(Range),
    CompileResult(CompileResult),
    WorkspaceBuildTargetsResult(WorkspaceBuildTargetsResult),
    TestFinish(TestFinish),
    DependencySourcesResult(DependencySourcesResult),
    Position(Position),
    BspConnectionDetails(BspConnectionDetails),
    TestProvider(TestProvider),
    RunParams(RunParams),
    InverseSourcesParams(InverseSourcesParams),
    TaskProgressParams(TaskProgressParams),
    PrintParams(PrintParams),
    TaskStartParams(TaskStartParams),
    OutputPathsResult(OutputPathsResult),
    CompileReport(CompileReport),
    BuildTargetIdentifier(BuildTargetIdentifier),
    ResourcesItem(ResourcesItem),
    OutputPathsItem(OutputPathsItem),
    RunResult(RunResult),
    ResourcesResult(ResourcesResult),
    CompileTask(CompileTask),
    CleanCacheResult(CleanCacheResult),
    TestReport(TestReport),
    TaskId(TaskId),
    BuildClientCapabilities(BuildClientCapabilities),
    InverseSourcesResult(InverseSourcesResult),
    SourcesResult(SourcesResult),
    BuildTargetCapabilities(BuildTargetCapabilities),
    OutputPathsParams(OutputPathsParams),
    DependencyModulesItem(DependencyModulesItem),
    DebugProvider(DebugProvider),
    CleanCacheParams(CleanCacheParams),
    SourcesParams(SourcesParams),
    TestTask(TestTask),
    SourceItem(SourceItem),
    BuildTargetEvent(BuildTargetEvent),
    ResourcesParams(ResourcesParams),
    DependencyModulesResult(DependencyModulesResult),
    DiagnosticRelatedInformation(DiagnosticRelatedInformation),
    CodeDescription(CodeDescription),
    SourcesItem(SourcesItem),
    TextDocumentIdentifier(TextDocumentIdentifier),
    BuildTarget(BuildTarget),
    InitializeBuildParams(InitializeBuildParams),
    RunProvider(RunProvider),
    DependencyModule(DependencyModule),
    DidChangeBuildTarget(DidChangeBuildTarget),
    BuildServerCapabilities(BuildServerCapabilities),
    TaskFinishParams(TaskFinishParams),
    LogMessageParams(LogMessageParams),
    TestResult(TestResult),
    OutputPathItem(OutputPathItem),
    DebugSessionParams(DebugSessionParams),
    DependencySourcesParams(DependencySourcesParams),
    CompileProvider(CompileProvider),
    CompileParams(CompileParams),
    InitializeBuildResult(InitializeBuildResult),
    DependencyModulesParams(DependencyModulesParams),
    ReadParams(ReadParams),
    Location(Location),
    DependencySourcesItem(DependencySourcesItem),
    TestStart(TestStart),
    PublishDiagnosticsParams(PublishDiagnosticsParams),
    TestParams(TestParams),
    DebugSessionAddress(DebugSessionAddress),
    Diagnostic(Diagnostic),
    ShowMessageParams(ShowMessageParams),
    CancelRequestParams(CancelRequestParams),
    SetCargoFeaturesResult(SetCargoFeaturesResult),
    SetCargoFeaturesParams(SetCargoFeaturesParams),
    PackageFeatures(PackageFeatures),
    CargoBuildTarget(CargoBuildTarget),
    CargoFeaturesStateResult(CargoFeaturesStateResult),
    CppOptionsResult(CppOptionsResult),
    CppOptionsParams(CppOptionsParams),
    CppOptionsItem(CppOptionsItem),
    CppBuildTarget(CppBuildTarget),
    JavacOptionsResult(JavacOptionsResult),
    JavacOptionsItem(JavacOptionsItem),
    JavacOptionsParams(JavacOptionsParams),
    JvmTestEnvironmentResult(JvmTestEnvironmentResult),
    JvmBuildTarget(JvmBuildTarget),
    JvmTestEnvironmentParams(JvmTestEnvironmentParams),
    JvmRunEnvironmentResult(JvmRunEnvironmentResult),
    JvmMainClass(JvmMainClass),
    JvmRunEnvironmentParams(JvmRunEnvironmentParams),
    JvmEnvironmentItem(JvmEnvironmentItem),
    MavenDependencyModuleArtifact(MavenDependencyModuleArtifact),
    MavenDependencyModule(MavenDependencyModule),
    PythonBuildTarget(PythonBuildTarget),
    PythonOptionsResult(PythonOptionsResult),
    PythonOptionsItem(PythonOptionsItem),
    PythonOptionsParams(PythonOptionsParams),
    RustPackage(RustPackage),
    RustWorkspaceParams(RustWorkspaceParams),
    RustDepKindInfo(RustDepKindInfo),
    RustTarget(RustTarget),
    RustRawDependency(RustRawDependency),
    RustDependency(RustDependency),
    RustWorkspaceResult(RustWorkspaceResult),
    SbtBuildTarget(SbtBuildTarget),
    ScalaTestSuiteSelection(ScalaTestSuiteSelection),
    ScalaMainClassesParams(ScalaMainClassesParams),
    ScalaTextEdit(ScalaTextEdit),
    ScalaBuildTarget(ScalaBuildTarget),
    ScalacOptionsResult(ScalacOptionsResult),
    ScalaMainClass(ScalaMainClass),
    ScalaMainClassesResult(ScalaMainClassesResult),
    ScalacOptionsItem(ScalacOptionsItem),
    ScalaWorkspaceEdit(ScalaWorkspaceEdit),
    ScalacOptionsParams(ScalacOptionsParams),
    ScalaTestClassesResult(ScalaTestClassesResult),
    ScalaMainClassesItem(ScalaMainClassesItem),
    ScalaTestParams(ScalaTestParams),
    ScalaTestSuites(ScalaTestSuites),
    ScalaTestClassesParams(ScalaTestClassesParams),
    ScalaAction(ScalaAction),
    ScalaTestClassesItem(ScalaTestClassesItem),
    ScalaAttachRemote(ScalaAttachRemote),
    ScalaDiagnostic(ScalaDiagnostic),
    OtherData(OtherData),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DataWrapper {
    name: String,
    json_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DataWrapperWithJson {
    name: String,
    json_data: serde_json::Value,
}

const TERMINAL_STRING: &str = "exit";

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let mut input_line = String::new();
        reader.read_line(&mut input_line).unwrap();

        if input_line.is_empty() || input_line.trim() == TERMINAL_STRING {
            break;
        }

        let wrapper = serde_json::from_str::<DataWrapper>(&input_line).unwrap();
        let wrapper_with_json = DataWrapperWithJson {
            name: wrapper.name,
            json_data: serde_json::from_str(&wrapper.json_data).unwrap(),
        };
        let wrapper_as_value = serde_json::to_value(wrapper_with_json).unwrap();

        let serialized_data = serde_json::from_value::<SerializedData>(wrapper_as_value);
        if let Ok(instance) = serialized_data {
            let instance_as_value = serde_json::to_value(instance).unwrap();
            let decoded_instance =  &instance_as_value["jsonData"];

            println!("{}", serde_json::to_string(decoded_instance).unwrap());
        } else {
            println!("Error decoding JSON data");
        }
    }
}
