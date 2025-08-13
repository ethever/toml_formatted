// Re-export the trait from the core crate
pub use core::*;
use toml_formatted_core as core;

// Re-export the derive macro from the macros crate
pub use macros::*;
use toml_formatted_macros as macros;

pub use toml_edit;

pub fn to_toml_string_formatted<T: core::TomlDocExt + ?Sized>(t: &T) -> String {
    core::TomlDocExt::to_toml_string_formatted(t)
}
