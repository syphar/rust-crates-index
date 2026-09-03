mod git;
mod config;
mod names;
mod sparse_index;
mod error {
    #[test]
    fn error_is_send() {
        fn is_send<T: Send>() {}
        is_send::<crates_index::Error>();
    }
}

use crates_index::{Crate, Dependency, Version};

#[test]
fn sizes() {
    assert!(std::mem::size_of::<Version>() <= 160);
    assert!(std::mem::size_of::<Crate>() <= 16);
    assert!(std::mem::size_of::<Dependency>() <= 104);
}

#[test]
fn semver() {
    let c = Crate::from_slice(r#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false, "pubtime":"2025-11-12T19:30:12Z"}
            {"vers":"1.2.0-alpha.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false, "pubtime":"2025-11-12T19:30:12Z"}
            {"vers":"1.0.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "yanked":false, "pubtime":"2025-11-12T19:30:12Z"}"#.as_bytes()).unwrap();
    assert_eq!(c.most_recent_version().version(), "1.0.1");
    assert_eq!(c.highest_version().version(), "1.2.0-alpha.1");
    assert_eq!(c.highest_normal_version().unwrap().version(), "1.0.1");
}

#[test]
fn features2() {
    let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{"a":["one"], "b":["x"]},"features2":{"a":["two"], "c":["y"]}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "pubtime":"2025-11-12T19:30:12Z"}"#).unwrap();
    let f2 = c.most_recent_version().features();

    assert_eq!(3, f2.len());
    assert_eq!(["one", "two"], &f2["a"][..]);
    assert_eq!(["x"], &f2["b"][..]);
    assert_eq!(["y"], &f2["c"][..]);
}

#[test]
fn rust_version() {
    let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{},"features2":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "rust_version":"1.64.0", "pubtime":"2025-11-12T19:30:12Z"}"#).unwrap();
    assert_eq!(c.most_recent_version().rust_version(), Some("1.64.0"));
}

#[test]
fn pubtime() {
    let c = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234", "pubtime":"2025-11-12T19:30:12Z"}
        {"vers":"1.0.1", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234"}"#).unwrap();

    assert_eq!(c.versions()[0].pubtime(), Some("2025-11-12T19:30:12Z"));
    assert_eq!(c.versions()[1].pubtime(), None, "pubtime is optional");
}

#[test]
fn missing_pubtime_is_accepted() {
    let result = Crate::from_slice(br#"{"vers":"1.0.0", "name":"test", "deps":[], "features":{}, "cksum":"1234567890123456789012345678901234567890123456789012345678901234"}"#);

    assert!(result.is_ok());
}
