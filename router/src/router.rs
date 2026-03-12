use crate::request::RequestContext;
use crate::routing_table::{RouteDecision, RoutingTable};
use arc_swap::ArcSwap;
use config::model::config::TrafficGateConfig;
use std::sync::Arc;

pub struct Router {
    routing_table: ArcSwap<RoutingTable>,
}

impl Router {
    pub fn new(initial_config: &TrafficGateConfig) -> Self {
        let table = Arc::new(RoutingTable::from_config(initial_config));

        Self {
            routing_table: ArcSwap::new(table),
        }
    }

    pub fn update_routing_table(&self, new_config: &TrafficGateConfig) {
        let new_table = Arc::new(RoutingTable::from_config(new_config));
        self.routing_table.store(new_table)
    }

    pub fn match_request(&self, req: &RequestContext) -> Option<RouteDecision> {
        self.routing_table
            .load()
            .match_request(req)
            .map(|a| RouteDecision::new(a))
    }
}
