//! [OCI runtime spec](https://github.com/opencontainers/runtime-spec) types and definitions.

use path_clean;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::error::{oci_error, Result};

mod capability;
mod hooks;
mod linux;
mod miscellaneous;
mod process;
mod solaris;
mod test;
mod version;
mod vm;
mod windows;

// re-export for ease of use
pub use capability::*;
pub use hooks::*;
pub use linux::*;
pub use miscellaneous::*;
pub use process::*;
pub use solaris::*;
pub use version::*;
pub use vm::*;
pub use windows::*;

make_pub!(
    /// Base configuration for the container.
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters, getset::Setters),
        builder(
            default,
            pattern = "owned",
            setter(into, strip_option),
            build_fn(error = "crate::error::OciSpecError")
        ),
        getset(get = "pub", set = "pub")
    )]
    struct Spec {
        #[serde(default, rename = "ociVersion")]
        ///  MUST be in SemVer v2.0.0 format and specifies the version of the
        /// Open Container Initiative  Runtime Specification with which
        /// the bundle complies. The Open Container Initiative
        ///  Runtime Specification follows semantic versioning and retains
        /// forward and backward  compatibility within major versions.
        /// For example, if a configuration is compliant with
        ///  version 1.1 of this specification, it is compatible with all
        /// runtimes that support any 1.1  or later release of this
        /// specification, but is not compatible with a runtime that supports
        ///  1.0 and not 1.1.
        version: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies the container's root filesystem. On Windows, for Windows
        /// Server Containers, this field is REQUIRED. For Hyper-V
        /// Containers, this field MUST NOT be set.
        ///
        /// On all other platforms, this field is REQUIRED.
        root: Option<Root>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies additional mounts beyond `root`. The runtime MUST mount
        /// entries in the listed order.
        ///
        /// For Linux, the parameters are as documented in
        /// [`mount(2)`](http://man7.org/linux/man-pages/man2/mount.2.html) system call man page. For
        /// Solaris, the mount entry corresponds to the 'fs' resource in the
        /// [`zonecfg(1M)`](http://docs.oracle.com/cd/E86824_01/html/E54764/zonecfg-1m.html) man page.
        mounts: Option<Vec<Mount>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies the container process. This property is REQUIRED when
        /// [`start`](https://github.com/opencontainers/runtime-spec/blob/master/runtime.md#start) is
        /// called.
        process: Option<Process>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies the container's hostname as seen by processes running
        /// inside the container. On Linux, for example, this will
        /// change the hostname in the container [UTS namespace](http://man7.org/linux/man-pages/man7/namespaces.7.html). Depending on your
        /// [namespace
        /// configuration](https://github.com/opencontainers/runtime-spec/blob/master/config-linux.md#namespaces),
        /// the container UTS namespace may be the runtime UTS namespace.
        hostname: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Hooks allow users to specify programs to run before or after various
        /// lifecycle events. Hooks MUST be called in the listed order.
        /// The state of the container MUST be passed to hooks over
        /// stdin so that they may do work appropriate to the current state of
        /// the container.
        hooks: Option<Hooks>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Annotations contains arbitrary metadata for the container. This
        /// information MAY be structured or unstructured. Annotations
        /// MUST be a key-value map. If there are no annotations then
        /// this property MAY either be absent or an empty map.
        ///
        /// Keys MUST be strings. Keys MUST NOT be an empty string. Keys SHOULD
        /// be named using a reverse domain notation - e.g.
        /// com.example.myKey. Keys using the org.opencontainers
        /// namespace are reserved and MUST NOT be used by subsequent
        /// specifications. Runtimes MUST handle unknown annotation keys
        /// like any other unknown property.
        ///
        /// Values MUST be strings. Values MAY be an empty string.
        annotations: Option<HashMap<String, String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Linux is platform-specific configuration for Linux based containers.
        linux: Option<Linux>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Solaris is platform-specific configuration for Solaris based
        /// containers.
        solaris: Option<Solaris>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Windows is platform-specific configuration for Windows based
        /// containers.
        windows: Option<Windows>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// VM specifies configuration for Virtual Machine based containers.
        vm: Option<VM>,
    }
);

// This gives a basic boilerplate for Spec that can be used calling
// Default::default(). The values given are similar to the defaults seen in
// docker and runc, it creates a containerized shell! (see respective types
// default impl for more info)
impl Default for Spec {
    fn default() -> Self {
        Spec {
            // Defaults to most current oci version
            version: String::from("1.0.2-dev"),
            process: Some(Default::default()),
            root: Some(Default::default()),
            // Defaults hostname as youki
            hostname: "youki".to_string().into(),
            mounts: get_default_mounts().into(),
            // Defaults to empty metadata
            annotations: Some(Default::default()),
            linux: Some(Default::default()),
            hooks: None,
            solaris: None,
            windows: None,
            vm: None,
        }
    }
}

impl Spec {
    /// Load a new `Spec` from the provided JSON file `path`.
    /// # Errors
    /// This function will return an [OciSpecError::Io](crate::OciSpecError::Io)
    /// if the spec does not exist or an
    /// [OciSpecError::SerDe](crate::OciSpecError::SerDe) if it is invalid.
    /// # Example
    /// ``` no_run
    /// use oci_spec::runtime::Spec;
    ///
    /// let spec = Spec::load("config.json").unwrap();
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = fs::File::open(path)?;
        let s = serde_json::from_reader(&file)?;
        Ok(s)
    }

    /// Save a `Spec` to the provided JSON file `path`.
    /// # Errors
    /// This function will return an [OciSpecError::Io](crate::OciSpecError::Io)
    /// if a file cannot be created at the provided path or an
    /// [OciSpecError::SerDe](crate::OciSpecError::SerDe) if the spec cannot be
    /// serialized.
    /// # Example
    /// ``` no_run
    /// use oci_spec::runtime::Spec;
    ///
    /// let mut spec = Spec::load("config.json").unwrap();
    /// spec.save("my_config.json").unwrap();
    /// ```
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let file = fs::File::create(path)?;
        serde_json::to_writer(&file, self)?;
        Ok(())
    }

    #[cfg(not(feature = "builder"))]
    /// Canonicalize the `root.path` of the `Spec` for the provided `bundle`.
    pub fn canonicalize_rootfs<P: AsRef<Path>>(&mut self, bundle: P) -> Result<()> {
        let root = self
            .root
            .as_mut()
            .ok_or_else(|| oci_error("no root path provided for canonicalization"))?;
        root.path = Self::canonicalize_path(bundle, &root.path)?;
        Ok(())
    }

    #[cfg(feature = "builder")]
    /// Canonicalize the `root.path` of the `Spec` for the provided `bundle`.
    pub fn canonicalize_rootfs<P: AsRef<Path>>(&mut self, bundle: P) -> Result<()> {
        let root = self
            .root
            .as_ref()
            .ok_or_else(|| oci_error("no root path provided for canonicalization"))?;
        let path = Self::canonicalize_path(bundle, root.path())?;
        self.root = Some(
            RootBuilder::default()
                .path(path)
                .readonly(root.readonly())
                .build()
                .map_err(|_| oci_error("failed to set canonicalized root"))?,
        );
        Ok(())
    }

    fn canonicalize_path<B, P>(bundle: B, path: P) -> Result<PathBuf>
    where
        B: AsRef<Path>,
        P: AsRef<Path>,
    {
        Ok(if path.as_ref().is_absolute() {
            fs::canonicalize(path.as_ref())?
        } else {
            let canonical_bundle_path = fs::canonicalize(&bundle)?;
            fs::canonicalize(canonical_bundle_path.join(path.as_ref()))?
        })
    }

    #[cfg(not(feature = "builder"))]
    /// Converts the given spec file into one that should work with rootless containers,
    /// by removing incompatible options and adding others that are needed.
    pub fn set_for_rootless(&mut self, uid: u32, gid: u32) -> Result<()> {
        let linux = self.linux.as_mut().unwrap();
        linux.resources = None;

        // Remove network from the default spec
        let mut namespaces = vec![];
        for ns in linux.namespaces.as_ref().unwrap().iter() {
            if ns.typ != LinuxNamespaceType::Network && ns.typ != LinuxNamespaceType::User {
                namespaces.push(ns.clone());
            }
        }
        // Add user namespace
        namespaces.push(LinuxNamespace {
            typ: LinuxNamespaceType::User,
            path: None,
        });
        linux.namespaces = Some(namespaces);

        linux.uid_mappings = Some(vec![LinuxIdMapping {
            host_id: uid,
            container_id: 0,
            size: 1,
        }]);
        linux.gid_mappings = Some(vec![LinuxIdMapping {
            host_id: gid,
            container_id: 0,
            size: 1,
        }]);

        // Fix the mounts
        let mut mounts = vec![];
        for mount in self.mounts.as_ref().unwrap().iter() {
            let dest = mount.destination.clone();
            if path_clean::clean(dest.as_path().to_str().unwrap()) == "/sys" {
                mounts.push(Mount {
                    destination: PathBuf::from("/sys"),
                    source: Some(PathBuf::from("/sys")),
                    typ: Some("none".to_string()),
                    options: Some(vec![
                        "rbind".to_string(),
                        "nosuid".to_string(),
                        "noexec".to_string(),
                        "nodev".to_string(),
                        "ro".to_string(),
                    ]),
                });
            } else {
                let options: Vec<String> = mount
                    .options
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|&o| !o.starts_with("gid=") && !o.starts_with("uid="))
                    .map(|o| o.to_string())
                    .collect();
                let mut t = mount.clone();
                t.options = Some(options);
                mounts.push(t);
            }
        }
        self.mounts = Some(mounts);
        Ok(())
    }

    #[cfg(feature = "builder")]
    /// Converts the given spec file into one that should work with rootless containers,
    /// by removing incompatible options and adding others that are needed.
    pub fn set_for_rootless(&mut self, uid: u32, gid: u32) -> Result<()> {
        let linux = self.linux.as_mut().unwrap();
        linux.set_resources(None);

        // Remove network from the default spec
        let mut namespaces = vec![];
        for ns in linux.namespaces().as_ref().unwrap().iter() {
            if ns.typ() != LinuxNamespaceType::Network && ns.typ() != LinuxNamespaceType::User {
                namespaces.push(ns.clone());
            }
        }
        // Add user namespace
        namespaces.push(
            LinuxNamespaceBuilder::default()
                .typ(LinuxNamespaceType::User)
                .build()?,
        );
        linux.set_namespaces(Some(namespaces));

        linux.set_uid_mappings(Some(vec![LinuxIdMappingBuilder::default()
            .host_id(uid)
            .container_id(0_u32)
            .size(1_u32)
            .build()?]));
        linux.set_gid_mappings(Some(vec![LinuxIdMappingBuilder::default()
            .host_id(gid)
            .container_id(0_u32)
            .size(1_u32)
            .build()?]));

        // Fix the mounts
        let mut mounts = vec![];
        for mount in self.mounts().as_ref().unwrap().iter() {
            let dest = mount.destination().clone();
            if path_clean::clean(dest.as_path().to_str().unwrap()) == "/sys" {
                let mount = MountBuilder::default()
                    .destination(PathBuf::from("/sys"))
                    .source(PathBuf::from("/sys"))
                    .typ("none".to_string())
                    .options(Some(vec![
                        "rbind".to_string(),
                        "nosuid".to_string(),
                        "noexec".to_string(),
                        "nodev".to_string(),
                        "ro".to_string(),
                    ]))
                    .build()?;
                mounts.push(mount);
            } else {
                let options: Vec<String> = mount
                    .options()
                    .as_ref()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|&o| !o.starts_with("gid=") && !o.starts_with("uid="))
                    .map(|o| o.to_string())
                    .collect();
                let mut t = mount.clone();
                t.set_options(Some(options));
                mounts.push(t);
            }
        }
        self.set_mounts(Some(mounts));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "builder")]
    fn test_canonicalize_rootfs() {
        let rootfs_name = "rootfs";
        let bundle = tempfile::tempdir().expect("failed to create tmp test bundle dir");
        let rootfs_absolute_path = bundle.path().join(rootfs_name);
        assert!(
            rootfs_absolute_path.is_absolute(),
            "rootfs path is not absolute path"
        );
        fs::create_dir_all(&rootfs_absolute_path).expect("failed to create the testing rootfs");
        {
            // Test the case with absolute path
            let mut spec = SpecBuilder::default()
                .root(
                    RootBuilder::default()
                        .path(rootfs_absolute_path.clone())
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap();

            spec.canonicalize_rootfs(bundle.path())
                .expect("failed to canonicalize rootfs");

            assert_eq!(
                &rootfs_absolute_path,
                spec.root.expect("no root in spec").path()
            );
        }
        {
            // Test the case with relative path
            let mut spec = SpecBuilder::default()
                .root(RootBuilder::default().path(rootfs_name).build().unwrap())
                .build()
                .unwrap();

            spec.canonicalize_rootfs(bundle.path())
                .expect("failed to canonicalize rootfs");

            assert_eq!(
                &rootfs_absolute_path,
                spec.root.expect("no root in spec").path()
            );
        }
    }

    #[test]
    #[cfg(not(feature = "builder"))]
    fn test_canonicalize_rootfs() {
        let rootfs_name = "rootfs";
        let bundle = tempfile::tempdir().expect("failed to create tmp test bundle dir");
        let rootfs_absolute_path = bundle.path().join(rootfs_name);
        assert!(
            rootfs_absolute_path.is_absolute(),
            "rootfs path is not absolute path"
        );
        fs::create_dir_all(&rootfs_absolute_path).expect("failed to create the testing rootfs");
        {
            // Test the case with absolute path
            let mut spec = Spec {
                root: Root {
                    path: rootfs_absolute_path.clone(),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };

            spec.canonicalize_rootfs(bundle.path())
                .expect("failed to canonicalize rootfs");

            assert_eq!(
                rootfs_absolute_path,
                spec.root.expect("no root in spec").path
            );
        }
        {
            // Test the case with relative path
            let mut spec = Spec {
                root: Root {
                    path: PathBuf::from(rootfs_name),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };

            spec.canonicalize_rootfs(bundle.path())
                .expect("failed to canonicalize rootfs");

            assert_eq!(
                rootfs_absolute_path,
                spec.root.expect("no root in spec").path
            );
        }
    }

    #[test]
    fn test_load_save() {
        let spec = Spec {
            ..Default::default()
        };
        let test_dir = tempfile::tempdir().expect("failed to create tmp test dir");
        let spec_path = test_dir.into_path().join("config.json");

        // Test first save the default config, and then load the saved config.
        // The before and after should be the same.
        spec.save(&spec_path).expect("failed to save spec");
        let loaded_spec = Spec::load(&spec_path).expect("failed to load the saved spec.");
        assert_eq!(
            spec, loaded_spec,
            "The saved spec is not the same as the loaded spec"
        );
    }

    #[test]
    #[cfg(not(feature = "builder"))]
    fn test_set_for_rootless() {
        let mut spec = Spec {
            ..Default::default()
        };
        spec.set_for_rootless(1, 2)
            .expect("failed to set spec for rootless");
        assert_eq!(spec.linux.as_ref().unwrap().uid_mappings.is_some(), true);
        assert_eq!(spec.linux.as_ref().unwrap().gid_mappings.is_some(), true);
    }

    #[test]
    #[cfg(feature = "builder")]
    fn test_set_for_rootless() {
        let mut spec = Spec {
            ..Default::default()
        };
        spec.set_for_rootless(1, 2)
            .expect("failed to set spec for rootless");
        assert_eq!(
            spec.linux().as_ref().unwrap().uid_mappings().is_some(),
            true
        );
        assert_eq!(
            spec.linux().as_ref().unwrap().gid_mappings().is_some(),
            true
        );
    }
}
