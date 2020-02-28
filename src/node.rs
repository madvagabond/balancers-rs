use std::hash::*;
use metrohash::*;
use crate::load::*; 
use std::sync::*; 

pub trait Node {
  fn id(&self) -> u64; 
}




impl <T> Node for T where T: Hash {


  fn id(&self) -> u64 {
    let mut hasher = MetroHash::default();
    self.hash(&mut hasher);
    hasher.finish() 
  }

  
}


#[derive(Clone)]
pub struct WeightedNode<T, L> {
  value: Arc<T>,
  load: L 
}



impl <T: Node, L> Node for WeightedNode<T, L> {
  fn id(&self) -> u64 {
    self.value.id() 
  }
}






impl <T, L> WeightedNode<T, L>  where L: Load {

  
  pub fn load(&self) -> &L {
    &self.load
  }

  pub fn value(&self) -> &T {
    &self.value
  }



  pub fn new(value: T) -> Self {
    WeightedNode {value: Arc::new(value), load: L::new()}
  }


  
}






pub type PeakEWMA<T> = WeightedNode<T, EWMA>;
pub type LeastLoaded<T> = WeightedNode<T, Counter>;




impl <T> WeightedNode <T, Counter> {
  pub fn start(&self) {
    self.load().incr();
  }

  pub fn end (&self) {
    self.load().decr();
  }
}




impl <T> WeightedNode <T, EWMA> {

  pub fn start(&self) -> i64 {
    self.load().start()
  }


  pub fn end(&self, begin: i64) {
    self.load().end(begin)
  }


  
}
