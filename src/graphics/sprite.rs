use super::Pixel;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Sprite<const N: usize, const M: usize>(pub [[Pixel; N]; M]);

// impl Sprite<const N: usize, const M: usize> {
//   pub fn super_scale<const K: usize>() {
//     println!("blah");
//   }
//   pub fn super_scale<const K: usize>() {
//     println!("blah");
//   }
// }
