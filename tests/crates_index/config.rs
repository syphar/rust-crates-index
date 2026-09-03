use crates_index::Crate;
use crates_index::IndexConfig;

#[test]
fn appends_the_default_download_path() {
    assert_eq!(
        config("https://example.invalid/crates")
            .download_url("demo", "1.2.3")
            .as_deref(),
        Some("https://example.invalid/crates/demo/1.2.3/download"),
    );
}

#[test]
fn substitutes_all_documented_markers() {
    let checksum = [0xab; 32];
    let config = config("https://example.invalid/{crate}/{version}/{prefix}/{lowerprefix}/{sha256-checksum}");

    assert_eq!(
        config
            .download_url_with_checksum("MyCrate", "1.2.3", &checksum)
            .as_deref(),
        Some(
            "https://example.invalid/MyCrate/1.2.3/My/Cr/my/cr/\
                 abababababababababababababababababababababababababababababababab"
        ),
    );
}

#[test]
fn checksum_marker_requires_a_checksum() {
    assert_eq!(
        config("https://example.invalid/{sha256-checksum}").download_url("demo", "1.2.3"),
        None,
    );
}

#[test]
fn templates_without_prefix_markers_do_not_build_one() {
    assert_eq!(
        config("https://example.invalid/{crate}/{version}")
            .download_url("", "1.2.3")
            .as_deref(),
        Some("https://example.invalid//1.2.3"),
    );
}

#[test]
fn version_download_url_supplies_its_checksum() {
    let krate = Crate::from_slice(
            br#"{"name":"demo","vers":"1.2.3","deps":[],"cksum":"abababababababababababababababababababababababababababababababab","features":{}}"#,
        )
        .unwrap();

    assert_eq!(
        krate.versions()[0]
            .download_url(&config("https://example.invalid/{sha256-checksum}"))
            .as_deref(),
        Some("https://example.invalid/abababababababababababababababababababababababababababababababab"),
    );
}

fn config(dl: &str) -> IndexConfig {
    IndexConfig {
        dl: dl.to_owned(),
        api: None,
    }
}
