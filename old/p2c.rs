use super::*;

use rand::prelude::*;



pub type MemberList<T> = Arc< RwLock< Vec<T>  >>;


pub struct Dispatch<W: WeightedNode, F: Future> {
  load: i64,
  node: W, 
  future: F 
}


impl <W: WeightedNode, F: Future> Future for Dispatch<W, F> {
  type Item = F::Item;
  type Error = F::Error;

  
  fn poll(&mut self) ->  Result<Async<Self::Item>, Self::Error> {

    match self.future.poll() {
      Ok(Async::Ready(t)) => {
        self.node.end(self.load);
        Ok( Async::Ready(t) )
      },

      Ok(Async::NotReady) => Ok(Async::NotReady), 

      Err(e) => {
        self.node.end(self.load);
        Err(e)
      }
    }
  }
  
}





impl<W: WeightedNode, F: Future>  Dispatch<W, F> {
  pub fn make(node: W, future: F) -> Self {
    Dispatch {load: node.start(), future, node}
  }
} 




pub struct P2C<W: WeightedNode + Clone> {
  pub inner: MemberList<W>
}







impl <W: WeightedNode + Clone> From <Vec<W::Item>> for P2C<W> {
  
  fn from(v: Vec<W::Item>) -> P2C<W> {

    let loaded = v.into_iter().map(|inner| {
     W::new(inner)
    }).collect();
    
    let locked = RwLock::new(loaded);
    let inner = Arc::new(locked);
    P2C {inner} 
  }

  
}





impl<W: WeightedNode + Clone > P2C<W> {



  fn dispatch<F, RV>(&self, fun: F) -> Dispatch<W, RV> 
  where
    RV: Future + Sized,
    F: Fn(&W::Item) -> RV {
    let nodes = self.inner.read().unwrap();
    let (a, b) = (
      rand::thread_rng().choose(&nodes).unwrap(),
      rand::thread_rng().choose(&nodes).unwrap()
    );

    
    if a.load() >= b.load() {
      let f = a.inner();
      Dispatch::make(a.clone(), fun(&f) )
    }

    else {
      
      let f = b.inner();
      Dispatch::make(b.clone(), fun(&f) )
    }
    
 

  }




  
}



type P2CLeastLoaded<T> = P2C<EwmaNode<T>>;

type P2CPeakEwma<T> = P2C<CountedNode<T>>;



