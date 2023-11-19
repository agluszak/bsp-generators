use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTaskProgressData {}

/// Task progress notifications may contain an arbitrary interface in their `data`
/// field. The kind of interface that is contained in a notification must be
/// specified in the `dataKind` field.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskProgressData {
    Named(NamedTaskProgressData),
    Other(OtherData),
}

impl TaskProgressData {}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;

    #[test]
    fn task_progress_data() {
        assert_compact_json_snapshot!(
           TaskProgressData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
