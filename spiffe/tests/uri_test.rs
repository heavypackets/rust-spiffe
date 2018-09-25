extern crate spiffe;

use spiffe::uri::URI;

#[test]
fn valid_spiffe_id() {
    "spiffe://example.org/path".parse::<URI>().unwrap();
}

#[test]
fn valid_spiffe_id_no_domain() {
    "spiffe://example/path".parse::<URI>().unwrap();
}

#[test]
fn valid_spiffe_id_special_characters_1() {
    "spiffe://example/h^t".parse::<URI>().unwrap();
}

#[test]
fn valid_spiffe_id_special_characters_2() {
    "spiffe://ex-ample*/path".parse::<URI>().unwrap();
}

#[test]
fn valid_spiffe_id_unicode() {
    "spiffe://example✔/path".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_blank() {
    "".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_path_1() {
    "spiffe://example.org/".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_path_2() {
    "spiffe://example.org".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_port() {
    "spiffe://example.org:8000/path".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_username_password() {
    "spiffe://admin:password@example.org/path"
        .parse::<URI>()
        .unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_http() {
    "http://example.org/path".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_no_scheme() {
    "/example.org/path".parse::<URI>().unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_query() {
    "spiffe://example.org/path?somequery"
        .parse::<URI>()
        .unwrap();
}

#[test]
#[should_panic]
fn invalid_spiffe_id_contains_fragment() {
    "spiffe://example.org/path#somequery"
        .parse::<URI>()
        .unwrap();
}

#[test]
fn fetch_trust_domain() {
    let id = "spiffe://example.org/path".parse::<URI>().unwrap();
    assert_eq!(id.trust_domain(), "example.org");
}

#[test]
fn fetch_trust_special_characters() {
    let id = "spiffe://exa^m-ple.org/path".parse::<URI>().unwrap();
    assert_eq!(id.trust_domain(), "exa^m-ple.org");
}

#[test]
fn fetch_path_domain() {
    let id = "spiffe://example.org/path/to/a/new/year"
        .parse::<URI>()
        .unwrap();
    assert_eq!(id.path(), "/path/to/a/new/year");
}

#[test]
fn test_equality() {
    let id1 = "spiffe://example.org/path/".parse::<URI>().unwrap();
    let id2 = "spiffe://example.org/path/".parse::<URI>().unwrap();

    assert_eq!(id1, id2);
    assert_eq!(id2, id1);
}

#[test]
fn test_inequality() {
    let id1 = "spiffe://example.org/path/".parse::<URI>().unwrap();
    let id2 = "spiffe://example.org/someotherpath/"
        .parse::<URI>()
        .unwrap();

    assert_ne!(id1, id2);
    assert_ne!(id2, id1);
}
