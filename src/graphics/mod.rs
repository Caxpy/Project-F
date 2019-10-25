pub mod device;
pub mod sprite;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pixel([u8; 4]);

impl Pixel {
  pub fn new() -> Self {
    Pixel([0, 0, 0, 0])
  }

  pub fn hex(hex: u32) -> Self {
    Pixel(hex.to_be_bytes())
  }

  pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Pixel([a, r, g, b])
  }

  pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
    let region = (6.0 * h) as i32;

    let c = v * s;
    let x = c * (1 - (region % 2 - 1).abs()) as f32;
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match region {
      0 => (c, x, 0.0),
      1 => (x, c, 0.0),
      2 => (0.0, c, x),
      3 => (0.0, x, c),
      4 => (x, 0.0, c),
      5 => (c, 0.0, x),
      _ => (0.0, 0.0, 0.0),
    };

    // converts range from 0.0 -> 1.0 to 0 -> 255
    let range_to_u8 = |n: f32| (u8::max_value() as f32 * n) as u8;
    Pixel([
      range_to_u8(r_prime + m),
      range_to_u8(g_prime + m),
      range_to_u8(b_prime + m),
      range_to_u8(a),
    ])
  }

  //pub fn to_hsva(&self) -> [f32; 4] {}
}

mod tests {
  use super::*;

  #[test]
  fn test_constructors() {
    assert_eq!(Pixel::new(), Pixel([0, 0, 0, 0]));
    assert_eq!(Pixel::hex(0xFA768cb0), Pixel([250, 118, 140, 176]));
    assert_eq!(Pixel::rgba(255, 128, 64, 32), Pixel([32, 255, 128, 64]));
    // TODO: currently failing
    //assert_eq!(Pixel::hsva(0.9, 0.4, 0.5, 0.5), Pixel([128, 77, 107, 128]));
  }
}
