//! Patch RGB value to ANSI-256 value in TOML document.

use toml_edit::{DocumentMut, Value};

use crate::toml_traverse::visit_item;

/// Transform value node of RGB hex values to ANSI-256 color index.
pub fn patch_doc(doc: &str) -> String {
    let mut doc = doc.parse::<DocumentMut>().expect("should be proper toml");

    visit_item(doc.as_item_mut(), |v| {
        if let Value::String(s) = v {
            if let Ok(Rgb { r, g, b }) = string_to_rgb(s.value()) {
                let ansi_256 = rgb2ansi256::rgb_to_ansi256(r, g, b);

                // extract old decoration
                let decor = s.decor();
                let prefix = decor.prefix().unwrap();
                let suffix = decor.suffix().unwrap();

                // wrap ansi_256 with decoration
                let new_value =
                    Value::from(ansi_256.to_string()).decorated(prefix.clone(), suffix.clone());

                *v = new_value;
            }
        }
    });

    doc.to_string()
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fn string_to_rgb(s: &str) -> Result<Rgb, String> {
    if s.starts_with('#') && s.len() >= 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[1..3], 16),
            u8::from_str_radix(&s[3..5], 16),
            u8::from_str_radix(&s[5..7], 16),
        ) {
            Ok(Rgb { r, g, b })
        } else {
            Err(format!("bad rgb hexcode: {s}"))
        }
    } else {
        Err(format!("{s} is not a rgb str"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_value_to_ansi256() {
        const INPUT: &str = r##"
        a = "unchanged"
        b = "#00ffaf"
        "##;

        const OUTPUT: &str = r##"
        a = "unchanged"
        b = "49"
        "##;

        assert_eq!(patch_doc(INPUT), OUTPUT);
    }
}
