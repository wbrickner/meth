use mouse_rs::types::Point;

/// Forms a state machine that applies transformations
/// to mouse position in a cycle.
pub enum Wiggler {
  Up,
  Down,
  Left,
  Right
}

impl Default for Wiggler {
  fn default() -> Wiggler { Wiggler::Up }
}

impl Wiggler {
  /// 5px displacement
  const WIGGLE_DISPLACEMENT: i32 = 5;

  ///```
  ///     RIGHT       
  ///   ▲───────▶     
  ///   │       │     
  /// UP│       │DOWN 
  ///   │       │     
  ///   ◀───────▼     
  ///     LEFT        
  ///```
  fn next(&self) -> Wiggler {
    match self {
      Wiggler::Up    => Wiggler::Right,
      Wiggler::Right => Wiggler::Down,
      Wiggler::Down  => Wiggler::Left,
      Wiggler::Left  => Wiggler::Up
    }
  }

  pub fn transform(&mut self, point: &mut Point) {
    match self {
      Wiggler::Up    => point.y -= Self::WIGGLE_DISPLACEMENT,
      Wiggler::Right => point.x += Self::WIGGLE_DISPLACEMENT,
      Wiggler::Down  => point.y += Self::WIGGLE_DISPLACEMENT,
      Wiggler::Left  => point.x -= Self::WIGGLE_DISPLACEMENT
    }

    *self = self.next();
  }
}