use super::ZoneMapSource;

pub struct ZoneMap {
    pub source: ZoneMapSource,
}

impl ZoneMap {
    pub fn new(source: ZoneMapSource) -> Self {
        Self { source }
    }
}
