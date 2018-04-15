use sdl2::pixels::Color;

#[derive(Clone)]
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

  pub fn obstruction(&self) -> bool {
    match self {
      Tile::Water => true,
      _ => false
    }
  }
}
