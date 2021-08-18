use std::{collections::HashMap, fs, path::Path};

use super::Descriptor;

use serde::{Deserialize, Serialize};
use anyhow::Result;

/// The image index is a higher-level manifest which points to specific image manifests, 
/// ideal for one or more platforms. While the use of an image index is OPTIONAL for 
/// image providers, image consumers SHOULD be prepared to process them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageIndex {
    /// This REQUIRED property specifies the image manifest schema version. 
    /// For this version of the specification, this MUST be 2 to ensure backward 
    /// compatibility with older versions of Docker. The value of this field will 
    /// not change. This field MAY be removed in a future version of the specification.
    schema_version: u32,
    /// This property is reserved for use, to maintain compatibility. When used, 
    /// this field contains the media type of this document, which differs from 
    /// the descriptor use of mediaType.
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<String>,
    /// This REQUIRED property contains a list of manifests for specific platforms. 
    /// While this property MUST be present, the size of the array MAY be zero.
    manifests: Vec<Descriptor>,
    /// This OPTIONAL property contains arbitrary metadata for the image index. 
    /// This OPTIONAL property MUST use the annotation rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<HashMap<String, String>>
}

impl ImageIndex {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ImageIndex> {
        let path = path.as_ref();
        let index_file = fs::File::open(path)?;
        let index = serde_json::from_reader(&index_file)?;
        Ok(index)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_load_index() {
        let index_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_files/image/index.json");
        let result = ImageIndex::load(index_path);
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
