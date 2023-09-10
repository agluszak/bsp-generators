use crate::*;

#[derive(Debug)]
pub enum OnBuildShowMessage {}

impl Notification for OnBuildShowMessage {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "build/showMessage";
}

#[derive(Debug)]
pub enum OnBuildLogMessage {}

impl Notification for OnBuildLogMessage {
    type Params = LogMessageParams;
    const METHOD: &'static str = "build/logMessage";
}

#[derive(Debug)]
pub enum OnBuildPublishDiagnostics {}

impl Notification for OnBuildPublishDiagnostics {
    type Params = PublishDiagnosticsParams;
    const METHOD: &'static str = "build/publishDiagnostics";
}

#[derive(Debug)]
pub enum OnBuildTargetDidChange {}

impl Notification for OnBuildTargetDidChange {
    type Params = DidChangeBuildTarget;
    const METHOD: &'static str = "buildTarget/didChange";
}

#[derive(Debug)]
pub enum OnBuildTaskStart {}

impl Notification for OnBuildTaskStart {
    type Params = TaskStartParams;
    const METHOD: &'static str = "build/taskStart";
}

#[derive(Debug)]
pub enum OnBuildTaskProgress {}

impl Notification for OnBuildTaskProgress {
    type Params = TaskProgressParams;
    const METHOD: &'static str = "build/taskProgress";
}

#[derive(Debug)]
pub enum OnBuildTaskFinish {}

impl Notification for OnBuildTaskFinish {
    type Params = TaskFinishParams;
    const METHOD: &'static str = "build/taskFinish";
}
