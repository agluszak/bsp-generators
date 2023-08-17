use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticDataKind(pub std::borrow::Cow<'static, str>);
impl DiagnosticDataKind {
    /** `data` field must contain a ScalaDiagnostic object. */
    pub const SCALA: DiagnosticDataKind = DiagnosticDataKind::new("scala");

    pub const fn new(tag: &'static str) -> Self {
        DiagnosticDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
