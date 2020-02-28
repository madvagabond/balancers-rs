use std::sync::*; 







pub trait Load: Clone {
  type Metric: Ord; 
  fn load(&self) -> Self::Metric;
  fn new() -> Self; 
}







mod ewma {

  use super::*;  
  use std::time::{Instant};
  use std::sync::{Arc, RwLock};
  
  


  
  #[derive(Clone, Debug)]
  pub struct EWMA {
    inner: Arc< RwLock<Inner> >
  }



  impl EWMA {
    
    pub fn new() -> Self {
      let i = Inner::new();
      let locked = RwLock::new(i);
      EWMA { inner: Arc::new(locked) }
    }


    
    pub fn start(&self) -> i64 {
      let mut inner = self.inner.write().unwrap();
      inner.pending += 1;
      inner.elapsed_ns() as i64
    }


    

    
    pub fn end(&self, ts: i64) {

      let mut inner = self.inner.write().unwrap(); 
      let tm = inner.elapsed_ns() - ts; 


      let rtt = if tm > 0 {
        tm
      } else {
        0
      };


      inner.pending -= 1;
      inner.update(rtt)
    }

    
  }



  
  impl Load for EWMA {
    type Metric = i64;
   
    fn load(&self) -> i64 {
      let inner = self.inner.read().unwrap();

      
      if inner.cost == 0 && inner.pending != 0 {
        (inner.penalty as i64) + (inner.pending as i64)
      }

      else {
        inner.cost * (inner.pending as i64 + 1)
      }

    }


    fn new() -> Self {
     Self::new()
    }

    
  }







  
  
  
  #[derive(Clone, Debug)]
  struct Inner {

    clock: Instant, 
    penalty: f64,
    tau: i64,
    
    stamp: i64,
    cost: i64,
    pending: usize
  }





  



  
  impl Inner {


    
    fn elapsed_ns(&mut self) -> i64 {
      let time = self.clock.elapsed();
      time.as_nanos() as i64
    }


    


    


    fn update(&mut self, rtt: i64) {

      let t = self.elapsed_ns();

      let tr = t - self.stamp;
      let zero = 0;

      let td = if tr >= zero {
        tr
      }  else {
        zero
      };

      

      let w = ( (-td / self.tau) as f64).exp() as i64;

      if rtt < self.cost {
        self.cost = rtt
      } else {
        self.cost = self.cost * w + rtt * (1 - w)
      }
      
      self.stamp = t;
      
    }




    fn new() -> Self {
      let penalty = ( (i32::max_value() as f64) / 2.0) as f64;
      let tau = 1000*1000*1000 * 15 as i64;
      let clock = Instant::now();

      let stamp = clock.elapsed().as_nanos() as i64;
      let cost = 0;

      let pending = 0;
      Inner{penalty, tau, clock, stamp, cost, pending}
    }


  }




  

}



pub use self::ewma::EWMA;

use std::sync::atomic::{AtomicUsize, Ordering};


#[derive(Debug, Clone, Default)]
pub struct Counter {
  inner: Arc<AtomicUsize>
}



impl Counter {
  
  pub fn incr(&self) -> usize {
    self.inner.fetch_add(1, Ordering::Relaxed)
  }


  pub fn decr(&self) -> usize {
    self.inner.fetch_sub(1, Ordering::Relaxed)
  }


  
}



impl Load for Counter {

  type Metric = usize;


  fn load(&self) -> usize {
    self.inner.load(Ordering::SeqCst)
  }

  fn new() -> Self {
    Self::default()
  }
  
}





impl Load for () {
  type Metric = usize;

  fn load(&self) -> usize {
    0
  }


  fn new() -> Self {
    Self::default()
  }

  
}
