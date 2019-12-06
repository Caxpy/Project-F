use super::geometry::Point;

enum Button {
  Left,
  Middle,
  Right,
}

pub struct Mouse {
  button: Button,
  point: Point,
}

pub struct Key {}

mod handler {
  trait Mouse {
    fn on_press(&self, _: &event::Mouse) -> bool;
    fn on_release(&self, _: &event::Mouse) -> bool;
    fn on_scroll(&self, _: &event::Mouse) -> bool;
    fn on_enter(&self, _: &event::Mouse) -> bool;
    fn on_exit(&self, _: &event::Mouse) -> bool;
  }

  trait Key {
    fn on_press(&self, _: &event::Key) -> bool;
    fn on_release(&self, _: &event::Key) -> bool;
  }
}
