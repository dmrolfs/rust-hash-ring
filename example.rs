extern crate hash_ring;

use hash_ring::hash_ring::HashRing;

#[derive(Clone)]
struct NodeInfo {
	pub host: &'static str,
	pub port: u16
}

impl ToString for NodeInfo {
	fn to_string(&self) -> String {
		format!("{}:{}", self.host, self.port)
	}
}

fn main() {
	let mut nodes: Vec<NodeInfo> = Vec::new();
	nodes.push(NodeInfo{host: "localhost", port: 15324});
	nodes.push(NodeInfo{host: "localhost", port: 15325});
	nodes.push(NodeInfo{host: "localhost", port: 15326});
	nodes.push(NodeInfo{host: "localhost", port: 15327});
	nodes.push(NodeInfo{host: "localhost", port: 15328});
	nodes.push(NodeInfo{host: "localhost", port: 15329});

	let mut hash_ring: HashRing<NodeInfo> = HashRing::new(nodes, 10);

	println!("{}", hash_ring.get_node("hello".to_string()).to_string());

	println!("{}", hash_ring.get_node("dude".to_string()).to_string());

	println!("{}", hash_ring.get_node("martian".to_string()).to_string());

	println!("{}", hash_ring.get_node("tardis".to_string()).to_string());

	hash_ring.remove_node(&NodeInfo{host: "localhost", port: 15329});

	println!("{}", hash_ring.get_node("hello".to_string()).to_string());

	hash_ring.add_node(&NodeInfo{host: "localhost", port: 15329});

	println!("{}", hash_ring.get_node("hello".to_string()).to_string());
}