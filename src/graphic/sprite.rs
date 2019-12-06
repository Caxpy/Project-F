use super::Color;
use std::ops::Index;

#[derive(Clone)]
pub struct Sprite {
  width: u16,
  height: u16,
  buffer: Vec<Color>,
}

impl Sprite {
  pub fn new(width: u16, height: u16) -> Self {
    Sprite {
      width: width,
      height: height,
      buffer: vec![Color::default(); (width * height) as usize],
    }
  }
}

impl Index<(u16, u16)> for Sprite {
  type Output = Color;
  fn index(&self, index: (u16, u16)) -> &Self::Output {
    assert!(index.0 <= self.width && index.1 <= self.height);
    &self.buffer[(index.0 * index.1 * self.width) as usize]
  }
}

mod test {
  use super::*;

  #[test]
  fn test_constructors() {
    let sprite = Sprite::new(16, 16);
    let x = sprite[(4, 4)];
    assert_eq!(1, 1);
  }
}
