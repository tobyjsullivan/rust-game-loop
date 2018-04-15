use component::{
  ComponentManager,
  MapTile,
  Camera
}

pub struct MapGeneration {}

impl MapGeneration {
  pub fn new() -> Self {
    MapGeneration{}
  }

  pub fn apply(&self, map_tiles: ComponentManager<MapTile>, cameras: &ComponentManager<Camera>) -> (ComponentManager<MapTile>) {
    // TODO Determine all the coordinates currently visible by cameras

    // TODO Filter down to all coordinates which do not currently have a map tile

    // TODO Generate map tiles for all coordinates which are currently lacking a tile
  }
}
