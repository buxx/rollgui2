use crate::zone;
pub struct ZoneState {
    pub map: zone::map::ZoneMap,
}

impl ZoneState {
    pub fn new(map: zone::map::ZoneMap) -> Self {
        Self { map }
    }
}
