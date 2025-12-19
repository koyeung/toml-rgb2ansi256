//! Traverse TOML document.
use toml_edit::{Item, Table, Value};

/// Apply function on `Value` node while traverse from input `Item`.
pub fn visit_item<F: Fn(&mut Value) + Copy>(item: &mut Item, f: F) {
    match item {
        Item::Value(value) => visit_value(value, f),
        Item::Table(table) => visit_table(table, f),
        Item::ArrayOfTables(array_of_tables) => {
            array_of_tables.iter_mut().for_each(|t| visit_table(t, f))
        }
        Item::None => (),
    }
}

fn visit_table<F: Fn(&mut Value) + Copy>(table: &mut Table, f: F) {
    for (_, item) in table.iter_mut() {
        visit_item(item, f);
    }
}

fn visit_value<F: Fn(&mut Value) + Copy>(value: &mut Value, f: F) {
    match value {
        Value::InlineTable(inline_table) => {
            inline_table.iter_mut().for_each(|(_, v)| visit_value(v, f))
        }
        Value::Array(array) => array.iter_mut().for_each(|v| visit_value(v, f)),
        _ => f(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml_edit::DocumentMut;

    #[test]
    fn test_modify_values() {
        const INPUT: &str = r##"
        # this is comment

        a = "a"
        "a.b" = "a.b"          # comment on keyword.directive
        c = { d = "c.d" }

        [p]
        "a" = "p.a"  # some comment
        "b" = "p.b"

        [[q]]
        a = "a1"

        [[q]]
        a = "a2"
        "##;

        const OUTPUT: &str = r##"
        # this is comment

        a = "A"
        "a.b" = "A.B"          # comment on keyword.directive
        c = { d = "C.D" }

        [p]
        "a" = "P.A"  # some comment
        "b" = "P.B"

        [[q]]
        a = "A1"

        [[q]]
        a = "A2"
        "##;

        let mut doc = INPUT.parse::<DocumentMut>().expect("doc should be in toml");

        visit_item(doc.as_item_mut(), |v| {
            if let Value::String(s) = v {
                let old_str_value = s.value();

                let decor = s.decor();
                let prefix = decor.prefix().unwrap();
                let suffix = decor.suffix().unwrap();

                let new_v = Value::from(old_str_value.to_uppercase());
                let decorated_new_v = new_v.decorated(prefix.clone(), suffix.clone());

                *v = decorated_new_v;
            }
        });

        assert_eq!(doc.to_string(), OUTPUT);
    }
}
