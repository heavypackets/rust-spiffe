extern crate futures;
extern crate spiffe;

use spiffe::workload::*;
use std::time::Duration;

use futures::Future;
use futures::Stream;

#[macro_use]
extern crate assert_matches;

#[test]
fn x509_fetch_once_svid() {
    let client = X509Client::new("unix:///tmp/agent.sock", None);
    let result = client.fetch(Some(Duration::new(5, 0))).unwrap().unwrap();
    println!("{:?}", result)
}

#[test]
fn x509_fetch_once_fail_no_path() {
    let client = X509Client::new("", None);
    let result = client.fetch(Some(Duration::new(5, 0)));
    if let Err(err) = result {
        assert_matches!(err, Error(ErrorKind::FetchFailure, _));
    } else {
        panic!("Expected error")
    }
}

#[test]
fn x509_fetch_once_fail_invalid_path() {
    let client = X509Client::new("/path/to/nowhere", None);
    let result = client.fetch(Some(Duration::new(5, 0)));
    if let Err(err) = result {
        assert_matches!(err, Error(ErrorKind::FetchFailure, _));
    } else {
        panic!("Expected error")
    }
}

#[test]
fn x509_fetch_once_fail_no_uri() {
    let client = X509Client::new("///tmp/agent.sock", None);
    let result = client.fetch(Some(Duration::new(5, 0)));
    if let Err(err) = result {
        assert_matches!(err, Error(ErrorKind::FetchFailure, _));
    } else {
        panic!("Expected error")
    }
}

#[test]
fn x509_stream_take_one() {
    let client = X509Client::new("unix:///tmp/agent.sock", None);
    let stream = client.stream(Some(Duration::new(5, 0))).unwrap();
    let _item = match stream.take(1).collect().wait() {
        Ok(mut items) => {
            let val = items.pop().unwrap();
            X509Payload::new(val).unwrap()
        }
        _ => panic!("Expected responses returned"),
    };
}

#[test]
fn x509_stream_for_each() {
    let client = X509Client::new("unix:///tmp/agent.sock", None);
    let stream = client.stream(Some(Duration::new(60, 0))).unwrap();
    let _res = stream
        .take(2)
        .for_each(|val| {
            X509Payload::new(val).unwrap();
            Ok(())
        }).wait();
}
