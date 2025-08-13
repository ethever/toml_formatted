use toml_edit::DocumentMut;
use toml_edit::{Array, InlineTable, value};

pub trait TomlDocExt {
    fn to_toml_document_formatted(&self) -> DocumentMut;
    fn to_toml_string_formatted(&self) -> String {
        self.to_toml_document_formatted().to_string()
    }
}

// /// Convert an array-of-tables at `doc[field]` into an inline array of inline tables.
// pub fn inline_field(doc: &mut DocumentMut, field: &str) -> bool {
//     let item = &mut doc[field];
//     if let Some(aot) = item.as_array_of_tables() {
//         let mut arr = Array::default();
//         for t in aot {
//             let mut inline = InlineTable::default();
//             for (k, v) in t.iter() {
//                 if let Some(mut val) = v.as_value().cloned() {
//                     // do not work. why??
//                     val.decor_mut().set_prefix("                   dsadas");

//                     inline.insert(k, val);
//                     // do not work. why??
//                     inline.decor_mut().set_prefix("xxxxxxxxxxxxxxxxxxx");
//                     // do not work. why??
//                     inline.decor_mut().set_suffix("\n\n\n\n");
//                     inline.fmt();
//                 }
//             }
//             // works
//             arr.set_trailing("dasdasdasdasdada");
//             arr.push(inline);
//         }
//         // arr.fmt();
//         *item = value(arr);
//         true
//     } else {
//         false
//     }
// }

/// Ensure `doc[field]` is rendered as a multi-line array of inline tables:
/// field = [
///     { a = 1, b = 2 },
///     { a = 3, b = 4 },
/// ]
pub fn inline_field(doc: &mut DocumentMut, field: &str) -> bool {
    let item = &mut doc[field];

    // Case 1: [[field]] (Array-of-Tables) -> convert to array of inline tables
    if let Some(aot) = item.as_array_of_tables() {
        let mut arr = Array::default();

        for t in aot {
            let mut it = InlineTable::default();
            for (k, v) in t.iter() {
                if let Some(val) = v.as_value().cloned() {
                    it.insert(k, val);
                    it.decor_mut().set_prefix("\n    ");
                }
            }

            it.fmt();

            arr.push_formatted(it.into());
        }

        arr.set_trailing("\n");
        *item = value(arr);
        return true;
    }

    false
}
