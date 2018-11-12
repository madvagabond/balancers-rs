use super::*;
//use ring::*;
use round_robin::*;
use p2c::*;
use std::iter::Iterator;


pub trait ServerSet<T>: From<Vec<T>> {
  fn new() -> Self;
  
  fn add(&self, node: T);
  fn remove(&self, node: T);

  fn remove_nodes(&self, nodes: Vec<T>);
  fn add_nodes(&self, nodes: Vec<T>);
  fn list(&self) -> Vec<T>;

  fn resolve(&self, v: Vec<T>);
}




impl <T: Clone + PartialEq> From<Vec<T>> for RoundRobin<T> {
  
  
  fn from(vec: Vec<T>) -> RoundRobin<T> {
    let inner = Arc::new( RwLock::new(vec) );
    let index = AtomicUsize::new(0);
    RoundRobin { inner, index }
  }


  
}








impl <T: Clone + PartialEq> ServerSet<T> for RoundRobin<T> {
  
  fn new() -> RoundRobin<T> {
    Self::from(vec![])
  }


  fn add(&self, host: T) {
    let mut nodes = self.inner.write().unwrap();
    nodes.push(host)
  }



  fn remove(&self, host: T) {
    let mut nodes = self.inner.write().unwrap();

    let pos = nodes.iter().position(|x| x == &host);
    match pos {

      Some(x) => {
        nodes.remove(x);
        ()
      }
      
      None => ()
    }

    }






  fn remove_nodes(&self, hosts: Vec<T>) {
    let mut nodes = self.inner.write().unwrap();
    nodes.retain(|n| hosts.contains(n) == false );
  }



  fn add_nodes(&self, hosts: Vec<T>) {
    let mut nodes = self.inner.write().unwrap();
   
    nodes.extend(hosts)
  }

  
  fn list(&self) -> Vec<T> {
    let nodes = self.inner.read().unwrap();
    nodes.clone().into_iter().collect()
  } 


  fn resolve(&self, v: Vec<T>) {
    *self.inner.write().unwrap() = v; 
  }

  
  
}



impl <W> ServerSet<W::Item> for P2C<W>
where W : WeightedNode + Clone,
W::Item: Clone
{
  

  fn new() -> P2C<W> {
    Self::from(vec![])
  }



  
  fn add(&self, node: W::Item) {
    let mut nodes = self.inner.write().unwrap();
    let loaded = W::new(node);
    nodes.push(loaded);
  }



  

  fn remove(&self, node: W::Item) {
    let mut nodes = self.inner.write().unwrap() ;
    let result = nodes.iter().position(|x| x.inner() == node);
    
    match result {
      Some(i) => Some( nodes.remove(i) ),
      None => None
    };
    
  }




  
  fn add_nodes(&self, hosts: Vec<W::Item>) {
    let mut inner = self.inner.write().unwrap();
    let mut add = hosts.into_iter().map(|x| W::new(x) ).collect();
    inner.append(&mut add)
  }





  
  fn remove_nodes(&self, hosts: Vec<W::Item>) {
    let mut nodes = self.inner.write().unwrap();
    nodes.retain(|n| hosts.contains(&n.inner()) == false );
  }



  fn resolve(&self, hosts: Vec<W::Item>) {

    let mut handle = self.inner.write().unwrap();
    handle.retain(|n| hosts.contains(&n.inner()) );
    let data = handle.clone();
    


    
    for h in hosts.iter() {
      if let Some(_) = data.iter().position(|x| &x.inner() != h) {
        let w = W::new(h.clone() );
        handle.push(w)
      }
    }
    
  }
  


  fn list(&self) -> Vec<W::Item> {
    let inner = self.inner.read().unwrap();
    inner.iter().map(|x| x.inner() ).collect()
  }

  
}
