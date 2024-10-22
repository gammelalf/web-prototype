use framework_core::capability::{Capability, CapabilityProvider};

pub trait DatabaseAccess {}
impl Capability for dyn DatabaseAccess {
    fn name() -> &'static str {
        "DatabaseAccess"
    }

    fn providers() -> Vec<CapabilityProvider> {
        vec![CapabilityProvider {
            crate_name: "framework-db".to_string(),
            module_path: Some("module::Db".to_string()),
            crates_url: None,
            docs_url: None,
            repo_url: Some("https://github.com/gammelalf/web-prototype".to_string()),
        }]
    }
}
