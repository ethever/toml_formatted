use toml_edit::DocumentMut;

pub trait TomlDocExt {
    fn to_toml_document_formatted(&self) -> DocumentMut;
    fn to_toml_string_formatted(&self) -> String {
        self.to_toml_document_formatted().to_string()
    }
}

/// Convert an array-of-tables at `doc[field]` into an inline array of inline tables.
pub fn inline_field(doc: &mut DocumentMut, field: &str) -> bool {
    use toml_edit::{Array, InlineTable, value};

    let item = &mut doc[field];
    if let Some(aot) = item.as_array_of_tables() {
        let mut arr = Array::default();
        for t in aot {
            let mut inline = InlineTable::default();
            for (k, v) in t.iter() {
                if let Some(val) = v.as_value().cloned() {
                    inline.insert(k, val);
                }
            }
            arr.push(inline);
        }
        *item = value(arr);
        true
    } else {
        false
    }
}
