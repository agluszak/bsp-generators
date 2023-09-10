use serde::{Deserialize, Serialize};

/** The Rust edition. */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Edition(pub std::borrow::Cow<'static, str>);
impl Edition {
    pub const E2015: Edition = Edition::new("2015");
    pub const E2018: Edition = Edition::new("2018");
    pub const E2021: Edition = Edition::new("2021");

    pub const fn new(tag: &'static str) -> Self {
        Edition(std::borrow::Cow::Borrowed(tag))
    }
}
