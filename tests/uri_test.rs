extern crate spiffe;

#[test]
fn valid_spiffe_id() {
    spiffe::uri::URI::from_str("spiffe://example.org/path").unwrap();
}

#[test]
fn valid_spiffe_id_no_domain() {
    spiffe::uri::URI::from_str("spiffe://example/path").unwrap();
}

#[test]
fn valid_spiffe_id_special_characters_1() {
    spiffe::uri::URI::from_str("spiffe://example/h^t").unwrap();
}

#[test]
fn valid_spiffe_id_special_characters_2() {
    spiffe::uri::URI::from_str("spiffe://ex-ample*/path").unwrap();
}

#[test]
fn valid_spiffe_id_unicode() {
    spiffe::uri::URI::from_str("spiffe://example✔/path").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_blank() {
    spiffe::uri::URI::from_str("").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_path_1() {
    spiffe::uri::URI::from_str("spiffe://example.org/").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_path_2() {
    spiffe::uri::URI::from_str("spiffe://example.org").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_port() {
    spiffe::uri::URI::from_str("spiffe://example.org:8000/path").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_username_password() {
    spiffe::uri::URI::from_str("spiffe://admin:password@example.org/path").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_http() {
    spiffe::uri::URI::from_str("http://example.org/path").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_scheme() {
    spiffe::uri::URI::from_str("/example.org/path").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_query() {
    spiffe::uri::URI::from_str("spiffe://example.org/path?somequery").unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_fragment() {
    spiffe::uri::URI::from_str("spiffe://example.org/path#somequery").unwrap();
}

#[test]
fn fetch_trust_domain() {
    let id = spiffe::uri::URI::from_str("spiffe://example.org/path").unwrap();
    assert_eq!(id.trust_domain(), "example.org");
}

#[test]
fn fetch_trust_special_characters() {
    let id = spiffe::uri::URI::from_str("spiffe://exa^m-ple.org/path").unwrap();
    assert_eq!(id.trust_domain(), "exa^m-ple.org");
}

/*
#[test]
#[should_panic]
fn fetch_trust_domain_unicode() {
    let id = spiffe::uri::URI::from_str("spiffe://example✔/path").unwrap();
    assert_eq!(id.trust_domain(), "example✔");
} */

#[test]
fn fetch_path_domain() {
    let id = spiffe::uri::URI::from_str("spiffe://example.org/path/to/a/new/year").unwrap();
    assert_eq!(id.path(), "/path/to/a/new/year");
}
