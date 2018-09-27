extern crate spiffe;

use spiffe::workload::*;
use std::time::Duration;

#[test]
fn fetch_one_svid() {
    let client = Client::<X509>::new("unix:///tmp/agent.sock", None);
    let result = client
        .fetch_once(Some(Duration::new(5, 0)))
        .unwrap()
        .unwrap();
    println!("{:?}", result)
}
