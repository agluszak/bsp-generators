use serde::{Deserialize, Serialize};

/// The debug session will connect to a running process. The DAP client will send the port of the running process later.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaAttachRemote {}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn scala_attach_remote() {
        assert_json_snapshot!(
           ScalaAttachRemote {},
           @r#"
{}
   "#
        );
    }
}
