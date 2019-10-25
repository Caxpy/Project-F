#![allow(unused)]
#![allow(incomplete_features)]
#![feature(const_generics)]

mod graphics;
use graphics::Pixel;

fn main() {
  println!("Pixel::new -> {:?}", Pixel::new());
  println!("Pixel::rgba -> {:?}", Pixel::rgba(255, 128, 64, 32));
  println!("Pixel::hsva {:?}", Pixel::hsva(0.9, 0.4, 0.5, 0.9));
}
