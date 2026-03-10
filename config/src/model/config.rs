pub struct TrafficGateConfig {
    pub listener: ListenerAddr,
    pub upstreams: Vec<Upstream>,
    pub routes: Vec<Route>,
}

pub struct ListenerAddr {
    pub address: String,
}

pub struct Upstream {
    pub name: String,
    pub strategy: Strategy,
    pub backends: Vec<Backend>,
}

pub struct Backend {
    pub address: String,
    pub weight: Option<usize>,
}

pub struct Route {
    pub route_match: RouteMatch,
    pub upstream_name: String,
}

pub struct RouteMatch {
    pub host: Option<String>,
    pub path_prefix: String,
}

pub enum Strategy {
    WeighedRoundRobin,
    LeastConnections,
    RoundRobin,
}
