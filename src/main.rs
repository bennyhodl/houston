#[macro_use] extern crate rocket;

use ldk_node::bitcoin::Address;
use ldk_node::lightning::ln::channelmanager::ChannelDetails;
use ldk_node::{Builder, Node, Error};
use rocket::{get, State, routes};
use rocket::serde::{Serialize, json::Json};

struct NodeState {
	node: Node
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NodeInfo {
	node_id: String,
	listening_address: String,
	onchain_balance: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NewAddress {
	address: String
}

#[get("/getinfo")]
fn node_info(ldk: &State<NodeState>) -> Json<NodeInfo> {
	let listening_address = ldk.node.listening_address();
	let node_id = ldk.node.node_id();
	let onchain_balance = ldk.node.on_chain_balance();
	let _channels = ldk.node.list_channels();
	Json(NodeInfo { 
		node_id: node_id.to_string(), 
		listening_address: listening_address.unwrap(), 
		onchain_balance: onchain_balance.unwrap().confirmed,
	})
}

#[get("/newaddress")]
fn new_funding_address(ldk: &State<NodeState>) -> Json<NewAddress> {
	let address = ldk.node.new_funding_address();
	Json(NewAddress {
		address: address.unwrap().to_string()
	})
}

#[launch]
fn rocket() -> _ {
	let node = Builder::new()
		.set_network("testnet")
		.set_esplora_server_url("https://blockstream.info/testnet/api".to_string())
		.build();
  
	node.start().unwrap();

	let my_node = NodeState { node: node };

	rocket::build()
		.manage(my_node)
		.mount("/", routes![node_info, new_funding_address])
}


