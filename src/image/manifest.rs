use std::{collections::HashMap, fs, path::Path};

use super::Descriptor;
use anyhow::Result;
use serde::{Deserialize, Serialize};

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(pattern = "owned", setter(into, strip_option))
    )]
    /// Unlike the image index, which contains information about a set of images
    /// that can span a variety of architectures and operating systems, an image
    /// manifest provides a configuration and set of layers for a single container
    /// image for a specific architecture and operating system.
    struct ImageManifest {
        /// This REQUIRED property specifies the image manifest schema version.
        /// For this version of the specification, this MUST be 2 to ensure backward
        /// compatibility with older versions of Docker. The value of this field will
        /// not change. This field MAY be removed in a future version of the specification.
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        schema_version: u32,
        /// This property is reserved for use, to maintain compatibility. When used,
        /// this field contains the media type of this document, which differs from
        /// the descriptor use of mediaType.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        media_type: Option<String>,
        /// This REQUIRED property references a configuration object for a container,
        /// by digest. Beyond the descriptor requirements, the value has the following additional restrictions:
        /// The media type descriptor property has additional restrictions for config.
        /// Implementations MUST support at least the following media types:
        /// - application/vnd.oci.image.config.v1+json
        /// Manifests concerned with portability SHOULD use one of the above media types.
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        config: Descriptor,
        /// Each item in the array MUST be a descriptor. The array MUST have the base
        /// layer at index 0. Subsequent layers MUST then follow in stack order (i.e. from layers[0]
        /// to layers[len(layers)-1]). The final filesystem layout MUST match the result of applying
        /// the layers to an empty directory. The ownership, mode, and other attributes of the initial
        /// empty directory are unspecified.
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        layers: Vec<Descriptor>,
        /// This OPTIONAL property contains arbitrary metadata for the image manifest.
        /// This OPTIONAL property MUST use the annotation rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        annotations: Option<HashMap<String, String>>,
    }
);

impl ImageManifest {
    /// Attempts to load an image manifest.
    /// # Errors
    /// This function will return an error if the image manifest does
    /// not exist or is invalid.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageManifest;
    ///
    /// let image_manifest = ImageManifest::load("my-manifest.json").unwrap();
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ImageManifest> {
        let path = path.as_ref();
        let manifest_file = fs::File::open(path)?;
        let manifest = serde_json::from_reader(&manifest_file)?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_load_manifest() {
        let manifest_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/data/manifest.json");
        let result = ImageManifest::load(manifest_path);
        assert!(result.is_ok());
    }
}
