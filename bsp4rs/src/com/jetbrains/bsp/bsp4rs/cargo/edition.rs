use serde::{Deserialize, Serialize};

/** The Rust edition.
As of writing this comment rust editions 2024, 2027 and 2030 are not
actually a thing yet but are parsed nonetheless for future proofing. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Edition(pub std::borrow::Cow<'static, str>);
impl Edition {
    pub const E2015: Edition = Edition::new("2015");
    pub const E2018: Edition = Edition::new("2018");
    pub const E2021: Edition = Edition::new("2021");
    pub const _E2024: Edition = Edition::new("2024");
    pub const _E2027: Edition = Edition::new("2027");
    pub const _E2030: Edition = Edition::new("2030");

    pub const fn new(tag: &'static str) -> Self {
        Edition(std::borrow::Cow::Borrowed(tag))
    }
}
