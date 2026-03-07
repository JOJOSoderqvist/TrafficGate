pub struct TrafficGateConfig {
    listener: ListenerAddr,
    upstreams: Vec<Upstream>,
    routes: Vec<Route>,
}

pub struct ListenerAddr {
    address: String,
}

pub struct Upstream {
    name: String,
    strategy: Strategy,
    backends: Vec<Backend>,
}

pub struct Backend {
    address: String,
    weight: Option<usize>,
}

pub struct Route {
    route_match: RouteMatch,
    upstream_name: String,
}

pub struct RouteMatch {
    host: String,
    path_prefix: String,
}

pub enum Strategy {
    WeighedRoundRobin,
    LeastConnections,
    RoundRobin,
}
