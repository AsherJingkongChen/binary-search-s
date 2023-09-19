#![no_std]

pub trait BinarySearchable<T> {
  type Index;
  // fn lower_bound_s(&self, value: &T)
  //   -> Option<Self::Index>;
  fn binary_search_s(&self, value: &T)
    -> Result<Self::Index, Self::Index>;
}

impl<T> BinarySearchable<T> for [T]
where
  T: Ord,
{
  type Index = usize;

  // fn lower_bound_s(&self, value: &T)
  //   -> Option<Self::Index>
  // {
  //   let mut pivot = 0;
  //   let mut length = self.len();
  //   while length != 0 {
  //     let padding = length & 1;
  //     length >>= 1;
  //     if unsafe {
  //       self.get_unchecked(pivot + length) < value
  //     } {
  //       pivot += length + padding;
  //     }
  //   }
  //   if pivot == self.len() {
  //     None
  //   } else {
  //     Some(pivot)
  //   }
  // }

  fn binary_search_s(&self, value: &T)
    -> Result<Self::Index, Self::Index>
  {
    let mut left = 0;
    let mut size = self.len();
    while size > 0 {
      let half_size = size >> 1;
      let cmp_value = unsafe {
        self.get_unchecked(left + half_size)
      };
      if cmp_value < value {
        left += size - half_size;
      } else if !(cmp_value > value) {
        return Ok(left + half_size);
      }
      size = half_size;
    }
    Err(left)
  }
}
