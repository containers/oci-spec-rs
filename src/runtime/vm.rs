use serde::{Deserialize, Serialize};
use std::path::PathBuf;

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// VM contains information for virtual-machine-based containers.
    struct VM {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Hypervisor specifies hypervisor-related configuration for
        /// virtual-machine-based containers.
        hypervisor: Option<VMHypervisor>,

        /// Kernel specifies kernel-related configuration for
        /// virtual-machine-based containers.
        kernel: VMKernel,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Image specifies guest image related configuration for
        /// virtual-machine-based containers.
        image: Option<VMImage>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// VMHypervisor contains information about the hypervisor to use for a
    /// virtual machine.
    struct VMHypervisor {
        /// Path is the host path to the hypervisor used to manage the virtual
        /// machine.
        path: PathBuf,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Parameters specifies parameters to pass to the hypervisor.
        parameters: Option<Vec<String>>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// VMKernel contains information about the kernel to use for a virtual
    /// machine.
    struct VMKernel {
        /// Path is the host path to the kernel used to boot the virtual
        /// machine.
        path: PathBuf,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Parameters specifies parameters to pass to the kernel.
        parameters: Option<Vec<String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// InitRD is the host path to an initial ramdisk to be used by the
        /// kernel.
        initrd: Option<String>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// VMImage contains information about the virtual machine root image.
    struct VMImage {
        /// Path is the host path to the root image that the VM kernel would
        /// boot into.
        path: PathBuf,

        /// Format is the root image format type (e.g. "qcow2", "raw", "vhd",
        /// etc).
        format: String,
    }
);
