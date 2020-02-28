use super::*;



pub trait Metric {
  fn new() -> Self;
  fn load(&self) -> i64;
  
  fn start(&self) -> i64;
  fn end(&self, ts: i64);
}









fn to_ns(d: &Duration) -> f64 {
  let nanos = d.subsec_nanos() as f64;
  let secs =  ( (1000*1000*1000) * d.as_secs() ) as f64;
  secs + nanos
}




mod ewma {


  use super::*;


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
      let nanos = time.subsec_nanos() as i64;
      let secs = ( 1000*1000*1000 * time.as_secs() ) as i64;
      secs + nanos  
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

      let stamp = to_ns( &clock.elapsed() ) as i64;
      let cost = 0;

      let pending = 0;
      Inner{penalty, tau, clock, stamp, cost, pending}
    }


  }









  

  pub struct EWMA {
    inner: Arc< RwLock<Inner> >
  }




  impl Metric for EWMA {

    fn new() -> Self {
      let i = Inner::new();
      let locked = RwLock::new(i);
      EWMA { inner: Arc::new(locked) }
    }


    
    fn start(&self) -> i64 {
      let mut inner = self.inner.write().unwrap();
      inner.pending += 1;
      inner.elapsed_ns()
    }


    

    
    fn end(&self, ts: i64) {

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


    
    fn load(&self) -> i64 {
      let inner = self.inner.read().unwrap();

      
      if inner.cost == 0 && inner.pending != 0 {
        (inner.penalty as i64) + (inner.pending as i64)
      }

      else {
        inner.cost * (inner.pending as i64 + 1)
      }

    }
    

 
  }


  impl Clone for EWMA {

    fn clone(&self) -> EWMA {
      EWMA {inner:self.inner.clone()}
    } 


  }

  
  
}









pub struct Counter {
  inner: AtomicUsize
}




impl Metric for Counter {

  fn new() -> Self {
    let inner = AtomicUsize::new(0);
    Counter{inner}
  }

  fn load(&self) -> i64 {
    self.inner.load(Ordering::SeqCst)  as i64
  }


  fn start(&self) -> i64 {
    self.inner.fetch_add(1, Ordering::SeqCst);
    self.inner.load(Ordering::SeqCst)  as i64
  }


  fn end(&self, _rtt: i64) {
    self.inner.fetch_sub(1, Ordering::SeqCst);
  }

    

}



pub use self::ewma::{EWMA};
