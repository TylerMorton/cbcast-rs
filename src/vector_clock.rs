use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ptr::eq;

// Some sort of hash table for lookups, updates, etc.
pub struct VectorClock<T: Eq + Hash> {
  id: T,
  h: HashMap<T, u32>,
}


impl<T: Copy + Eq + Hash> VectorClock<T> {
  /// Creates an empty `VectorClock`.
    ///
    /// The vector clock is initially created an id and a hashmap with a capacity of 0. 
    ///
    /// # Examples
    ///
    /// ```
    /// use cbcast::vector_clock::VectorClock;
    /// let mut vc: VectorClock<string> = VectorClock::new("process a");
    /// ```
  fn new(&mut self, id: T) -> VectorClock<T> {
    let vc = VectorClock { id, h: HashMap::new() };
    self.h.insert(vc.id, 0);
    return vc;
  }


  // fn update(&mut self, k: T, mut v: u32) -> Option<u32> {
  //   let val = *self.h.get_mut(&k).unwrap_or(&mut v);
  //   self.h.insert(k, val)
  // }
}

impl<T: Eq + Hash> PartialEq for VectorClock<T>{
  fn eq(&self, other: &VectorClock<T>) -> bool {
    self.h.iter().eq_by(other.h.iter(), |x, _y| (x.1 == x.1))
  }
  fn ne(&self, other: &VectorClock<T>) -> bool {
    !eq(self, other)
  }
}

impl<T: Eq + Hash> PartialOrd for VectorClock<T>{
  fn partial_cmp(&self, other: &VectorClock<T>) -> Option<std::cmp::Ordering> {
      if self.eq(other) {
        return Some(Ordering::Equal)
      }
      else {
        let zipped = self.h.iter().zip(other.h.iter());
        for (s, o) in zipped {
          if o.1 - s.1 > 1 {
            return Some(Ordering::Greater)
          }
        }
        return Some(Ordering::Less)
    }
}
}

impl<T: Eq + Hash> Display for VectorClock<T>{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut iter = self.h.iter();
    let initial = iter.next();
    if initial.is_none() {
      return Ok(())
    }
    write!(f, "[")?;
    write!(f, "{}", initial.unwrap().1)?;
    for i in iter {
      write!(f, ", {}", i.1)?;
    }
    write!(f, "]")?;
    Ok(())
  }
}