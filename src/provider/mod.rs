pub mod external;
pub mod local;
pub mod trait_;

pub use external::ExternalProvider;
pub use local::LocalProvider;
pub use trait_::InferenceProvider;
