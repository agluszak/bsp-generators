use serde::{Deserialize, Serialize};

/** Task progress notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskProgressDataKind(pub std::borrow::Cow<'static, str>);
impl TaskProgressDataKind {
    pub const fn new(tag: &'static str) -> Self {
        TaskProgressDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
