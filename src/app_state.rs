use std::sync::Arc;

use crate::config::Config;
use crate::discovery::service::DiscoveryService;
use crate::groups::service::GroupService;
use crate::targets::service::TargetService;
use crate::labels::service::LabelService;

#[derive(Clone)]
pub struct AppState {
    pub discovery_service: Arc<DiscoveryService>,
    pub group_service: Arc<GroupService>,
    pub target_service: Arc<TargetService>,
    pub label_service: Arc<LabelService>,
    pub config: Arc<Config>,
}
