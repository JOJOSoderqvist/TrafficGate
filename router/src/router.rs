use std::sync::Arc;
use arc_swap::ArcSwap;
use config::model::config::TrafficGateConfig;
use crate::request::RequestContext;
use crate::routing_table::{RouteDecision, RoutingTable};

pub struct Router {
    routing_table: ArcSwap<RoutingTable>
}

impl Router {
    pub fn new(initial_config: &TrafficGateConfig) -> Self {
        let table = Arc::new(RoutingTable::from_config(initial_config));

        Self {
            routing_table: ArcSwap::new(table),
        }
    }

    pub fn update_table(&self, new_config: &TrafficGateConfig) {
        let new_table = Arc::new(RoutingTable::from_config(new_config));
        self.routing_table.store(new_table)
    }

    // TODO: Do i need to allocate?
    pub fn match_request(& self, req: &RequestContext) -> Option<String> {
        self.routing_table.load().match_request(req).map(|a| a.to_string())
    }
}