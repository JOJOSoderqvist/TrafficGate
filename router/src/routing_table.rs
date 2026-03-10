use config::model::config::{Route, TrafficGateConfig};

pub(crate) struct RouteEntry {
    pub(crate) host: Option<String>,
    pub(crate) path_prefix: String,
    pub(crate) upstream_name: String,
}


pub(crate) struct RoutingTable {
    pub entries: Vec<RouteEntry>
}

impl RoutingTable {
    pub fn from_config(config: &TrafficGateConfig) -> Self {
        let mut entries = config
            .routes
            .iter()
            .map(|route| RouteEntry {
                host: route.route_match.host.clone(),
                path_prefix: route.route_match.path_prefix.clone(),
                upstream_name: route.upstream_name.clone(),
            }).collect::<Vec<RouteEntry>>();

        entries.sort_by(
            |a, b| {
                let host_rule = a.host.is_some().cmp(&b.host.is_some()).reverse();
                let path_len_rule = a.path_prefix.len().cmp(&b.path_prefix.len()).reverse();
                host_rule.then(path_len_rule)
            }
        );

        Self {
            entries,
        }
    }
}