extern crate futures;
extern crate spiffe;

use spiffe::workload::jwt::JWTClient;
use spiffe::workload::x509::{X509Client, X509Payload};
use spiffe::workload::{Error, ErrorKind};
use std::time::Duration;

use futures::executor::block_on;
use futures::future;
use futures::StreamExt;

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
    let mut stream = client.stream(Some(Duration::new(5, 0))).unwrap();
    let item = block_on(stream.next());

    match item {
        Some(Ok(item)) => {
            X509Payload::new(item).unwrap();
        }
        Some(Err(_)) => panic!("Expected responses returned"),
        None => panic!("Expected responses returned"),
    }
}

#[test]
fn x509_stream_for_each() {
    let client = X509Client::new("unix:///tmp/agent.sock", None);
    let stream = client.stream(Some(Duration::new(60, 0))).unwrap();
    let _res = stream.take(2).for_each(|val| {
        X509Payload::new(val.unwrap()).unwrap();
        future::ready(())
    });
}

#[test]
fn jwt_validate_svid() {
    let client = JWTClient::new("unix:///tmp/agent.sock", None);

    let svid = String::from("eyJhbGciOiJFUzI1NiIsImtpZCI6Im04cXZmTXNDYWp2Wkhac3o1Y21qbkJIMlVvMjVDQ0ZYIiwidHlwIjoiSldUIn0.eyJhdWQiOlsicGFyc2VjIl0sImV4cCI6MTYwMjEwMTcxOSwiaWF0IjoxNjAyMTAxNDE5LCJzdWIiOiJzcGlmZmU6Ly9leGFtcGxlLm9yZy9teXNlcnZpY2UifQ.EPpsPJg48T616vZEoZqclR8HjRyTj8qTUCymf5yWYTFXI2goL9bLCZP8Im6heHE_4o7JhRGgpzXbjxmqzgsUyw");
    let audience = String::from("parsec");

    let result = client
        .validate(audience, svid, Some(Duration::new(5, 0)))
        .unwrap();
    println!("{:?}", result.0);
    println!("{:?}", result.1);
}
