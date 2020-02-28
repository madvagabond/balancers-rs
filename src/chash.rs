

use metrohash::*;
use std::hash::Hasher;

use crate::server_list::*;
use crate::node::*;
use crate::load::*;
use super::*; 


pub struct Ring<T, L> {
  servers: ServerList<T, L>,
  replicas: usize 
}




impl <T, L> Ring <T, L> where T: Node, L: Load {


  fn hash(&self, key: &[u8]) -> usize {
    let mut hasher = MetroHash64::default();
    let servers = self.servers.list();
    
    hasher.write(key);
    let id = hasher.finish();
    jump_hash(id, servers.len())
  } 


  
  pub fn replicas(&self, key: &[u8]) -> Vec<&WeightedNode<T, L> > {
    let i = self.hash(key);
    let j = i + self.replicas;
    let hosts = self.servers.list();
    
    (i..j).map(|x| {
      let index = x % hosts.len();
      &hosts[index]
    }).collect()
      
  }

  
  pub fn least_loaded(&self, key: &[u8]) -> &WeightedNode<T, L> {
    self.replicas(key).iter().min_by_key(|node| node.load().load() ).unwrap()
  }


  pub fn get_node(&self, key: &[u8]) -> &WeightedNode<T, L> {
    let i = self.hash(key);
    &self.servers.list()[i]
  }


  pub fn servers(&self) -> &ServerList<T, L> {
    &self.servers
  }


  pub fn servers_mut(&mut self) -> &mut ServerList<T, L> {
    &mut self.servers
  }

  
}






pub struct Consistent <T> {
  pub ring: Ring<T, ()> 
}


pub struct Bounded <T, L> {
  pub ring: Ring<T, L>
}





impl <T: Node> KeyedBalancer for Consistent<T> {
  type Node = WeightedNode<T, ()>;
  type Servers = ServerList<T, ()>;

  fn balance(&self, key: &[u8]) -> Self::Node {
    self.ring.get_node(key)
  }


  fn servers(&self) -> &Self::Servers {
    &self.ring.servers
  }

  fn servers_mut(&mut self) -> &mut Self::Servers {
    &mut self.ring
  }
  
}



impl <T: Node, L: Load> KeyedBalancer for Bounded <T, L> {
  type Node = WeightedNode<T, ()>;
  type Servers = ServerList<T, ()>;

  fn balance(&self, key: &[u8]) -> Self::Node {
    self.ring.get_node(key)
  }


  fn servers(&self) -> &Self::Servers {
    &self.ring.servers
  }

  fn servers_mut(&mut self) -> &mut Self::Servers {
    &mut self.ring
  }
  
}




fn jump_hash(key: u64, num_buckets: usize) -> usize {

  let num_buckets = num_buckets as i64; 
  let mut b = 0;
  let mut j = 0;
  let mut key = key;

  
  
  while j < num_buckets {
    b = j;
    key = key.wrapping_mul(2_862_933_555_777_941_757).wrapping_add(1);
    j = ((b.wrapping_add(1) as f64) * ((1i64 << 31) as f64)
         / ((key >> 33).wrapping_add(1) as f64)) as i64;
  }

  b as usize 
}


