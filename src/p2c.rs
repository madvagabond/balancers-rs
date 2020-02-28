use super::*;
use rand::prelude::*;

use metrohash::*;
use twox_hash::*;
use std::hash::Hasher;


use crate::server_list::*;
use crate::node::*;
use crate::load::*;




pub struct P2C<T, L> where T: Node, L: Load  {
  servers: ServerList<T, L>
}



impl <T, L> P2C <T, L> where T: Node, L: Load {

    
    fn hash(&self, mut hasher: impl Hasher, key: &[u8]) -> &WeightedNode<T, L> {
      let servers = self.servers.list();

      hasher.write(key);
      let code = hasher.finish() as usize;
      let index = code % servers.len();
      &servers[index]
    }

    
  }







impl <T, L> Balancer for P2C <T, L> where T: Node, L: Load  {
  type Node = WeightedNode<T, L>;
  type Servers = ServerList<T, L>;
  
  
  fn balance(&self) -> &Self::Node {

    let servers = self.servers.list(); 
    let mut rng = thread_rng();
    
    let a = servers.choose(&mut rng).unwrap();
    let b =  servers.choose(&mut rng).unwrap();


    if a.load().load() >= b.load().load() {
      &a
    }

    else {
      &b
    }
    

  }
  


  fn servers(&self) -> &Self::Servers {
    &self.servers
  }


  fn servers_mut(&mut self) -> &mut Self::Servers {
    &mut self.servers
  }

  
}






impl <T, L> KeyedBalancer for P2C <T, L> where T: Node, L: Load {
  type Node = WeightedNode<T, L>;
  type Servers = ServerList<T, L>;

  fn balance(&self, key: &[u8]) -> &Self::Node {
    let hasher_a = MetroHash64::default();
    let hasher_b = XxHash64::default();      

    let a = self.hash(hasher_a, key);
    let b = self.hash(hasher_b, key);

    if a.load().load() >= b.load().load() {
      &a
    }

    else {
      &b 
    }

    
  }



  fn servers(&self) -> & Self::Servers {
    &self.servers
  }

  fn servers_mut(&mut self) -> &mut Self::Servers {
    &mut self.servers
  }
  


}

