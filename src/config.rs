extern crate serde_json;

use serde_json::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
    struct Cluster {
    dnsname: String,
    strategy: String,
    port: u8,
    type: String,
    hosts: Vec<String>,
}

fn start_cluster(dnsname: Option<String>, strategy: Option<String>, port: Option<u8>, ) -> Result<()> {
    

}