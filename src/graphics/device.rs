use super::sprite::Sprite;

pub trait Device {
  fn draw_sprite<const N: usize, const M: usize>(&self, sprite: Sprite<{ N }, { M }>);
}
