extern crate spiffe;
use spiffe::workload::*;

#[test]
fn fetch_svid() {
    let client = Client::<X509>::new("/tmp/agent.sock");
    let result = client.fetch().unwrap();
    println!("{:?}", result)
}
