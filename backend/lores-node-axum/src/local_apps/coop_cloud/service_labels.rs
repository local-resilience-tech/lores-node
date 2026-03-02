use std::collections::HashMap;

#[allow(dead_code)]
pub struct CoopCloudServiceLabels {
    stack_namespace: String,
    namespace_labels: HashMap<String, String>,
    traefik_labels: HashMap<String, String>,
}

impl CoopCloudServiceLabels {
    pub fn new(labels: HashMap<String, String>) -> Result<Self, anyhow::Error> {
        let stack_namespace = labels
            .get("com.docker.stack.namespace")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing com.docker.stack.namespace label"))?;

        let prefix = format!("coop-cloud.{stack_namespace}.");

        let namespace_labels: HashMap<String, String> = labels
            .iter()
            .filter_map(|(key, value)| {
                if key.starts_with(&prefix) {
                    Some((key[prefix.len()..].to_string(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        let traefik_labels: HashMap<String, String> = labels
            .iter()
            .filter_map(|(key, value)| {
                if key.starts_with("traefik.") {
                    Some((key.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        Ok(CoopCloudServiceLabels {
            stack_namespace,
            namespace_labels,
            traefik_labels,
        })
    }

    pub fn version(&self) -> String {
        self.namespace_labels
            .get("version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn host(&self) -> Option<String> {
        match self
            .traefik_labels
            .get(&format!(
                "traefik.http.routers.{}.rule",
                self.stack_namespace
            ))
            .cloned()
        {
            Some(rule) => host_from_host_rule(&rule),
            None => None,
        }
    }
}

fn host_from_host_rule(rule: &str) -> Option<String> {
    let prefix = "Host(`";
    let suffix = "`)";
    if rule.starts_with(prefix) && rule.ends_with(suffix) {
        Some(rule[prefix.len()..rule.len() - suffix.len()].to_string())
    } else {
        None
    }
}
