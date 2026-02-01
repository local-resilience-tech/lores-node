use crate::{
    data::entities::{LocalApp, LocalAppInstallStatus, Node, NodeAppUrl},
    docker::docker_stack::docker_stack_ls,
};

pub fn find_deployed_local_apps(node: &Node) -> Vec<LocalApp> {
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let local_apps = deployed_stacks
        .into_iter()
        .map(|stack| LocalApp {
            name: stack.name.clone(),
            version: "unknown".to_string(),
            status: LocalAppInstallStatus::StackDeployed,
            url: Some(NodeAppUrl {
                internet_url: app_url(&stack.name, node.domain_on_internet.as_deref()),
                local_network_url: app_url(&stack.name, node.domain_on_local_network.as_deref()),
            }),
        })
        .collect();

    local_apps
}

fn app_url(app_name: &str, domain: Option<&str>) -> Option<String> {
    domain.map(|d| format!("http://{}.{}", app_name, d))
}
