use crate::service_labels::CoopCloudServiceLabels;

#[derive(Debug, Clone)]
pub struct AppUrl {
    pub internet_url: Option<String>,
    pub local_network_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LoResApp {
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CoopCloudApp {
    pub name: String,
    pub recipe: String,
    pub version: Option<String>,
    pub url: Option<AppUrl>,
    pub lores: Option<LoResApp>,
}

pub fn build_coop_cloud_app_from_labels(
    labels: &CoopCloudServiceLabels,
) -> Result<CoopCloudApp, anyhow::Error> {
    let recipe = labels
        .recipe()
        .ok_or_else(|| anyhow::anyhow!("Missing recipe label"))?;

    Ok(CoopCloudApp {
        name: recipe.clone(),
        recipe: recipe.clone(),
        version: labels.version(),
        url: Some(AppUrl {
            internet_url: app_url(labels.host()),
            local_network_url: None,
        }),
        lores: labels.lores_instance_id().map(|id| LoResApp {
            instance_id: Some(id),
        }),
    })
}

fn app_url(host: Option<String>) -> Option<String> {
    host.map(|h| format!("https://{}", h))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack_namespace_label(namespace: &str) -> (String, String) {
        (
            "com.docker.stack.namespace".to_string(),
            namespace.to_string(),
        )
    }

    fn version_label(namespace: &str, version: &str) -> (String, String) {
        (
            format!("coop-cloud.{}.version", namespace),
            version.to_string(),
        )
    }

    fn recipe_label(namespace: &str, recipe: &str) -> (String, String) {
        (
            format!("coop-cloud.{}.recipe", namespace),
            recipe.to_string(),
        )
    }

    #[test]
    fn test_build_app_gets_version() {
        let labels = CoopCloudServiceLabels::new(
            vec![
                version_label("foobar", "1.2.3"),
                stack_namespace_label("foobar"),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();

        let result = build_coop_cloud_app_from_labels(&labels).unwrap();
        assert_eq!(result.version, Some("1.2.3".to_string()));
    }

    #[test]
    fn test_build_has_no_version_if_not_specified() {
        let labels = CoopCloudServiceLabels::new(
            vec![stack_namespace_label("foobar")].into_iter().collect(),
        )
        .unwrap();

        let result = build_coop_cloud_app_from_labels(&labels).unwrap();
        assert_eq!(result.version, None);
    }

    #[test]
    fn test_build_fails_if_no_recipe() {
        let labels = CoopCloudServiceLabels::new(
            vec![stack_namespace_label("foobar")].into_iter().collect(),
        )
        .unwrap();

        let result = build_coop_cloud_app_from_labels(&labels);
        assert!(result.is_err());
    }

    #[test]
    fn test_recipe_comes_from_recipe() {
        let labels = CoopCloudServiceLabels::new(
            vec![
                stack_namespace_label("foobar"),
                recipe_label("foobar", "my-recipe"),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();

        let result = build_coop_cloud_app_from_labels(&labels).unwrap();
        assert_eq!(result.recipe, "my-recipe".to_string());
    }

    #[test]
    fn test_name_comes_from_recipe() {
        let labels = CoopCloudServiceLabels::new(
            vec![
                stack_namespace_label("foobar"),
                recipe_label("foobar", "my-recipe"),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();

        let result = build_coop_cloud_app_from_labels(&labels).unwrap();
        assert_eq!(result.name, "my-recipe".to_string());
    }
}
