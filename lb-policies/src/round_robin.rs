use super::*;

use std::sync::*;
use std::convert::From;


pub struct RoundRobin<T: Clone + PartialEq> {
  nodes: Arc< RwLock< Vec<T> >>,
  index: AtomicUsize
}



impl <T: Clone + PartialEq> From<Vec<T>> for RoundRobin<T> {
  
  
  fn from(vec: Vec<T>) -> RoundRobin<T> {
    let nodes = Arc::new( RwLock::new(vec) );
    let index = AtomicUsize::new(0);
    RoundRobin { nodes, index }
  }


  
}





impl<T: Clone + PartialEq> RoundRobin<T> {

  pub fn new() -> RoundRobin<T> {
    Self::from(vec![])
  }


  pub fn add_node(&mut self, host: T) {
    let mut nodes = self.nodes.write().unwrap();
    nodes.push(host)
  }



  pub fn remove_node(&mut self, host: T) {
    let mut nodes = self.nodes.write().unwrap();

    let pos = nodes.iter().position(|x| x == &host);
    match pos {

      Some(x) => {
        nodes.remove(x);
        ()
      }
      
      None => ()
    }
    
   

  }


  pub fn remove_nodes(&mut self, hosts: Vec<T>) {
    let mut nodes = self.nodes.write().unwrap();
    nodes.retain(|n| hosts.contains(n) == false );
  }



  pub fn add_nodes(&mut self, hosts: Vec<T>) {
    let mut nodes = self.nodes.write().unwrap();
   
    nodes.extend(hosts)
  }

  
  pub fn list_nodes(&self) -> Vec<T> {
    let nodes = self.nodes.read().unwrap();
    nodes.clone().into_iter().collect()
  } 



  fn next(&self) {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let val = &self.index;
    let inner = self.nodes.read().unwrap();
    
    let mut old = val.load(Ordering::Relaxed);

    
    loop {
      let new = (old + 1) % inner.len();
      match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(_) => break,
        Err(x) => old = x,
      }
    }


    
  } 



  fn index(&self) -> usize {
    self.index.load(Ordering::SeqCst)
  }

  
}













impl <T: Clone + PartialEq> Distributor for RoundRobin<T> {
  type Item = T; 


  fn pick(&self) -> Option<Self::Item> {


    let inner = self.nodes.read().unwrap();
    let i = self.index();

    let n = inner[i].clone();
    self.next();
    
    Some(n)
  }

}



