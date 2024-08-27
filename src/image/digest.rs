//! Functionality corresponding to <https://github.com/opencontainers/image-spec/blob/main/descriptor.md#digests>.

use std::str::FromStr;

/// The well-known identifier for a SHA-256 digest.
/// At this point, no one is really using alternative digests like sha512, so we
/// don't yet try to expose them here in a higher level way.
const ALG_SHA256: &str = "sha256";

fn char_is_lowercase_ascii_hex(c: char) -> bool {
    matches!(c, '0'..='9' | 'a'..='f')
}

/// algorithm-component ::= [a-z0-9]+
fn char_is_algorithm_component(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9')
}

/// encoded ::= [a-zA-Z0-9=_-]+
fn char_is_encoded(c: char) -> bool {
    char_is_algorithm_component(c) || matches!(c, '=' | '_' | '-')
}

/// A parsed pair of algorithm:digest as defined
/// by <https://github.com/opencontainers/image-spec/blob/main/descriptor.md#digests>
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::str::FromStr;
/// use oci_spec::image::Digest;
/// let d = Digest::from_str("sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b")?;
/// assert_eq!(d.algorithm(), "sha256");
/// assert_eq!(d.value(), "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b");
/// let d = Digest::from_str("multihash+base58:QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8")?;
/// assert_eq!(d.algorithm(), "multihash+base58");
/// assert_eq!(d.value(), "QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8");
/// # Ok(())
/// # }
/// ```

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Digest {
    /// The underlying buffer
    buf: Box<str>,
    /// The byte offset of the `:`
    split: usize,
}

impl<'de> serde::Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl serde::ser::Serialize for Digest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.buf)
    }
}

impl Digest {
    const ALGORITHM_SEPARATOR: &'static [char] = &['+', '.', '_', '-'];
    /// The algorithm name (e.g. sha256, sha512)
    pub fn algorithm(&self) -> &str {
        &self.buf[0..self.split]
    }

    /// The algorithm component (lowercase hexadecimal)
    pub fn value(&self) -> &str {
        &self.buf[self.split + 1..]
    }

    /// View this digest as a valid SHA-256 digest, or return an error.
    pub fn to_sha256(&self) -> crate::Result<Sha256Digest> {
        Sha256Digest::try_from(self.clone())
    }
}

impl FromStr for Digest {
    type Err = crate::OciSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(split) = s.find(':') else {
            return Err(crate::OciSpecError::Other("missing ':' in digest".into()));
        };
        let (algorithm, value) = s.split_at(split);
        let value = &value[1..];

        // algorithm ::= algorithm-component (algorithm-separator algorithm-component)*
        let algorithm_parts = algorithm.split(Self::ALGORITHM_SEPARATOR);
        for part in algorithm_parts {
            if part.is_empty() {
                return Err(crate::OciSpecError::Other(
                    "Empty algorithm component".into(),
                ));
            }
            if !part.chars().all(char_is_algorithm_component) {
                return Err(crate::OciSpecError::Other(format!(
                    "Invalid algorithm component: {part}"
                )));
            }
        }

        if value.is_empty() {
            return Err(crate::OciSpecError::Other("Empty algorithm value".into()));
        }
        if !value.chars().all(char_is_encoded) {
            return Err(crate::OciSpecError::Other(format!(
                "Invalid encoded value {value}"
            )));
        }

        Ok(Self {
            buf: s.into(),
            split,
        })
    }
}

/// A SHA-256 digest; this can only be constructed by first parsing a [`Digest`].
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::str::FromStr;
/// use oci_spec::image::{Digest, Sha256Digest};
/// let d = Digest::from_str("sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b")?;
/// assert_eq!(d.algorithm(), "sha256");
/// assert_eq!(d.value(), "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b");
/// let d = Sha256Digest::try_from(d)?;
/// assert_eq!(d.value(), "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b");
/// // But other digests will fail to be converted
/// let d = Digest::from_str("multihash+base58:QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8")?;
/// assert!(Sha256Digest::try_from(d).is_err());
/// # Ok(())
/// # }
/// ```

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sha256Digest {
    /// The underlying SHA-256 digest, guaranteed to be 64 lowercase hexadecimal ASCII characters.
    value: Box<str>,
}

impl From<Sha256Digest> for Digest {
    fn from(value: Sha256Digest) -> Self {
        Self {
            buf: format!("sha256:{}", value.value).into_boxed_str(),
            split: 6,
        }
    }
}

impl TryFrom<Digest> for Sha256Digest {
    type Error = crate::OciSpecError;

    fn try_from(algdigest: Digest) -> Result<Self, Self::Error> {
        match algdigest.algorithm() {
            ALG_SHA256 => {}
            o => {
                return Err(crate::OciSpecError::Other(format!(
                    "Expected algorithm {ALG_SHA256} but found {o}"
                )))
            }
        }

        Self::from_str(algdigest.value())
    }
}

impl FromStr for Sha256Digest {
    type Err = crate::OciSpecError;

    fn from_str(digest: &str) -> Result<Self, Self::Err> {
        let is_all_hex = digest.chars().all(char_is_lowercase_ascii_hex);
        if !(digest.len() == 64 && is_all_hex) {
            return Err(crate::OciSpecError::Other(format!(
                "Invalid SHA-256 digest: {digest}"
            )));
        }
        Ok(Self {
            value: digest.into(),
        })
    }
}

impl Sha256Digest {
    /// The underlying SHA-256 digest, guaranteed to be 64 lowercase hexadecimal ASCII characters.
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest_invalid() {
        let invalid = [
            "",
            "foo",
            ":",
            "blah+",
            "_digest:somevalue",
            ":blah",
            "blah:",
            "^:foo",
            "bar^baz:blah",
            "sha256:123456*78",
        ];
        for case in invalid {
            assert!(
                Digest::from_str(case).is_err(),
                "Should have failed to parse: {case}"
            )
        }
    }

    const VALID_DIGEST_SHA256: &str =
        "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b";

    #[test]
    fn test_digest_valid() {
        let cases = ["foo:bar", "sha256:blah", "sha512:12345"];
        for case in cases {
            Digest::from_str(case).unwrap();
        }

        let d = Digest::from_str("multihash+base58:QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8")
            .unwrap();
        assert_eq!(d.algorithm(), "multihash+base58");
        assert_eq!(d.value(), "QmRZxt2b1FVZPNqd8hsiykDL3TdBDeTSPX9Kv46HmX4Gx8");
    }

    #[test]
    fn test_sha256_invalid() {
        let invalid = [
            "sha256:123456=78",
            "foo:bar",
            "sha256+blah:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b",
        ];
        for case in invalid {
            let d = Digest::from_str(case).unwrap();
            assert!(
                Sha256Digest::try_from(d).is_err(),
                "Should have failed to parse: {case}"
            )
        }
    }

    #[test]
    fn test_sha256_valid() {
        let d = Digest::from_str(VALID_DIGEST_SHA256).unwrap();
        let d = d.to_sha256().unwrap();
        assert_eq!(d.value(), VALID_DIGEST_SHA256.split_once(':').unwrap().1);
    }
}
