extern crate rand;


use std::sync::*;
use std::collections::VecDeque;
use std::convert::From;

use std::sync::atomic::{Ordering, AtomicUsize};
use rand::prelude::*;



pub trait Sharder {
  type Item;
  type Key;

  fn pick(&mut self, key: &Self::Key) -> &Self::Item;
}



pub trait Distributor {
  type Item;
  fn pick(&mut self) -> Option<Self::Item>;
}



pub trait Completeable {
  type Item;
  fn done(&self, Self::Item);
}




pub struct RoundRobin<T: Clone + PartialEq> {
  nodes: Arc< Mutex< VecDeque<T> > >,
}




impl <T: Clone + PartialEq> From<Vec<T>> for RoundRobin<T> {

  fn from(vec: Vec<T>) -> RoundRobin<T> {
    let queue = VecDeque::from(vec);
    let nodes = Arc::new( Mutex::new(queue) );
    RoundRobin { nodes }
  }


  
}





impl<T: Clone + PartialEq> RoundRobin<T> {

  pub fn new() -> RoundRobin<T> {
    Self::from(vec![])
  }


  pub fn add_node(&mut self, host: T) {
    let mut nodes = self.nodes.lock().unwrap();
    nodes.push_back(host)
  }



  pub fn remove_node(&mut self, host: T) {
    let mut nodes = self.nodes.lock().unwrap();
    let result = nodes.iter().position(|n| n == &host);

    match result {
      Some(i) => nodes.remove(i),
      None => None
    };

   

  }


  pub fn remove_nodes(&mut self, hosts: Vec<T>) {
    let mut nodes = self.nodes.lock().unwrap();
    nodes.retain(|n| hosts.contains(n) == false );
  }



  pub fn add_nodes(&mut self, hosts: Vec<T>) {
    let mut nodes = self.nodes.lock().unwrap();
    let mut add = VecDeque::from(hosts);
    nodes.append(&mut add)
  }

  
  pub fn list_nodes(&self) -> Vec<T> {
    let nodes = self.nodes.lock().unwrap();
    nodes.clone().into_iter().collect()
  } 


}













impl <T: Clone + PartialEq> Distributor for RoundRobin<T> {
  type Item = T; 


  fn pick(&mut self) -> Option<Self::Item>{
    let mut nodes = self.nodes.lock().unwrap();
    let node_opt = nodes.pop_front();

    match node_opt {

      Some(node) => {
        nodes.push_back(node.clone());
        Some( node )
      }, 

      None => None 

      
    }

  }

}








pub struct Loaded<T> {
  pub inner: T,
  pub load: AtomicUsize
}


impl<T> Loaded<T> {
  fn new(inner: T) -> Self {
    let load = AtomicUsize::new(0);
    Loaded {load, inner}
  }
}





struct P2C<T> {
  nodes: Arc<RwLock< Vec<Loaded<T> > > >
}


impl <T: Clone> Distributor for P2C<T> {
  type Item = T;


  
  fn pick(&mut self) -> Option<Self::Item> {

    let mut rng = thread_rng();
    let nodes = self.nodes.read().unwrap();
    
    let (i1, i2) = (
      rng.gen_range(0, nodes.len()), rng.gen_range(0, nodes.len())
    );

    let (a, b) = (nodes.get(i1).unwrap(), nodes.get(i2).unwrap() );


    if a.load.load(Ordering::SeqCst) <= b.load.load(Ordering::SeqCst)  {
      a.load.fetch_add(1,Ordering::SeqCst);
      Some( a.inner.to_owned() )
    }

    else {
      b.load.fetch_add(1,Ordering::SeqCst);
      Some(b.inner.to_owned() )
    }


    
  }


  
}


impl <T> From <Vec<T>> for P2C<T> {
  fn from(v: Vec<T>) -> P2C<T> {

    let loaded = v.into_iter().map(|t| Loaded::new(t) ).collect();
    let locked = RwLock::new(loaded);
    let nodes = Arc::new(locked);
    P2C {nodes} 
  }

  
}




impl<T: Clone + PartialEq > P2C<T> {


  fn new() -> P2C<T> {
    Self::from(vec![])
  }


  fn add_node(&mut self, node: T) {
    let mut nodes = self.nodes.write().unwrap();
    let loaded = Loaded::new(node);
    nodes.push(loaded);
  }


  fn remove_node(&mut self, node: T) {
    let mut nodes = self.nodes.write().unwrap() ;
    let result = nodes.iter().position(|x| x.inner == node);
    
    match result {
      Some(i) => Some( nodes.remove(i) ),
      None => None
    };
    
  }


  pub fn add_nodes(&mut self, hosts: Vec<T>) {
    let mut inner = self.nodes.write().unwrap();
    let mut add = hosts.into_iter().map(|x| Loaded::new(x) ).collect();
    inner.append(&mut add)
  }


  pub fn remove_nodes(&self, hosts: Vec<T>) {
    let mut nodes = self.nodes.write().unwrap();
    nodes.retain(|n| hosts.contains(&n.inner) == false );
  }
  
}


#[cfg(test)]
mod tests {


  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }


  
}
