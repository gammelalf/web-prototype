use std::any::type_name;

pub trait Capability {
    /// The capability's name used when printing diagnostics
    fn name() -> &'static str {
        type_name::<Self>()
    }

    /// A list of [`Module`]s providing the capability
    ///
    /// This list is consulted when an application author is missing a capability.
    /// It suggests a module which provides the missing capability.
    ///
    /// This is a completely optional feature and can be ignored by a capability author.
    fn providers() -> Vec<CapabilityProvider> {
        vec![]
    }
}

/// A module providing a capability which is suggested by the capability's author.
#[derive(Debug)]
pub struct CapabilityProvider {
    /// The module's crate
    pub crate_name: String,

    /// The module's import path inside its crate
    pub module_path: Option<String>,

    /// The crate's [crates.io](https://crates.io/) page
    pub crates_url: Option<String>,

    /// The crate's documentation
    pub docs_url: Option<String>,

    /// The crate's repository
    pub repo_url: Option<String>,
}
