//! [OCI image spec](https://github.com/opencontainers/image-spec) types and definitions.

mod annotations;
mod config;
mod descriptor;
mod index;
mod manifest;
mod version;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub use annotations::*;
pub use config::*;
pub use descriptor::*;
pub use index::*;
pub use manifest::*;
pub use version::*;

/// Media types used by OCI image format spec. Values MUST comply with RFC 6838,
/// including the naming requirements in its section 4.2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaType {
    /// MediaType Descriptor specifies the media type for a content descriptor.
    Descriptor,
    /// MediaType LayoutHeader specifies the media type for the oci-layout.
    LayoutHeader,
    /// MediaType ImageManifest specifies the media type for an image manifest.
    ImageManifest,
    /// MediaType ImageIndex specifies the media type for an image index.
    ImageIndex,
    /// MediaType ImageLayer is the media type used for layers referenced by the
    /// manifest.
    ImageLayer,
    /// MediaType ImageLayerGzip is the media type used for gzipped layers
    /// referenced by the manifest.
    ImageLayerGzip,
    /// MediaType ImageLayerZstd is the media type used for zstd compressed
    /// layers referenced by the manifest.
    ImageLayerZstd,
    /// MediaType ImageLayerNonDistributable is the media type for layers
    /// referenced by the manifest but with distribution restrictions.
    ImageLayerNonDistributable,
    /// MediaType ImageLayerNonDistributableGzip is the media type for
    /// gzipped layers referenced by the manifest but with distribution
    /// restrictions.
    ImageLayerNonDistributableGzip,
    /// MediaType ImageLayerNonDistributableZstd is the media type for zstd
    /// compressed layers referenced by the manifest but with distribution
    /// restrictions.
    ImageLayerNonDistributableZstd,
    /// MediaType ImageConfig specifies the media type for the image
    /// configuration.
    ImageConfig,
    /// MediaType not specified by OCI image format.
    Other(String),
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Descriptor => write!(f, "application/vnd.oci.descriptor"),
            Self::LayoutHeader => write!(f, "application/vnd.oci.layout.header.v1+json"),
            Self::ImageManifest => write!(f, "application/vnd.oci.image.manifest.v1+json"),
            Self::ImageIndex => write!(f, "application/vnd.oci.image.index.v1+json"),
            Self::ImageLayer => write!(f, "application/vnd.oci.image.layer.v1.tar"),
            Self::ImageLayerGzip => write!(f, "application/vnd.oci.image.layer.v1.tar+gzip"),
            Self::ImageLayerZstd => write!(f, "application/vnd.oci.image.layer.v1.tar+zstd"),
            Self::ImageLayerNonDistributable => {
                write!(f, "application/vnd.oci.image.layer.nondistributable.v1.tar")
            }
            Self::ImageLayerNonDistributableGzip => write!(
                f,
                "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip"
            ),
            Self::ImageLayerNonDistributableZstd => write!(
                f,
                "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd"
            ),
            Self::ImageConfig => write!(f, "application/vnd.oci.image.config.v1+json"),
            Self::Other(media_type) => write!(f, "{}", media_type),
        }
    }
}

impl Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let media_type = format!("{}", self);
        media_type.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let stringified_type = String::deserialize(deserializer)?;
        let media_type = match stringified_type.as_str() {
            "application/vnd.oci.descriptor" => MediaType::Descriptor,
            "application/vnd.oci.layout.header.v1+json" => MediaType::LayoutHeader,
            "application/vnd.oci.image.manifest.v1+json" => MediaType::ImageManifest,
            "application/vnd.oci.image.index.v1+json" => MediaType::ImageIndex,
            "application/vnd.oci.image.layer.v1.tar" => MediaType::ImageLayer,
            "application/vnd.oci.image.layer.v1.tar+gzip" => MediaType::ImageLayerGzip,
            "application/vnd.oci.image.layer.v1.tar+zstd" => MediaType::ImageLayerZstd,
            "application/vnd.oci.image.layer.nondistributable.v1.tar" => {
                MediaType::ImageLayerNonDistributable
            }
            "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip" => {
                MediaType::ImageLayerNonDistributableGzip
            }
            "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd" => {
                MediaType::ImageLayerNonDistributableZstd
            }
            "application/vnd.oci.image.config.v1+json" => MediaType::ImageConfig,
            _ => MediaType::Other(stringified_type),
        };

        Ok(media_type)
    }
}

/// Name of the target operating system
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Os {
    AIX,
    Android,
    Darwin,
    DragonFlyBSD,
    FreeBSD,
    Hurd,
    Illumos,
    #[allow(non_camel_case_types)]
    iOS,
    Js,
    Linux,
    Nacl,
    NetBSD,
    OpenBSD,
    Plan9,
    Solaris,
    Windows,
    #[allow(non_camel_case_types)]
    zOS,
    Other(String),
}

impl From<&str> for Os {
    fn from(os: &str) -> Self {
        match os {
            "aix" => Os::AIX,
            "android" => Os::Android,
            "darwin" => Os::Darwin,
            "dragonfly" => Os::DragonFlyBSD,
            "freebsd" => Os::FreeBSD,
            "hurd" => Os::Hurd,
            "illumos" => Os::Illumos,
            "ios" => Os::iOS,
            "js" => Os::Js,
            "linux" => Os::Linux,
            "nacl" => Os::Nacl,
            "netbsd" => Os::NetBSD,
            "openbsd" => Os::OpenBSD,
            "plan9" => Os::Plan9,
            "solaris" => Os::Solaris,
            "windows" => Os::Windows,
            "zos" => Os::zOS,
            name => Os::Other(name.to_owned()),
        }
    }
}

impl Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = match self {
            Os::AIX => "aix",
            Os::Android => "android",
            Os::Darwin => "darwin",
            Os::DragonFlyBSD => "dragonfly",
            Os::FreeBSD => "freebsd",
            Os::Hurd => "hurd",
            Os::Illumos => "illumos",
            Os::iOS => "ios",
            Os::Js => "js",
            Os::Linux => "linux",
            Os::Nacl => "nacl",
            Os::NetBSD => "netbsd",
            Os::OpenBSD => "openbsd",
            Os::Plan9 => "plan9",
            Os::Solaris => "solaris",
            Os::Windows => "windows",
            Os::zOS => "zos",
            Os::Other(name) => name,
        };

        write!(f, "{}", print)
    }
}

impl Serialize for Os {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let os = format!("{}", self);
        os.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Os {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let os = String::deserialize(deserializer)?;
        Ok(os.as_str().into())
    }
}
