use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub listener: RawListenerAddr,
    #[serde(default)]
    pub upstreams: Vec<RawUpstream>,
    #[serde(default)]
    pub routes: Vec<RawRoute>,
}

#[derive(Debug, Deserialize)]
pub struct RawListenerAddr {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct RawUpstream {
    pub name: String,
    pub strategy: Option<RawStrategy>,
    #[serde(default)]
    pub backends: Vec<RawBackend>,
}

#[derive(Debug, Deserialize)]
pub struct RawBackend {
    pub address: String,
    pub weight: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct RawRoute {
    pub route_match: RawRouteMatch,
    pub upstream_name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawRouteMatch {
    pub host: Option<String>,
    pub path_prefix: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawStrategy {
    WeightedRoundRobin,
    LeastConnections,
    RoundRobin,
}