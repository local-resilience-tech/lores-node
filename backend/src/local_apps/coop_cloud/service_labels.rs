use std::collections::HashMap;

#[allow(dead_code)]
pub struct CoopCloudServiceLabels {
    stack_namespace: String,
    data: HashMap<String, String>,
}

impl CoopCloudServiceLabels {
    pub fn new(labels: HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let stack_namespace = labels
            .get("com.docker.stack.namespace")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing com.docker.stack.namespace label"))?;

        let prefix = format!("coop-cloud.{stack_namespace}.");

        let data: HashMap<String, String> = labels
            .into_iter()
            .filter_map(|(key, value)| {
                if key.starts_with(&prefix) {
                    Some((key[prefix.len()..].to_string(), value))
                } else {
                    None
                }
            })
            .collect();

        Ok(CoopCloudServiceLabels {
            stack_namespace,
            data,
        })
    }

    pub fn version(&self) -> String {
        self.data
            .get("version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string())
    }
}
