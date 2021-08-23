use std::{collections::HashMap, fs, path::Path};

use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Result;

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// The image configuration is associated with an image and describes some
    /// basic information about the image such as date created, author, as
    /// well as execution/runtime configuration like its entrypoint, default
    /// arguments, networking, and volumes.
    struct ImageConfiguration {
        /// An combined date and time at which the image was created,
        /// formatted as defined by [RFC 3339, section 5.6.](https://tools.ietf.org/html/rfc3339#section-5.6)
        #[serde(skip_serializing_if = "Option::is_none")]
        created: Option<String>,
        /// Gives the name and/or email address of the person or entity
        /// which created and is responsible for maintaining the image.
        #[serde(skip_serializing_if = "Option::is_none")]
        author: Option<String>,
        /// The CPU architecture which the binaries in this
        /// image are built to run on. Configurations SHOULD use, and
        /// implementations SHOULD understand, values listed in the Go
        /// Language document for [GOARCH](https://golang.org/doc/install/source#environment).
        architecture: String,
        /// The name of the operating system which the image is built to run on.
        /// Configurations SHOULD use, and implementations SHOULD understand,
        /// values listed in the Go Language document for [GOOS](https://golang.org/doc/install/source#environment).
        os: String,
        /// This OPTIONAL property specifies the version of the operating
        /// system targeted by the referenced blob. Implementations MAY refuse
        /// to use manifests where os.version is not known to work with
        /// the host OS version. Valid values are
        /// implementation-defined. e.g. 10.0.14393.1066 on windows.
        #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
        os_version: Option<String>,
        /// This OPTIONAL property specifies an array of strings,
        /// each specifying a mandatory OS feature. When os is windows, image
        /// indexes SHOULD use, and implementations SHOULD understand
        /// the following values:
        /// - win32k: image requires win32k.sys on the host (Note: win32k.sys is
        ///   missing on Nano Server)
        #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
        os_features: Option<Vec<String>>,
        /// The variant of the specified CPU architecture. Configurations SHOULD
        /// use, and implementations SHOULD understand, variant values
        /// listed in the [Platform Variants](https://github.com/opencontainers/image-spec/blob/main/image-index.md#platform-variants) table.
        #[serde(skip_serializing_if = "Option::is_none")]
        variant: Option<String>,
        /// The execution parameters which SHOULD be used as a base when
        /// running a container using the image. This field can be None, in
        /// which case any execution parameters should be specified at
        /// creation of the container.
        #[serde(skip_serializing_if = "Option::is_none")]
        config: Option<Config>,
        /// The rootfs key references the layer content addresses used by the
        /// image. This makes the image config hash depend on the
        /// filesystem hash.
        rootfs: RootFs,
        /// Describes the history of each layer. The array is ordered from first
        /// to last.
        history: Vec<History>,
    }
);

impl ImageConfiguration {
    /// Attempts to load an image configuration.
    /// # Errors
    /// This function will return an [OciSpecError::Io](crate::OciSpecError::Io)
    /// if the image configuration does not exist or an
    /// [OciSpecError::SerDe](crate::OciSpecError::SerDe) if it is invalid.
    /// # Example
    /// ``` no_run
    /// use oci_spec::image::ImageConfiguration;
    ///
    /// let image_config = ImageConfiguration::load("my-config.json").unwrap();
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ImageConfiguration> {
        let path = path.as_ref();
        let file = fs::File::open(path)?;
        let image = serde_json::from_reader(file)?;
        Ok(image)
    }
}

impl Default for ImageConfiguration {
    fn default() -> Self {
        Self {
            created: Default::default(),
            author: Default::default(),
            architecture: "amd64".to_owned(),
            os: "linux".to_owned(),
            os_version: Default::default(),
            os_features: Default::default(),
            variant: Default::default(),
            config: Default::default(),
            rootfs: Default::default(),
            history: Default::default(),
        }
    }
}

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "PascalCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// The execution parameters which SHOULD be used as a base when
    /// running a container using the image.
    struct Config {
        /// The username or UID which is a platform-specific
        /// structure that allows specific control over which
        /// user the process run as. This acts as a default
        /// value to use when the value is not specified when
        /// creating a container. For Linux based systems, all
        /// of the following are valid: user, uid, user:group,
        /// uid:gid, uid:group, user:gid. If group/gid is not
        /// specified, the default group and supplementary
        /// groups of the given user/uid in /etc/passwd from
        /// the container are applied.
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<String>,
        /// A set of ports to expose from a container running this
        /// image. Its keys can be in the format of: port/tcp, port/udp,
        /// port with the default protocol being tcp if not specified.
        /// These values act as defaults and are merged with any
        /// specified when creating a container.
        #[serde(
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_as_vec",
            serialize_with = "serialize_as_map"
        )]
        exposed_ports: Option<Vec<String>>,
        /// Entries are in the format of VARNAME=VARVALUE. These
        /// values act as defaults and are merged with any
        /// specified when creating a container.
        #[serde(skip_serializing_if = "Option::is_none")]
        env: Option<Vec<String>>,
        /// A list of arguments to use as the command to execute
        /// when the container starts. These values act as defaults
        /// and may be replaced by an entrypoint specified when
        /// creating a container.
        #[serde(skip_serializing_if = "Option::is_none")]
        entrypoint: Option<Vec<String>>,
        /// Default arguments to the entrypoint of the container.
        /// These values act as defaults and may be replaced by any
        /// specified when creating a container. If an Entrypoint
        /// value is not specified, then the first entry of the Cmd
        /// array SHOULD be interpreted as the executable to run.
        #[serde(skip_serializing_if = "Option::is_none")]
        cmd: Option<Vec<String>>,
        /// A set of directories describing where the process is
        /// likely to write data specific to a container instance.
        #[serde(
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_as_vec",
            serialize_with = "serialize_as_map"
        )]
        volumes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Sets the current working directory of the entrypoint process
        /// in the container. This value acts as a default and may be
        /// replaced by a working directory specified when creating
        /// a container.
        working_dir: Option<String>,
        /// The field contains arbitrary metadata for the container.
        /// This property MUST use the annotation rules.
        #[serde(skip_serializing_if = "Option::is_none")]
        labels: Option<HashMap<String, String>>,
        /// The field contains the system call signal that will be
        /// sent to the container to exit. The signal can be a signal
        /// name in the format SIGNAME, for instance SIGKILL or SIGRTMIN+3.
        #[serde(skip_serializing_if = "Option::is_none")]
        stop_signal: Option<String>,
    }
);

// Some fields of the image configuration are a json serialization of a
// Go map[string]struct{} leading to the following json:
// {
//    "ExposedPorts": {
//       "8080/tcp": {},
//       "443/tcp": {},
//    }
// }
// Instead we treat this as a list
#[derive(Deserialize, Serialize)]
struct GoMapSerde {}

fn deserialize_as_vec<'de, D>(deserializer: D) -> std::result::Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<HashMap<String, GoMapSerde>>::deserialize(deserializer)?;
    if let Some(data) = opt {
        let vec: Vec<String> = data.keys().cloned().collect();
        return Ok(Some(vec));
    }

    Ok(None)
}

fn serialize_as_map<S>(
    target: &Option<Vec<String>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match target {
        Some(values) => {
            let map: HashMap<_, _> = values.iter().map(|v| (v, GoMapSerde {})).collect();
            let mut map_ser = serializer.serialize_map(Some(map.len()))?;
            for (key, value) in map {
                map_ser.serialize_entry(key, &value)?;
            }
            map_ser.end()
        }
        _ => unreachable!(),
    }
}

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// RootFs references the layer content addresses used by the image.
    struct RootFs {
        /// MUST be set to layers.
        #[serde(rename = "type")]
        typ: String,
        /// An array of layer content hashes (DiffIDs), in order
        /// from first to last.
        diff_ids: Vec<String>,
    }
);

impl Default for RootFs {
    fn default() -> Self {
        Self {
            typ: "layers".to_owned(),
            diff_ids: Default::default(),
        }
    }
}

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option))
    )]
    /// Describes the history of a layer.
    struct History {
        /// A combined date and time at which the layer was created,
        /// formatted as defined by [RFC 3339, section 5.6.](https://tools.ietf.org/html/rfc3339#section-5.6).
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        created: Option<String>,
        /// The author of the build point.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        author: Option<String>,
        /// The command which created the layer.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        created_by: Option<String>,
        /// A custom message set when creating the layer.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        comment: Option<String>,
        /// This field is used to mark if the history item created
        /// a filesystem diff. It is set to true if this history item
        /// doesn't correspond to an actual layer in the rootfs section
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        empty_layer: Option<bool>,
    }
);

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_load_image() {
        let config = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test/data/config.json");
        let result = ImageConfiguration::load(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serialize_config() {
        let config = Config {
            user: Some("me".to_owned()),
            env: Some(vec![
                "SHELL=/bin/bash".to_owned(),
                "XDG_RUNTIME_DIR=/run/user/1000".to_owned(),
            ]),
            entrypoint: None,
            cmd: None,
            working_dir: None,
            labels: None,
            stop_signal: None,
            exposed_ports: Some(vec!["tpc/8080".to_owned(), "udp/8080".to_owned()]),
            volumes: Some(vec!["/a/b/c".to_owned(), "/b/c/g".to_owned()]),
        };

        let _ = serde_json::to_string(&config).expect("failed to serialize");
    }
}
