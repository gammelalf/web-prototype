use crate::registry::ModuleDependencies;
use std::error::Error;
use std::future::Future;

/// A module is a globally available singleton which exists for the entire duration of the application.
pub trait Module: Sized + Send + Sync + 'static {
    /// Arbitrary data the `pre_init` function may pass to `init`
    type PreInit: Sized + Send + Sync + 'static;
    fn pre_init() -> impl Future<Output = Result<Self::PreInit, PreInitError>> + Send;

    /// A tuple of [`Module`]s which need to be initialized before this one.
    type Dependencies: ModuleDependencies;
    fn init(
        pre_init: Self::PreInit,
        dependencies: &mut Self::Dependencies,
    ) -> impl Future<Output = Result<Self, InitError>> + Send;

    fn post_init(&'static self) -> impl Future<Output = Result<(), PostInitError>> + Send;
}

pub type PreInitError = Box<dyn Error + Send + Sync + 'static>;
pub type InitError = Box<dyn Error + Send + Sync + 'static>;
pub type PostInitError = Box<dyn Error + Send + Sync + 'static>;
