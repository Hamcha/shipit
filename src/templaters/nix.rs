use anyhow::Result;
use bytes::Bytes;
use std::collections::HashMap;
use std::str;

pub fn update_file(file: &Bytes, changes: &HashMap<String, String>) -> Result<Bytes> {
    let mut file_str: String = str::from_utf8(&file).unwrap().to_string();

    // apply changes
    for (path, value) in changes {
        let new_value = if value.starts_with("r##") {
            value.strip_prefix("r##").unwrap().to_string()
        } else {
            format!("\"{}\"", value)
        };
        file_str = nix_editor::write::write(&file_str, path, &new_value)?;
    }

    Ok(Bytes::from(file_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_file() {
        let original = r#"
	{
	    test = {
	        nested = "original";
	    };

        test.inline = "original";
	}"#;

        let file = update_file(
            &Bytes::from(original),
            &HashMap::from([
                ("test.nested".to_string(), "changed".to_string()),
                ("test.inline".to_string(), "changed".to_string()),
            ]),
        )
        .unwrap();

        let file_str = str::from_utf8(&file).unwrap();
        assert_eq!(
            nix_editor::read::readvalue(file_str, "test.nested").unwrap(),
            "\"changed\""
        );
        assert_eq!(
            nix_editor::read::readvalue(file_str, "test.inline").unwrap(),
            "\"changed\""
        );
    }
}
