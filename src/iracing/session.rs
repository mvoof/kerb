/// Parsed representation of the iRacing session-info YAML blob.
/// Retains the full YAML for ad-hoc, path-based lookups.
#[derive(Debug, Clone)]
pub struct IracingSession {
    pub raw_yaml: serde_yaml::Value,
}

impl IracingSession {
    /// Parse raw session-info YAML emitted by iRacing into an `IracingSession`.
    /// Returns `None` if the YAML is malformed.
    pub fn from_yaml(yaml_str: &str) -> Option<Self> {
        let raw_yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str).ok()?;

        Some(Self { raw_yaml })
    }

    /// Look up an arbitrary value in the session YAML using a dot-separated path.
    /// Array elements are addressed with bracket notation, e.g. `"DriverInfo.Drivers[0].UserName"`.
    pub fn get_value(&self, path: &str) -> Option<String> {
        let mut current = &self.raw_yaml;

        for part in path.split('.') {
            if let Some(open_idx) = part.find('[') {
                let close_idx = part.find(']')?;

                if close_idx != part.len() - 1 {
                    return None;
                }

                let field = &part[..open_idx];

                let index: usize = part[open_idx + 1..close_idx].parse().ok()?;
                current = current.get(field)?.get(index)?;

                continue;
            }

            current = current.get(part)?;
        }

        match current {
            serde_yaml::Value::String(s) => Some(s.clone()),
            serde_yaml::Value::Number(n) => Some(n.to_string()),
            serde_yaml::Value::Bool(b) => Some(b.to_string()),
            _ => None,
        }
    }
}
