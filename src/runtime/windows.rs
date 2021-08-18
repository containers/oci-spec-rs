use serde::{Deserialize, Serialize};
use std::collections::HashMap;

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option))
    )]
    /// Windows defines the runtime configuration for Windows based containers, including Hyper-V
    /// containers.
    struct Windows {
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// LayerFolders contains a list of absolute paths to directories containing image layers.
        layer_folders: Vec<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Devices are the list of devices to be mapped into the container.
        devices: Option<Vec<WindowsDevice>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Resources contains information for handling resource constraints for the container.
        resources: Option<WindowsResources>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// CredentialSpec contains a JSON object describing a group Managed Service Account (gMSA)
        /// specification.
        credential_spec: Option<HashMap<String, Option<serde_json::Value>>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Servicing indicates if the container is being started in a mode to apply a Windows Update
        /// servicing operation.
        servicing: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// IgnoreFlushesDuringBoot indicates if the container is being started in a mode where disk
        /// writes are not flushed during its boot process.
        ignore_flushes_during_boot: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// HyperV contains information for running a container with Hyper-V isolation.
        hyperv: Option<WindowsHyperV>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Network restriction configuration.
        network: Option<WindowsNetwork>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// WindowsDevice represents information about a host device to be mapped into the container.
    struct WindowsDevice {
        /// Device identifier: interface class GUID, etc..
        id: String,

        /// Device identifier type: "class", etc..
        id_type: String,
    }
);

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    struct WindowsResources {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Memory restriction configuration.
        memory: Option<WindowsMemoryResources>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// CPU resource restriction configuration.
        cpu: Option<WindowsCPUResources>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Storage restriction configuration.
        storage: Option<WindowsStorageResources>,
    }
);

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    /// WindowsMemoryResources contains memory resource management settings.
    struct WindowsMemoryResources {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Memory limit in bytes.
        limit: Option<u64>,
    }
);

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    /// WindowsCPUResources contains CPU resource management settings.
    struct WindowsCPUResources {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Number of CPUs available to the container.
        count: Option<u64>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// CPU shares (relative weight to other containers with cpu shares).
        shares: Option<u16>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies the portion of processor cycles that this container can use as a percentage times
        /// 100.
        maximum: Option<u16>,
    }
);

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    /// WindowsStorageResources contains storage resource management settings.
    struct WindowsStorageResources {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies maximum Iops for the system drive.
        iops: Option<u64>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Specifies maximum bytes per second for the system drive.
        bps: Option<u64>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Sandbox size specifies the minimum size of the system drive in bytes.
        sandbox_size: Option<u64>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// WindowsHyperV contains information for configuring a container to run with Hyper-V isolation.
    struct WindowsHyperV {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// UtilityVMPath is an optional path to the image used for the Utility VM.
        utility_vm_path: Option<String>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option))
    )]
    /// WindowsNetwork contains network settings for Windows containers.
    struct WindowsNetwork {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// List of HNS endpoints that the container should connect to.
        endpoint_list: Option<Vec<String>>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "allowUnqualifiedDNSQuery"
        )]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Specifies if unqualified DNS name resolution is allowed.
        allow_unqualified_dns_query: Option<bool>,

        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "DNSSearchList"
        )]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Comma separated list of DNS suffixes to use for name resolution.
        dns_search_list: Option<Vec<String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Name (ID) of the container that we will share with the network stack.
        network_shared_container_name: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// name (ID) of the network namespace that will be used for the container.
        network_namespace: Option<String>,
    }
);
