use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuctionItem {
    Pet(Pet),
    Armor(Armor),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pet {
    name: String,
    level: i16,
    exp: i32,
    candy: i8,
}

impl Pet {
    pub fn new(name: String, level: i16, exp: i32, candy: i8) -> Self {
        Pet {
            name,
            level,
            exp,
            candy,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Armor {
    name: String,
    armor_type: ArmorType,
    enchants: Vec<String>,
    gemstones: Vec<String>,
    fuming: i16,
    art_of_peace: i16,
    reforge: String,
    star: i16,
}

impl Armor {
    pub(crate) fn new(
        name: String,
        armor_type: ArmorType,
        enchants: Vec<String>,
        gemstones: Vec<String>,
        fuming: i16,
        art_of_peace: i16,
        reforge: String,
        star: i16,
    ) -> Armor {
        Self {
            name,
            armor_type,
            enchants,
            gemstones,
            fuming,
            art_of_peace,
            reforge,
            star,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuctionListing {
    uuid: String,
    auctioneer: String,
    profile_id: String,
    coop: Value,
    start: usize,
    end: usize,
    item_name: String,
    item_lore: String,
    extra: String,
    categories: Value,
    category: String,
    tier: String,
    starting_bid: usize,
    item_bytes: String,
    claimed: bool,
    claimed_bidders: Vec<String>,
    highest_bid_amount: usize,
    last_updated: usize,
    bin: bool,
    bids: Vec<Value>,
    item_uuid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HypixelAuction {
    success: bool,
    page: usize,
    totalPages: usize,
    totalAuctions: usize,
    lastUpdated: usize,
    pub auctions: Vec<AuctionListing>,
}
