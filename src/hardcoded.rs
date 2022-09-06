pub fn get_tiles_list() -> Vec<(&'static str, i16, i16, i16)> {
    [
        ("UNKNOWN", 0, 0, 1),
        ("SEA", 7, 0, 1),
        ("JUNGLE", 7, 4, 1),
        ("PLAIN", 7, 2, 1),
        ("HILL", 7, 5, 1),
        ("MOUNTAIN", 7, 6, 1),
        ("BEACH", 7, 1, 1),
        ("BACK_BEACH", 0, 3, 1),
        ("BACK_PLAIN", 0, 6, 1),
        ("BACK_JUNGLE", 0, 4, 1),
        ("BACK_HILL", 0, 4, 1),
        ("BACK_MOUNTAIN", 0, 5, 1),
        ("BACK_SEA", 0, 2, 1),
        ("SAND", 0, 3, 1),
        ("DRY_BUSH", 2, 8, 1),
        ("ROCK", 2, 9, 1),
        ("SEA_WATER", 14, 0, 6),
        ("FRESH_WATER_TILE", 13, 0, 6),
        ("SHORT_GRASS", 1, 0, 1),
        ("HIGH_GRASS", 15, 0, 6),
        ("ROCKY_GROUND", 0, 0, 1),
        ("DIRT", 0, 10, 1),
        ("LEAF_TREE", 1, 7, 1),
        ("TROPICAL_TREE", 1, 8, 1),
        ("DEAD_TREE", 1, 9, 1),
        ("CHARACTER", 16, 0, 1),
        ("CHARACTER_RUNNING_TOP", 17, 0, 6),
        ("CHARACTER_RUNNING_DOWN", 18, 0, 6),
        ("CHARACTER_RUNNING_RIGHT", 19, 0, 6),
        ("CHARACTER_RUNNING_LEFT", 20, 0, 6),
        ("STONE_WALL", 4, 3, 1),
        ("DOOR", 4, 12, 1),
        ("RAW_CLAY_FLOOR", 0, 6, 1),
        ("PLAYER", 6, 0, 1),
        ("PLAYER_LEFT", 6, 1, 1),
        ("STUFF_GENERIC", 3, 0, 1),
        ("BOTTLE", 3, 1, 1),
        ("BAG", 3, 2, 1),
        ("COAT", 3, 3, 1),
        ("ARMOR", 3, 4, 1),
        ("WEAPON", 3, 6, 1),
        ("CORPSE", 3, 11, 1),
        ("ANIMAL", 3, 9, 1),
        ("CRAFT", 3, 10, 1),
        ("RESOURCE_GENERIC", 5, 0, 1),
        ("COPPER_DEPOSIT", 0, 7, 1),
        ("TIN_DEPOSIT", 0, 8, 1),
        ("IRON_DEPOSIT", 0, 9, 1),
        ("FRESH_WATER", 5, 10, 1),
        ("SALTED_WATER", 5, 10, 1),
        ("BEACH_SAND", 5, 11, 1),
        ("SOIL", 5, 9, 1),
        ("WET_SOIL", 5, 9, 1),
        ("WOOD", 5, 6, 1),
        ("VEGETAL_FOOD_FRESH", 5, 3, 1),
        ("SHELLFISH_FRESH", 5, 1, 1),
        ("FISH_FRESH", 5, 1, 1),
        ("RAW_MEAT", 5, 4, 1),
        ("COOKED_MEAT", 5, 5, 1),
        ("SMOKED_MEAT", 5, 5, 1),
        ("ANIMAL_SKIN", 5, 8, 1),
        ("GRAMINEAE", 5, 13, 1),
        ("BREAD", 5, 14, 1),
        ("RAW_STONE", 5, 12, 1),
        ("LEATHER_PIECE", 5, 7, 1),
        ("BUILD_GENERIC", 4, 1, 1),
        ("CAMPFIRE__OFF", 9, 0, 1),
        ("CAMPFIRE", 9, 1, 6),
        ("CAMP_FIRE__OFF", 9, 0, 1),
        ("CAMP_FIRE", 9, 1, 6),
        ("WALL", 4, 2, 1),
        ("WOOD_FENCE", 4, 2, 1),
        ("LOOM", 4, 6, 1),
        ("BRUSHWOOD_EDGE", 4, 4, 1),
        ("SOIL_WALL", 4, 5, 1),
        ("BASKETRY_BAG", 3, 12, 1),
        ("SKIN_BAG", 3, 14, 1),
        ("LEATHER_BAG", 3, 13, 1),
        ("TRAVOIS", 3, 20, 1),
        ("CLOTH_BAG", 3, 15, 1),
        ("ANIMAL_SKIN_CLOTHES", 3, 17, 1),
        ("LEATHER_CLOTHES", 3, 16, 1),
        ("LEATHER_BRIGANDINE", 3, 19, 1),
        ("HARE", 3, 26, 1),
        ("PIG", 3, 25, 1),
        ("GOAT", 3, 24, 1),
        ("MOORHEN", 3, 23, 1),
        ("CRAB", 3, 22, 1),
        ("RAW_BRICK", 3, 27, 1),
        ("FIRED_BRICK", 3, 28, 1),
        ("RAW_BRICK_WALL", 4, 7, 1),
        ("FIRED_BRICK_WALL", 4, 8, 1),
        ("TOTEM", 4, 9, 1),
        ("COMPASS", 4, 10, 1),
        ("SOIL_KILN__OFF", 10, 0, 1),
        ("SOIL_KILN", 10, 1, 2),
        ("RAW_BRICK_KILN__OFF", 11, 0, 1),
        ("RAW_BRICK_KILN", 11, 1, 2),
        ("FIRED_BRICK_KILN__OFF", 12, 0, 1),
        ("FIRED_BRICK_KILN", 12, 1, 2),
        ("RAW_COPPER", 5, 15, 1),
        ("RAW_TIN", 5, 16, 1),
        ("RAW_IRON", 5, 17, 1),
        ("COPPER", 5, 18, 1),
        ("TIN", 5, 19, 1),
        ("IRON", 5, 20, 1),
        ("BRONZE", 4, 18, 1),
        ("VEGETAL_FIBER", 5, 21, 1),
        ("CLOTH", 5, 22, 1),
        ("GROUND", 0, 10, 1),
        ("PLOUGHED_LAND", 0, 11, 1),
        ("SEEDS", 1, 10, 1),
        ("CEREAL", 5, 13, 1),
        ("GROW_PROGRESS_0", 8, 0, 1),
        ("GROW_PROGRESS_1", 8, 1, 1),
        ("GROW_PROGRESS_2", 8, 2, 1),
        ("GROW_PROGRESS_3", 8, 3, 1),
        ("GROW_PROGRESS_4", 8, 4, 1),
        ("GROW_PROGRESS_CEREAL_0", 8, 0, 1),
        ("GROW_PROGRESS_CEREAL_1", 8, 1, 1),
        ("GROW_PROGRESS_CEREAL_2", 8, 2, 1),
        ("GROW_PROGRESS_CEREAL_3", 8, 3, 1),
        ("GROW_PROGRESS_CEREAL_4", 8, 4, 1),
        ("FLOOR", 5, 23, 1),
        ("BREAD", 5, 24, 1),
        ("WOOL", 5, 25, 1),
        ("MORTIER_PILON", 5, 26, 1),
        ("CHARCOAL", 5, 27, 1),
        ("ROUET", 5, 28, 1),
        ("SPINDLE", 3, 29, 6),
        ("LITTLE_FISHING_NET", 5, 29, 1),
        ("STONE_ANVIL", 5, 30, 1),
        ("IRON_ANVIL", 5, 31, 1),
        ("COLLECT", 2, 10, 1),
        ("HARVEST", 2, 11, 1),
        ("ON", 2, 12, 1),
        ("OFF", 2, 13, 1),
        ("LOOK", 2, 18, 1),
        ("DRINK", 2, 14, 1),
        ("TAKE", 2, 16, 1),
        ("BOW", 3, 30, 1),
        ("RAW_CLAY_JAR", 5, 32, 1),
        ("CLAY_JAR", 5, 33, 1),
        ("CLAY", 2, 15, 1),
        ("SPEAR", 7, 14, 1),
        ("COPPER_SPEAR", 7, 15, 1),
        ("BRONZE_SPEAR", 7, 16, 1),
        ("IRON_SPEAR", 7, 17, 1),
        ("DAGGER", 8, 14, 1),
        ("COPPER_DAGGER", 8, 15, 1),
        ("BRONZE_DAGGER", 8, 16, 1),
        ("IRON_DAGGER", 8, 17, 1),
        ("HAXE", 9, 14, 1),
        ("COPPER_HAXE", 9, 15, 1),
        ("BRONZE_HAXE", 9, 16, 1),
        ("IRON_HAXE", 9, 17, 1),
        ("MACE", 6, 13, 1),
        ("SHIELD", 10, 13, 1),
        ("PRIMITIVE_WOOD_SHIELD", 10, 13, 1),
        ("LEATHER_SHIELD", 10, 14, 1),
        ("COPPER_SHIELD", 10, 16, 1),
        ("BRONZE_SHIELD", 10, 17, 1),
        ("IRON_SHIELD_HEATER", 10, 18, 1),
        ("TILE_SELECTION", 6, 0, 6),
        ("TILE_HOVER", 6, 6, 1),
        ("WIP", 2, 17, 1),
        ("LITTLE_SHOVEL", 21, 0, 6),
        ("DO_BUILD_WORK", 6, 8, 1),
    ]
    .to_vec()
}

pub fn get_map_source() -> &'static str {
    "::GEO
    ⁖⁖ፆፆፆፆ⁖⁖⁖⁖ፆ߉          
   ⁖⁖ፆ⁖߉⁖⁖⁖ፆ⁖ፆፆ⁖⁖         
  ⁖ፆ؛⁖⁖⁖⁖⁖⁖ፆ⁖؛⁖⁖ፆፆ        
 ؛؛⁖ፆ⁖ፆ⁖⁖⁖⁖؛⁖⁖⁖⁖⁖ፆፆ       
⁖؛⁖ፆ⁖⁖⁖؛⁖؛؛ፆ⁖؛ፆ⁖⁖⁖ፆፆ      
߉⁖⁖؛ፆ⁖ፆፆ؛⁖ፆ⁖⁖⁖ፆ⁖⁖⁖⁖⁖⁖⁖     
ፆ⁖؛؛⁖⁖⁖ፆፆፆ؛⁖⁖⁖⁖⁖ፆ⁖ፆፆፆ⁖⁖؛    
؛⁖⁖؛؛⁖⁖⁖؛ፆ⁖ፆ⁖⁖؛⁖؛ፆፆ؛؛؛⁖ፆ߉⁖   
⁖⁖⁖⁖؛⁖⁖⁖ፆ⁖⁖⁖؛؛⁖ፆ؛⁖⁖⁖؛⁖⁖⁖⁖⁖⁖⁖  
⁖؛߉ፆፆፆ⁖⁖؛ፆ؛⁖⁖ፆ⁖ፆ⁖⁖ፆ⁖⁖⁖⁖؛ፆ⁖⁖⁖⁖⁖ 
⁖ፆ؛⁖⁖⁖⁖ፆ⁖⁖ፆፆ⁖⁖⁖ፆ؛ፆ؛⁖ፆ⁖ፆ⁖؛⁖⁖⁖؛ፆ⁖ፆ
؛ፆ⁖ፆ؛ፆ⁖߉߉؛؛⁖⁖؛ፆ⁖⁖⁖؛؛⁖ፆ⁖⁖⁖⁖⁖ፆ⁖؛⁖⁖
ፆ⁖؛ፆ⁖⁖߉⁖؛؛⁖ፆ⁖ፆ߉⁖⁖⁖⁖؛⁖ፆ⁖؛߉ፆ⁖⁖⁖ፆፆ⁖
؛ፆፆ⁖⁖⁖⁖⁖ፆ؛؛ፆ؛ፆ߉⁖ፆ⁖؛ፆ؛؛ፆ⁖؛⁖⁖⁖ፆፆፆ⁖
⁖⁖⁖⁖⁖ፆ⁖ፆፆ⁖߉ፆፆ؛⁖ፆፆ⁖⁖⁖ፆ⁖⁖؛؛⁖߉ፆ⁖⁖⁖ፆ
ፆፆ؛ፆ⁖⁖⁖⁖߉⁖⁖⁖⁖؛⁖߉ፆ⁖؛⁖⁖؛⁖؛߉⁖⁖ፆ⁖⁖⁖߉
ፆ؛ፆ؛؛ፆ⁖ፆፆፆፆ⁖߉⁖؛⁖⁖⁖⁖⁖⁖ፆፆፆ⁖؛⁖߉⁖؛⁖⁖
⁖ፆ؛ፆ⁖⁖⁖؛⁖⁖ፆ⁖؛؛߉ፆ؛⁖ፆ⁖⁖⁖ፆ؛ፆፆፆፆፆፆ؛⁖
ፆፆ⁖؛ፆ؛⁖⁖⁖ፆፆ⁖߉⁖ፆ⁖⁖⁖⁖ፆ⁖⁖ፆ؛؛ፆ؛ፆፆ⁖ፆ⁖
⁖ፆፆ؛⁖⁖⁖⁖؛ፆ⁖⁖ፆ⁖⁖؛⁖⁖⁖⁖؛⁖ፆ߉ፆ⁖⁖߉؛ፆ⁖⁖
⁖⁖⁖⁖⁖⁖߉؛⁖ፆፆ⁖⁖ፆፆ⁖؛⁖ፆፆፆ؛⁖⁖⁖⁖؛⁖⁖؛⁖⁖
؛؛߉⁖⁖؛؛⁖⁖⁖⁖⁖⁖⁖⁖ፆ؛ፆ⁖⁖⁖؛ፆፆ؛ፆ⁖⁖ፆፆ⁖⁖
⁖⁖⁖⁖ፆ؛؛ፆፆ؛ፆ⁖⁖⁖⁖ፆ؛⁖ፆ⁖⁖⁖⁖؛⁖⁖⁖⁖؛⁖ 
؛⁖⁖ፆ؛⁖⁖⁖؛⁖⁖⁖؛⁖⁖⁖ፆ؛؛؛⁖⁖ፆ⁖⁖⁖⁖⁖  
ፆ⁖؛ፆ⁖ፆ⁖؛؛ፆ⁖؛⁖⁖⁖؛ፆ؛⁖؛⁖⁖ፆ؛⁖⁖   
⁖⁖⁖⁖؛⁖⁖⁖⁖ፆ⁖⁖߉⁖ፆ⁖ፆ⁖⁖⁖⁖ፆ؛⁖    
⁖؛⁖ፆ⁖߉؛ፆ⁖؛؛⁖⁖ፆ؛؛؛⁖⁖߉ፆ⁖     
⁖⁖ፆፆ⁖⁖⁖ፆ⁖؛؛⁖⁖؛ፆ⁖⁖߉؛⁖      
 ⁖⁖⁖⁖؛߉⁖⁖⁖ፆ⁖؛ፆ؛ፆ⁖ፆ⁖       
  ؛⁖⁖߉⁖⁖؛⁖؛⁖؛⁖؛⁖⁖ፆ        
   ߉߉ፆ⁖ፆ⁖⁖ፆ⁖⁖⁖؛؛߉         
    ⁖؛⁖؛ፆ⁖⁖⁖⁖⁖؛⁖          "
}

pub fn get_default_tile_id_for_zone_type_id(zone_type_id: &str) -> Option<String> {
    match zone_type_id {
        "JUNGLE" => Some("DIRT".to_string()),
        "SEA" => Some("SALTED_WATER".to_string()),
        "MOUNTAIN" => Some("ROCKY_GROUND".to_string()),
        "HILL" => Some("DIRT".to_string()),
        "BEACH" => Some("SAND".to_string()),
        "PLAIN" => Some("DIRT".to_string()),
        _ => None,
    }
}
