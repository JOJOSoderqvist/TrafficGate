use crate::request::RequestContext;
use config::model::config::{Route, TrafficGateConfig};

pub(crate) struct RouteEntry {
    pub(crate) host: Option<String>,
    pub(crate) path_prefix: String,
    pub(crate) upstream_name: String,
}

pub(crate) struct RoutingTable {
    pub entries: Vec<RouteEntry>,
}

pub struct RouteDecision {
    pub upstream_name: String,
}

impl RouteDecision {
    pub(crate) fn new(upstream_name: &str) -> Self {
        Self {
            upstream_name: upstream_name.to_string(),
        }
    }
}

// pub type RouteDecision<'a> = &'a str;

impl RoutingTable {
    pub fn from_config(config: &TrafficGateConfig) -> Self {
        let mut entries = config
            .routes
            .iter()
            .map(|route| RouteEntry {
                host: route.route_match.host.clone(),
                path_prefix: route.route_match.path_prefix.clone(),
                upstream_name: route.upstream_name.clone(),
            })
            .collect::<Vec<RouteEntry>>();

        entries.sort_by(|a, b| {
            let host_rule = a.host.is_some().cmp(&b.host.is_some()).reverse();
            let path_len_rule = a.path_prefix.len().cmp(&b.path_prefix.len()).reverse();
            host_rule.then(path_len_rule)
        });

        Self { entries }
    }

    pub fn match_request<'a>(&'a self, req: &RequestContext) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| Self::matches(e, req))
            .map(|m| m.upstream_name.as_str())
    }

    fn matches<'a>(entry: &RouteEntry, req: &RequestContext) -> bool {
        let host_matches = match (entry.host.as_deref(), req.host) {
            (Some(e), Some(r)) => e == r,
            (None, _) => true,
            (_, _) => false,
        };

        let path_matches = req.path.starts_with(&entry.path_prefix);

        host_matches && path_matches
    }
}
