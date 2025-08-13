// Re-export the trait from the core crate
pub use core::*;
use toml_formatted_core as core;

// Re-export the derive macro from the macros crate
pub use macros::*;
use toml_formatted_macros as macros;

#[cfg(test)]
mod tests {
    use toml_formatted_core::TomlDocExt;
    use {
        serde::{Deserialize, Serialize},
        toml_formatted_macros::TomlDocFormatted,
    };

    #[derive(Debug, Clone, Serialize, Deserialize, TomlDocFormatted, Default)]
    struct Config {
        #[inline_table]
        should_be_inline_table: Vec<Pair>,

        name: String,

        non_nested: Vec<Nested>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default, TomlDocFormatted)]
    struct Pair {
        name: String,
        u8: u8,
        #[inline_table]
        nested_inline_should_work: Vec<Nested>,
        non_inline_table: Vec<Nested>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    struct Nested {
        u32: u32,
        u8: u8,
    }

    #[test]
    fn test_serialize() {
        let nesteds = vec![Default::default(), Default::default()];
        let config = Config {
            non_nested: nesteds.clone(),
            should_be_inline_table: vec![
                Pair {
                    nested_inline_should_work: nesteds.clone(),
                    non_inline_table: nesteds,
                    ..Default::default()
                },
                Default::default(),
            ],
            name: Default::default(),
        };

        let res = config.to_toml_string_formatted();

        println!("{}", res);
    }
}
