use crate::server_list::*;
use crate::node::*;
use std::sync::atomic::*;
use super::*;

pub struct RoundRobin<T> {
  servers: ServerList<T, ()>,
  counter: AtomicUsize
}


impl <T: Node> RoundRobin<T>  {
  fn next(&self) -> &WeightedNode<T, ()> {
    
    let servers = self.servers.list();
    let i = self.counter.fetch_add(1, Ordering::Relaxed)  % servers.len();
    &servers[i]
  }

  
}





impl <T: Node> Balancer for RoundRobin<T> {
  type Node = WeightedNode<T, ()>;
  type Servers = ServerList<T, ()>;

  fn balance(&self) -> &Self::Node {
    self.next()
  }


  fn servers(&self) -> &Self::Servers {
    &self.servers
  }

  fn servers_mut(&mut self) -> &mut Self::Servers {
    &mut self.servers
  }
  
}
