
extern crate futures;
extern crate rand;

use std::time::*;

use std::sync::*;
use std::sync::atomic::*;
use futures::prelude::*;
use std::hash::{Hash, Hasher};

mod server_sets;
pub mod ring;
pub mod p2c;
pub mod round_robin;
pub mod metric;

use metric::*;


pub trait WeightedNode {
  type Item: PartialEq + Clone; 

  fn new(inner: Self::Item) -> Self;


  fn inner(&self) -> Self::Item; 
  fn load(&self) -> i64;
  
  fn start(&self) -> i64;
  fn end(&self, ts: i64);
}



trait Sampler {
  type Item; 
  fn pick(&self) -> Option<Self::Item>;
}



trait Sharder {

  type Item;
  type Key;

  
  fn pick(&self, key: Self::Key) -> Option<Self::Item>;
  fn replicas(&self, key: Self::Key) -> Vec<Self::Item>; 
}




pub struct Loaded<T: Clone, M: Metric + Clone> {
  pub inner: T,
  pub metric: M
}




impl<T: Clone + PartialEq, M: Metric + Clone> WeightedNode for Loaded<T, M> {

  type Item = T;
  
  fn new(inner: T) -> Self {
    let metric = M::new();
    Loaded{inner, metric}
  }

  fn start(&self) -> i64 {
    self.metric.start()
  }

  fn load(&self) -> i64 {
    self.metric.load()
  }


  fn end(&self, rtt: i64) {
    self.metric.end(rtt)
  }

  

  fn inner(&self) -> T {
    self.inner.clone()
  }

  
}







impl <T: Clone, M: Metric + Clone>  Clone for Loaded<T, M> {


  fn clone(&self) -> Self {
    Loaded{ inner: self.inner.clone(), metric:self.metric.clone() }
  }

  
}




pub type EwmaNode<T> = Loaded<T, EWMA>;
pub type CountedNode<T> = Loaded<T, Counter>;



#[cfg(test)]
mod tests {


  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }


  
}
