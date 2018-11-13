use super::*;
//use ring::*;
use round_robin::*;
use p2c::*;
use std::iter::Iterator;
use ring::*;
use std::collections::hash_map::DefaultHasher;

pub trait ServerSet<T>: From<Vec<T>> {
  fn new() -> Self {
    Self::from(vec![])
  }

  
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



impl <W: WeightedNode + Clone> ServerSet<W::Item> for P2C<W> {
  




  
  fn add(&self, node: W::Item) {
    let mut nodes = self.inner.write().unwrap();
    let loaded = W::new(node);
    nodes.push(loaded);
  }



  

  fn remove(&self, node: W::Item) {
    let mut nodes = self.inner.write().unwrap() ;
    let result = nodes.iter().position(|x| &x.inner() == &node);
    
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








impl<T: Clone + PartialEq + Ord> From<Vec<T>> for CHash<T> {
  fn from(v: Vec<T>) -> CHash<T> {
    let mut v1 = v;
    v1.sort();
    
    let locked = RwLock::new(v1);
    let inner = Arc::new(locked); 
    CHash{inner}
  }
} 






impl <T: Clone + PartialEq + Ord> ServerSet<T> for CHash<T> {


  fn add(&self, node: T) {
    let mut nodes = self.inner.write().unwrap();
    
    nodes.push(node);
    nodes.sort()

  }
  

  

  fn remove(&self, node: T) {
    let mut nodes = self.inner.write().unwrap() ;
    let result = nodes.iter().position(|x| x == &node);
    
    match result {
      Some(i) => Some( nodes.remove(i) ),
      None => None
    };
    nodes.sort()
  }


    
  fn remove_nodes(&self, hosts: Vec<T>) {
    let mut nodes = self.inner.write().unwrap();
    nodes.retain(|n| hosts.contains(&n) == false);
    nodes.sort()
  }




  
  fn add_nodes(&self, hosts: Vec<T>) {
    let mut inner = self.inner.write().unwrap();
    inner.extend(hosts);
    inner.sort()
  }





  


  fn resolve(&self, v: Vec<T>) {
    let mut v1 = v;
    v1.sort(); 
    *self.inner.write().unwrap() = v1;
   
  }

  fn list(&self) -> Vec<T> {
    self.inner.read().unwrap().clone()
  }

  
}



impl <W: WeightedNode + Clone> ServerSet<W::Item> for CHashLL<W>
where W::Item: Ord + Clone {
  
  fn add(&self, node: W::Item) {
    let mut nodes = self.inner.write().unwrap();
    let loaded = W::new(node);
    nodes.push(loaded);
    
    nodes.sort_by(|x, y| x.inner().cmp( &y.inner() ) )

  }

  
  fn remove(&self, host: W::Item) {
    let mut nodes = self.inner.write().unwrap();

    let pos = nodes.iter().position(|x| &x.inner() == &host);
    match pos {

      Some(x) => {
        nodes.remove(x);
        ()
      }
      
      None => ()
    };

     nodes.sort_by(|x, y| x.inner().cmp( &y.inner() ) )
  }






  fn remove_nodes(&self, hosts: Vec<W::Item>) {
    let mut nodes = self.inner.write().unwrap();
    nodes.retain(|n| hosts.contains(&n.inner() ) == false );
    nodes.sort_by(|x, y| x.inner().cmp( &y.inner() ) )
  }


  
  fn list(&self) -> Vec<W::Item> {
    let inner = self.inner.read().unwrap();
    inner.iter().map(|x| x.inner() ).collect()
  }

  

  fn add_nodes(&self, hosts: Vec<W::Item>) {
    let mut nodes = self.inner.write().unwrap();
    let add = hosts.iter().map(|x| W::new(x.clone() ) ).collect::<Vec<W>>(); 

    nodes.extend(add);
    nodes.sort_by(|x, y| x.inner().cmp( &y.inner() ) )
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


    handle.sort_by(|x, y| x.inner().cmp( &y.inner() ) )
    
  }
  


  
}




impl <W: WeightedNode + Clone> From <Vec<W::Item>> for CHashLL<W> {
  
  fn from(v: Vec<W::Item>) -> CHashLL<W> {

    let loaded = v.into_iter().map(|inner| {
     W::new(inner)
    }).collect();
    
    let locked = RwLock::new(loaded);
    let inner = Arc::new(locked);
    CHashLL {inner, factor: 3} 
  }

  
  
}


/*
impl <W: WeightedNode> ServerSet<W::Item> for CHashLL<W>
where W::Item: Ord + Clone {


  
}
*/

