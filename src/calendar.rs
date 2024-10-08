use serde::{Deserialize, Serialize};
use crate::skyblock::SkyblockEvent;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Event {
    Skyblock(SkyblockEvent)
}


