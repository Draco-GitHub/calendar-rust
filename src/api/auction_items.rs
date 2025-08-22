use serde::{Deserialize, Serialize};

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
        Pet { name, level, exp, candy }
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
    pub(crate) fn new (name: String, armor_type: ArmorType, enchants: Vec<String>, gemstones:Vec<String>, fuming:i16, art_of_peace:i16, reforge: String, star:i16) -> Armor {
        Self {name, armor_type, enchants, gemstones, fuming, art_of_peace, reforge, star}
    }
}