use super::map;
use crate::zone;

pub fn from_txt_map(_source: &str) -> Result<map::ZoneMap, String> {
    Ok(zone::map::ZoneMap::new(vec![]))
}
