

extern crate rand;

use std::time::*;
use std::borrow::Borrow;

use std::sync::*;
use std::sync::atomic::*;
use std::cmp::max;



pub mod ring;
pub mod p2c;
pub mod round_robin;
pub mod metric;




pub trait Sharder {
  type Item;
  type Key;

  fn pick(&self, key: Self::Key) -> Option<Self::Item>;
  fn pick_n(&self, key: Self::Key, n: usize) -> Vec<Self::Item>; 
}





pub trait Distributor {
  type Item;
  fn pick(&self) -> Option<Self::Item>;
}



pub trait CompletableDistributor {
  type Item;
  fn pick(&self) -> Option<Self::Item>;
  fn done(&self, item: Self::Item);
}




pub struct Loaded<T> {
  pub inner: T,
  pub load: AtomicUsize
}




impl<T> Loaded<T> {

  pub fn new(inner: T) -> Loaded<T> {
    let load = AtomicUsize::new(0);
    Loaded{load, inner}
  }

  pub fn incr(&self) {
    self.load.fetch_add(1, Ordering::SeqCst); 
  }

  pub fn load(&self) -> usize {
    self.load.load(Ordering::SeqCst)
  }


  pub fn decr(&self) {
    self.load.fetch_sub(1, Ordering::SeqCst);
  }

  
 

  
}



impl<T> Borrow<T> for Loaded<T> {
  fn borrow(&self) -> &T {
    &self.inner
  }
}




impl<T: Clone> Clone for Loaded<T> {
  fn clone(&self) -> Self {
    let load = AtomicUsize::new( self.load() );
    Loaded {inner: self.inner.clone(), load}
  }
}











#[cfg(test)]
mod tests {


  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }


  
}
