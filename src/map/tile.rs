use sdl2::pixels::Color;

pub enum Tile {
  Grass,
  Water
}

impl Tile {
  pub fn color(tile: &Tile) -> Color {
    match tile {
      Tile::Grass => Color::RGB(0, 255, 0),
      Tile::Water => Color::RGB(0, 0, 255)
    }
  }
}