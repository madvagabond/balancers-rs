use super::*;

use std::sync::*;
use std::convert::From;


pub struct RoundRobin<T: Clone + PartialEq> {
  pub inner: Arc< RwLock< Vec<T> >>,
  pub index: AtomicUsize
}











impl  <T: Clone + PartialEq> RoundRobin<T> {

  fn index(&self) -> usize {
    self.index.load(Ordering::SeqCst)
  }


  

  fn next(&self) {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let val = &self.index;
    let inner = self.inner.read().unwrap();
    
    let mut old = val.load(Ordering::Relaxed);

    
    loop {
      let new = (old + 1) % inner.len();
      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => break,
        Err(x) => old = x,
      }
    }
  }




  
  fn pick(&self) -> T {


    let inner = self.inner.read().unwrap();
    let i = self.index();

    let n = inner[i].clone();
    self.next();
    n
  }



  
  
  pub fn dispatch<F, RV>(&self, fun: F) -> RV
  where
    RV: Future,
    F: Fn(&T) -> RV
  {
    fun( &self.pick() )
  }



  
}
