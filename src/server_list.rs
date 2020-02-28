use crate::node::*;
use crate::load::*;




pub struct ServerList<T, L> {
  servers: Vec<WeightedNode<T, L> >,
  sorted: bool
}






impl <T: Node, L: Load> ServerList <T, L> {

  fn finalize(&mut self) {
    if self.sorted {
      self.servers.sort_unstable_by_key(|x| x.id())
    }
  }

  
  pub fn add(&mut self, host: T) {
    let servers = &mut self.servers;
    let item = WeightedNode::<T, L>::new(host);
    
    if !servers.iter().any(|x| x.id() == item.id()) {
      servers.push(item)
    }


    self.finalize() 
  }



  

  pub fn remove(&mut self, host: &T) {

    self.servers.retain(|x| x.id() != host.id());
    self.finalize()
  }


  
  pub fn update(&mut self, hosts: Vec<T>) {

    for x in hosts {
      self.add(x)
    }

    
  }



  pub fn set(&mut self, hosts: Vec<T>) {


    self.servers.retain(|x| {
      hosts.iter().any( |y| y.id() == x.id() )
    });
                        



    hosts.into_iter().for_each(|x| self.add(x))
    
  }


  pub fn list(&self) -> &Vec<WeightedNode<T, L> > {
    &self.servers
  }

}
