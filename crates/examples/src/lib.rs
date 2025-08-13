#[cfg(test)]
mod tests {
    use {
        serde::{Deserialize, Serialize},
        toml_formatted::{TomlDocFormatted, to_toml_string_formatted},
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
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            name: Default::default(),
        };

        let res = to_toml_string_formatted(&config);

        println!("{}", res);
    }
}
