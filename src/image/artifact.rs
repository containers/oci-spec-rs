use super::{Descriptor, MediaType};
use crate::error::{OciSpecError, Result};
use derive_builder::Builder;
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
};

#[derive(
    Builder, Clone, Debug, Deserialize, Eq, Getters, MutGetters, Setters, PartialEq, Serialize,
)]
#[serde(rename_all = "camelCase")]
#[builder(
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "OciSpecError")
)]
/// The OCI Artifact manifest describes content addressable artifacts
/// in order to store them along side container images in a registry.
pub struct ArtifactManifest {
    /// This property MUST be used and contain the media type
    /// `application/vnd.oci.artifact.manifest.v1+json`.
    #[getset(get = "pub")]
    #[builder(default = "MediaType::ArtifactManifest")]
    #[builder(setter(skip))]
    media_type: MediaType,

    /// This property SHOULD be used and contain
    /// the mediaType of the referenced artifact.
    /// If defined, the value MUST comply with RFC 6838,
    /// including the naming requirements in its section 4.2,
    /// and MAY be registered with IANA.
    #[getset(get = "pub", set = "pub")]
    artifact_type: MediaType,

    /// This OPTIONAL property is an array of objects and each item
    /// in the array MUST be a descriptor. Each descriptor represents
    /// an artifact of any IANA mediaType. The list MAY be ordered
    /// for certain artifact types like scan results.
    #[getset(get_mut = "pub", get = "pub", set = "pub")]
    #[builder(default)]
    blobs: Vec<Descriptor>,

    /// This OPTIONAL property specifies a descriptor of another manifest.
    /// This value, used by the referrers API, indicates a relationship
    /// to the specified manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get = "pub", set = "pub")]
    #[builder(default)]
    subject: Option<Descriptor>,

    /// This OPTIONAL property contains additional metadata for the artifact
    /// manifest. This OPTIONAL property MUST use the annotation rules.
    /// See Pre-Defined Annotation Keys. Annotations MAY be used to filter
    /// the response from the referrers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[getset(get_mut = "pub", get = "pub", set = "pub")]
    #[builder(default)]
    annotations: Option<HashMap<String, String>>,
}

impl ArtifactManifest {
    /// Attempts to load an image manifest from a file.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::Io] if the file does not exist
    /// - [OciSpecError::SerDe] if the image manifest cannot be deserialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        crate::from_file(path)
    }

    /// Attempts to load an image manifest from a stream.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the manifest cannot be deserialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    /// use std::fs::File;
    ///
    /// let reader = File::open("manifest.json").unwrap();
    /// let artifact_manifest = ArtifactManifest::from_reader(reader).unwrap();
    /// ```
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        crate::from_reader(reader)
    }

    /// Attempts to write an image manifest to a file as JSON. If the file already exists, it
    /// will be overwritten.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image manifest cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// artifact_manifest.to_file("my-manifest.json").unwrap();
    /// ```
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        crate::to_file(&self, path, false)
    }

    /// Attempts to write an image manifest to a file as pretty printed JSON. If the file already exists, it
    /// will be overwritten.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image manifest cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// artifact_manifest.to_file_pretty("my-manifest.json").unwrap();
    /// ```
    pub fn to_file_pretty<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        crate::to_file(&self, path, true)
    }

    /// Attempts to write an image manifest to a stream as JSON.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image manifest cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// let mut writer = Vec::new();
    /// artifact_manifest.to_writer(&mut writer);
    /// ```
    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        crate::to_writer(&self, writer, false)
    }

    /// Attempts to write an image manifest to a stream as pretty printed JSON.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image manifest cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// let mut writer = Vec::new();
    /// artifact_manifest.to_writer_pretty(&mut writer);
    /// ```
    pub fn to_writer_pretty<W: Write>(&self, writer: &mut W) -> Result<()> {
        crate::to_writer(&self, writer, true)
    }

    /// Attempts to write an image manifest to a string as JSON.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image configuration cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// let json_str = artifact_manifest.to_string().unwrap();
    /// ```
    pub fn to_string(&self) -> Result<String> {
        crate::to_string(&self, false)
    }

    /// Attempts to write an image manifest to a string as pretty printed JSON.
    ///
    /// # Errors
    ///
    /// - [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the image configuration cannot be serialized.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use oci_spec::image::ArtifactManifest;
    ///
    /// let artifact_manifest = ArtifactManifest::from_file("manifest.json").unwrap();
    /// let json_str = artifact_manifest.to_string_pretty().unwrap();
    /// ```
    pub fn to_string_pretty(&self) -> Result<String> {
        crate::to_string(&self, true)
    }
}

