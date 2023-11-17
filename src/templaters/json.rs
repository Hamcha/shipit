use anyhow::{anyhow, Result};
use bytes::Bytes;
use serde_json::Value;
use std::collections::HashMap;

/// returns the value at the given path
fn patch(base: &mut Value, path: &str, value: &str) -> Result<()> {
    let mut current = base;
    for part in path.split('/') {
        // if part is numeric, treat it as array index
        if let Ok(numeric_part) = part.parse::<usize>() {
            current = current
                .get_mut(numeric_part)
                .ok_or_else(|| anyhow!("could not find index path {}", path))?;
            continue;
        }

        // otherwise treat it as object key
        current = current
            .get_mut(part)
            .ok_or_else(|| anyhow!("could not find object path {}", path))?;
    }
    *current = Value::from(value);
    Ok(())
}

pub fn update_file(file: &Bytes, changes: &HashMap<String, String>) -> Result<Bytes> {
    let mut parsed = serde_json::from_slice(file)?;

    // apply changes
    for (path, value) in changes {
        patch(&mut parsed, path, value)?;
    }

    Ok(Bytes::from(serde_json::to_vec(&parsed)?))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_update_file() {
        let original = json!({"test": {"nested": "dummy"}, "array": ["changeme"]});

        let file = update_file(
            &Bytes::from(serde_json::to_vec(&original).unwrap()),
            &HashMap::from([
                ("test/nested".to_string(), "changed".to_string()),
                ("array/0".to_string(), "changed".to_string()),
            ]),
        )
        .unwrap();

        let parsed: Value = serde_json::from_slice(&file).unwrap();

        assert_eq!(parsed["test"]["nested"], "changed");
        assert_eq!(parsed["array"][0], "changed");
    }
}
