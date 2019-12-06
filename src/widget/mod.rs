trait Event {}

trait Handler<T: Event> {}

trait Dispatcher<T: Event> {
  fn set_handler(handler: &dyn Handler<T>);
  fn get_handler(handler: &dyn Handler<T>);
  fn dispatch()
}
