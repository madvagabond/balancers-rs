extern crate rand;
extern crate metrohash;
extern crate twox_hash;


pub mod server_list;
pub mod node;
pub mod load; 

pub mod p2c;
pub mod chash; 


pub trait Balancer {
  type Node;
  type Servers;

  
  fn balance(&self) -> &Self::Node;
  fn servers(&self) -> &Self::Servers;
  fn servers_mut(&mut self) -> &mut Self::Servers;
}





pub trait KeyedBalancer {

  type Node;
  type Servers; 
  
  fn balance(&self, key: &[u8]) -> &Self::Node;
  fn servers(&self) -> &Self::Servers;
  fn servers_mut(&mut self) -> &mut Self::Servers;
}


