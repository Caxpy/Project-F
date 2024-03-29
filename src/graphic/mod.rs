pub mod display;
pub mod sprite;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Color {
  a: u8,
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  /// Constructs Color from byte values for each color plane
  ///
  /// # Arguments
  ///
  /// * 'a' - Alpha channel
  /// * 'r' - Red channel
  /// * 'g' - Green channel
  /// * 'b' - Blue channel
  ///
  /// # Remarks
  ///
  /// Alpha channel is in non-traditional u8 format
  pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
    Color {
      a: a,
      r: r,
      g: g,
      b: b,
    }
  }

  /// Constructs Color with maximum default alpha channel (opaque)
  ///
  /// # Arguments
  ///
  /// * 'r' - Red channel
  /// * 'g' - Green channel
  /// * 'b' - Blue channel
  ///
  /// # Remarks
  ///
  /// None
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Color::new(0xFF, r, g, b)
  }

  /// Decomposes hex in format 0x __-RR-GG-BB into Color
  ///
  /// # Arguments
  ///
  /// * 'hex' - Code to decompose
  ///
  /// # Remarks
  ///
  /// The most significant byte of a 4 byte word is ignored and preset to 255.
  /// In other words, the alpha channel is always set to maximum (opaque).
  pub const fn hex(hex: u32) -> Self {
    let planes = hex.to_be_bytes();
    Color::new(0xFF, planes[1], planes[2], planes[3])
  }

  /// Converts HSV (Hue, Saturation, Value, Alpha) to Color value
  ///
  /// # Arguments
  ///
  /// * 'h' - Hue controls the color tone. [0.0 - 1.0]
  /// * 's' - Saturation controls intensity of color [0.0 - 1.0]
  /// * 'v' - Value controls greyscale [0.0 - 1.0]
  ///
  /// # Remarks
  ///
  /// https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
  /// Since Color stores as u8, HSV conversion is destructive; that is,
  /// converting from HSV to Color, then back to HSV will not yield exactly
  /// equal results.
  pub fn hsv(h: f32, s: f32, v: f32) -> Self {
    assert!((0.0..1.0).contains(&h));
    assert!((0.0..1.0).contains(&s));
    assert!((0.0..1.0).contains(&v));

    let region = (6.0 * h) as i32;

    let c = v * s;
    let x = c * (1.0 - ((6.0 * h) % 2.0 - 1.0).abs());
    let m = v - c;

    // calculate preoffset color planes by region
    let (r_prime, g_prime, b_prime) = match (6.0 * h) as i32 {
      0 => (c, x, 0.0),
      1 => (x, c, 0.0),
      2 => (0.0, c, x),
      3 => (0.0, x, c),
      4 => (x, 0.0, c),
      5 => (c, 0.0, x),
      _ => (0.0, 0.0, 0.0),
    };

    // converts range from 0.0 -> 1.0 to 0 -> 255
    let range_to_u8 = |n: f32| (u8::max_value() as f32 * n).round() as u8;
    Color::new(
      0xFF,
      range_to_u8(r_prime + m),
      range_to_u8(g_prime + m),
      range_to_u8(b_prime + m),
    )
  }

  /// Builds Color with alpha channel
  ///
  /// # Arguments
  ///
  /// * 'self' - Color to build from
  /// * 'a' - Alpha channel
  ///
  /// # Remarks
  ///
  /// None
  pub fn with_alpha(mut self, a: f32) -> Self {
    assert!((0.0..1.0).contains(&a));
    self.a = (u8::max_value() as f32 * a).round() as u8;
    self
  }
}

mod test {
  use super::*;

  #[test]
  fn test_constructors() {
    assert_eq!(Color::default(), Color::new(0, 0, 0, 0));
    assert_eq!(Color::rgb(255, 128, 64), Color::new(255, 255, 128, 64));
    assert_eq!(Color::hex(0xFA768CB0), Color::new(255, 118, 140, 176));
    assert_eq!(Color::hsv(0.9, 0.4, 0.5), Color::new(255, 128, 77, 107));
    assert_eq!(
      Color::rgb(255, 128, 64).with_alpha(0.5),
      Color::new(128, 255, 128, 64)
    );
  }
}
