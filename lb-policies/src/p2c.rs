use super::*;

use rand::prelude::*;
use std::sync::*;
use std::collections::HashMap; 


type MemberList<T> = Arc< RwLock< Vec< Loaded<T> >  > >;





pub struct P2C<T: Clone + PartialEq> {
  inner: MemberList<T>
}






impl <T: Clone + PartialEq> CompletableDistributor for P2C<T> {
  type Item = T;


  
  fn pick(&self) -> Option<Self::Item> {

    let mut rng = thread_rng();
    let nodes = self.inner.read().unwrap();
    
    let (i1, i2) = (
      rng.gen_range(0, nodes.len()), rng.gen_range(0, nodes.len())
    );

    let (a, b) = (nodes.get(i1).unwrap(), nodes.get(i2).unwrap() );


    if a.load.load(Ordering::SeqCst) <= b.load.load(Ordering::SeqCst)  {
      a.incr();
      Some( a.inner.to_owned() )
    }

    else {
      b.incr();
      Some(b.inner.to_owned() )
    }


  
  }




  fn done(&self, item: Self::Item) {
    let inner = self.inner.read().unwrap();

    if let Some(x) = inner.iter().find(|x| x.inner == item) {
      x.decr()
    }
  }


  
}


impl <T: Clone + PartialEq> From <Vec<T>> for P2C<T> {
  fn from(v: Vec<T>) -> P2C<T> {

    let loaded = v.into_iter().map(|t| Loaded::new(t) ).collect();
    let locked = RwLock::new(loaded);
    let inner = Arc::new(locked);
    P2C {inner} 
  }

  
}




impl<T: Clone + PartialEq > P2C<T> {


  pub fn new() -> P2C<T> {
    Self::from(vec![])
  }


  pub fn add_node(&mut self, node: T) {
    let mut nodes = self.inner.write().unwrap();
    let loaded = Loaded::new(node);
    nodes.push(loaded);
  }


  pub fn remove_node(&mut self, node: T) {
    let mut nodes = self.inner.write().unwrap() ;
    let result = nodes.iter().position(|x| x.inner == node);
    
    match result {
      Some(i) => Some( nodes.remove(i) ),
      None => None
    };
    
  }


  pub fn add_nodes(&mut self, hosts: Vec<T>) {
    let mut inner = self.inner.write().unwrap();
    let mut add = hosts.into_iter().map(|x| Loaded::new(x) ).collect();
    inner.append(&mut add)
  }


  pub fn remove_nodes(&self, hosts: Vec<T>) {
    let mut nodes = self.inner.write().unwrap();
    nodes.retain(|n| hosts.contains(&n.inner) == false );
  }

  
}

/*
pub struct P2C_EWMA<T: Clone + PartialEq> {
  inner: Arc< RwLock< Vec<T>>>,
  metrics: Arc<Metrics>
}

*/
