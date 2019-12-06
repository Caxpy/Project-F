use super::sprite::Sprite;
use super::Color;

pub trait Display {
  /// Draws point with color on display
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
  fn draw_point(&self, location: (i32, i32), trace: Color);
  fn draw_line(&self, location: ((i32, i32), (i32, i32)), trace: Color);
  fn draw_triangle(&self, location: ((i32, i32), (i32, i32), (i32, i32)), trace: Color);
  fn draw_rectangle(&self, location: ((i32, i32), (i32, i32)), trace: Color);
  fn draw_ellipse(&self);
  fn draw_round_rectangle(&self);
  fn draw_polygon(&self);

  fn draw_sprite<const N: usize, const M: usize>(&self, sprite: &Sprite);

  fn play_sound(&self);
}
