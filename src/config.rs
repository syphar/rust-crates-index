use crate::dirs::crate_prefix;
use serde_derive::Deserialize;

/// marker / template vars for generating download urls.
const CRATE_MARKER: &str = "{crate}";
const VERSION_MARKER: &str = "{version}";
const PREFIX_MARKER: &str = "{prefix}";
const LOWERPREFIX_MARKER: &str = "{lowerprefix}";
const SHA256_CHECKSUM_MARKER: &str = "{sha256-checksum}";
const DOWNLOAD_URL_MARKERS: [&str; 5] = [
    CRATE_MARKER,
    VERSION_MARKER,
    PREFIX_MARKER,
    LOWERPREFIX_MARKER,
    SHA256_CHECKSUM_MARKER,
];

/// Global configuration of an index, reflecting the [contents of config.json](https://doc.rust-lang.org/cargo/reference/registries.html#index-format).
#[derive(Clone, Debug, Deserialize)]
pub struct IndexConfig {
    /// Pattern for creating download URLs. Use [`IndexConfig::download_url`] or
    /// [`IndexConfig::download_url_with_checksum`] instead.
    pub dl: String,
    /// Base URL for publishing, etc.
    pub api: Option<String>,
}

impl IndexConfig {
    /// Get the URL from where the specified package can be downloaded.
    /// This method assumes the particular version is present in the registry,
    /// and does not verify that it is.
    ///
    /// Returns `None` when the configured URL requires the
    /// `{sha256-checksum}` placeholder. Use [`Self::download_url_with_checksum`]
    /// when the checksum is available.
    #[must_use]
    pub fn download_url(&self, name: &str, version: &str) -> Option<String> {
        self.download_url_inner(name, version, None)
    }

    /// Get the URL from where the specified package can be downloaded, including
    /// the package checksum when required by the registry's URL template.
    ///
    /// This method assumes the particular version is present in the registry,
    /// and does not verify that it is.
    #[must_use]
    pub fn download_url_with_checksum(&self, name: &str, version: &str, checksum: &[u8; 32]) -> Option<String> {
        self.download_url_inner(name, version, Some(checksum))
    }

    fn download_url_inner(&self, name: &str, version: &str, checksum: Option<&[u8; 32]>) -> Option<String> {
        if !DOWNLOAD_URL_MARKERS.iter().any(|marker| self.dl.contains(marker)) {
            let mut new = String::with_capacity(self.dl.len() + name.len() + version.len() + 10);
            new.push_str(&self.dl);
            new.push('/');
            new.push_str(name);
            new.push('/');
            new.push_str(version);
            new.push_str("/download");
            Some(new)
        } else {
            let prefix = if self.dl.contains(PREFIX_MARKER) || self.dl.contains(LOWERPREFIX_MARKER) {
                let mut prefix = String::with_capacity(5);
                crate_prefix(&mut prefix, name, '/')?;
                Some(prefix)
            } else {
                None
            };
            let lowerprefix = prefix.as_ref().map(|prefix| prefix.to_ascii_lowercase());
            let checksum = checksum.map(hex::encode);
            if self.dl.contains(SHA256_CHECKSUM_MARKER) && checksum.is_none() {
                return None;
            }

            Some(
                self.dl
                    .replace(CRATE_MARKER, name)
                    .replace(VERSION_MARKER, version)
                    .replace(PREFIX_MARKER, prefix.as_deref().unwrap_or_default())
                    .replace(LOWERPREFIX_MARKER, lowerprefix.as_deref().unwrap_or_default())
                    .replace(SHA256_CHECKSUM_MARKER, checksum.as_deref().unwrap_or_default()),
            )
        }
    }
}
