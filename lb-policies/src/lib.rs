use std::sync::*;
use std::collections::VecDeque;
use std::convert::From;




trait Sharder {
  type Item;
  type Key;

  fn pick(&mut self, key: &Self::Key) -> &Self::Item;
}



trait Distributor {
  type Item;
  fn pick(&mut self) -> Option<Self::Item>;
}



trait Completeable {
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









#[cfg(test)]
mod tests {


  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }


  
}
