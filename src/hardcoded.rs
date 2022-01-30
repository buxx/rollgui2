use std::collections::HashMap;

use crate::entity;

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
        ("SAND", 0, 0, 1),
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

pub fn builds() -> Vec<entity::build::Build> {
    let mut builds = vec![];
    let mut id = 0;
    for coordinates in [
        (15, 8),
        (15, 7),
        (15, 6),
        (15, 5),
        (14, 5),
        (13, 5),
        (11, 5),
        (10, 5),
        (10, 6),
        (10, 7),
        (10, 8),
        (10, 9),
        (11, 9),
        (12, 9),
        (13, 9),
        (14, 9),
        (15, 9),
        (14, 9),
        (13, 9),
        (12, 9),
        (11, 9),
    ]
    .iter()
    {
        id += 1;
        builds.push(entity::build::Build {
            id: id,
            build_id: "STONE_WALL".to_string(),
            row_i: coordinates.0,
            col_i: coordinates.1,
            classes: vec![],
            traversable: HashMap::new(),
            is_floor: false,
        });
    }

    builds.push(entity::build::Build {
        id: id,
        build_id: "DOOR".to_string(),
        row_i: 12,
        col_i: 5,
        classes: vec![],
        traversable: HashMap::new(),
        is_floor: false,
    });

    for coordinates in [
        (11, 7),
        (12, 7),
        (13, 7),
        (14, 7),
        (11, 6),
        (12, 6),
        (13, 6),
        (14, 6),
        (11, 8),
        (12, 8),
        (13, 8),
        (14, 8),
    ]
    .iter()
    {
        id += 1;
        builds.push(entity::build::Build {
            id: id,
            build_id: "RAW_CLAY_FLOOR".to_string(),
            row_i: coordinates.0,
            col_i: coordinates.1,
            classes: vec![],
            traversable: HashMap::new(),
            is_floor: true,
        });
    }

    builds
}
