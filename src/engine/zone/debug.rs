pub struct DebugInfo {
    tile_count: i32,
    resource_count: i32,
    stuff_count: i32,
    character_count: i32,
    build_count: i32,
    fps: i32,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            tile_count: 0,
            resource_count: 0,
            stuff_count: 0,
            character_count: 0,
            build_count: 0,
            fps: 0,
        }
    }

    pub fn incr_zone_tile_count(&mut self) {
        self.tile_count += 1;
    }

    pub fn incr_resource_count(&mut self) {
        self.resource_count += 1;
    }

    pub fn incr_stuff_count(&mut self) {
        self.stuff_count += 1;
    }

    pub fn incr_character_count(&mut self) {
        self.character_count += 1;
    }

    pub fn incr_build_count(&mut self) {
        self.build_count += 1;
    }

    pub fn set_fps(&mut self, fps: i32) {
        self.fps = fps
    }

    pub fn reset(&mut self) {
        self.tile_count = 0;
        self.resource_count = 0;
        self.stuff_count = 0;
        self.character_count = 0;
        self.build_count = 0;
        self.fps = 0;
    }

    pub fn to_string(&self) -> String {
        format!(
            "FPS:{},T:{},R:{},S:{},C:{},B:{}",
            self.fps,
            self.tile_count,
            self.resource_count,
            self.stuff_count,
            self.character_count,
            self.build_count
        )
    }
}
