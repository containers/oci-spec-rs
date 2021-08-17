use caps::Capability;
use serde::{Deserialize, Serialize};

make_pub!(
    /// Process contains information to start a specific application inside the container.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option))
    )]
    struct Process {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Terminal creates an interactive terminal for the container.
        terminal: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// ConsoleSize specifies the size of the console.
        console_size: Option<Box>,

        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// User specifies user information for the process.
        user: User,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Args specifies the binary and arguments for the application to execute.
        args: Option<Vec<String>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// CommandLine specifies the full command line for the application to execute on Windows.
        command_line: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Env populates the process environment for the process.
        env: Option<Vec<String>>,

        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Cwd is the current working directory for the process and must be relative to the
        /// container's root.
        cwd: String,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Capabilities are Linux capabilities that are kept for the process.
        capabilities: Option<LinuxCapabilities>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Rlimits specifies rlimit options to apply to the process.
        rlimits: Option<Vec<LinuxRlimit>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// NoNewPrivileges controls whether additional privileges could be gained by processes in the
        /// container.
        no_new_privileges: Option<bool>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// ApparmorProfile specifies the apparmor profile for the container.
        apparmor_profile: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// Specify an oom_score_adj for the container.
        oom_score_adj: Option<i32>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// SelinuxLabel specifies the selinux context that the container process is run as.
        selinux_label: Option<String>,
    }
);

// Default impl for processes in the container
impl Default for Process {
    fn default() -> Self {
        Process {
            // Creates an interactive terminal for container by default
            terminal: true.into(),
            // Gives default console size of 0, 0
            console_size: Default::default(),
            // Gives process a uid and gid of 0 (root)
            user: Default::default(),
            // By default executes sh command, giving user shell
            args: vec!["sh".to_string()].into(),
            // Sets linux default enviroment for binaries and default xterm emulator
            env: vec![
                "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin".into(),
                "TERM=xterm".into(),
            ]
            .into(),
            // Sets cwd of process to the container root by default
            cwd: "/".into(),
            // By default does not allow process to gain additional privileges
            no_new_privileges: true.into(),
            // Empty String, no default apparmor
            apparmor_profile: Default::default(),
            // Empty String, no default selinux
            selinux_label: Default::default(),
            // See impl Default for LinuxCapabilities
            capabilities: Some(Default::default()),
            // Sets the default maximum of 1024 files the process can open
            // This is the same as the linux kernel default
            rlimits: vec![LinuxRlimit {
                typ: LinuxRlimitType::RlimitNofile,
                hard: 1024,
                soft: 1024,
            }]
            .into(),
            oom_score_adj: None,
            command_line: None,
        }
    }
}

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    /// Box specifies dimensions of a rectangle. Used for specifying the size of a console.
    struct Box {
        #[serde(default)]
        /// Height is the vertical dimension of a box.
        height: u64,

        #[serde(default)]
        /// Width is the horizontal dimension of a box.
        width: u64,
    }
);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Available rlimit types (see <https://man7.org/linux/man-pages/man2/getrlimit.2.html>)
pub enum LinuxRlimitType {
    /// Limit in seconds of the amount of CPU time that the process can consume.
    RlimitCpu,

    /// Maximum size in bytes of the files that the process creates.
    RlimitFsize,

    /// Maximum size of the process's data segment (init data, uninit data and heap) in bytes.
    RlimitData,

    /// Maximum size of the proces stack in bytes.
    RlimitStack,

    /// Maximum size of a core dump file in bytes.
    RlimitCore,

    /// Limit on the process's resident set (the number of virtual pages resident in RAM).
    RlimitRss,

    /// Limit on number of threads for the real uid calling processes.
    RlimitNproc,

    /// One greator than the maximum number of file descritors that one process may open.
    RlimitNofile,

    /// Maximum number of bytes of memory that may be locked into RAM.
    RlimitMemlock,

    /// Maximum size of the process's virtual memory(address space) in bytes.
    RlimitAs,

    /// Limit on the number of locks and leases for the process.
    RlimitLocks,

    /// Limit on number of signals that may be queued for the process.
    RlimitSigpending,

    /// Limit on the number of bytes that can be allocated for POSIX message queue.
    RlimitMsgqueue,

    /// Specifies a ceiling to which the process's nice value can be raised.
    RlimitNice,

    /// Specifies a ceiling on the real-time priority.
    RlimitRtprio,

    /// This is a limit (in microseconds) on the amount of CPU time that a process scheduled under
    /// a real-time scheduling policy may consume without making a blocking system call.
    RlimitRttime,
}

impl Default for LinuxRlimitType {
    fn default() -> Self {
        Self::RlimitCpu
    }
}

make_pub!(
    #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::CopyGetters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get_copy = "pub")
    )]
    /// RLimit types and restrictions.
    struct LinuxRlimit {
        #[serde(rename = "type")]
        /// Type of Rlimit to set
        typ: LinuxRlimitType,

        #[serde(default)]
        /// Hard limit for specified type
        hard: u64,

        #[serde(default)]
        /// Soft limit for specified type
        soft: u64,
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
    /// User id (uid) and group id (gid) tracks file permssions.
    struct User {
        #[serde(default)]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// UID is the user id.
        uid: u32,

        #[serde(default)]
        #[cfg_attr(feature = "builder", getset(get_copy = "pub"))]
        /// GID is the group id.
        gid: u32,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// AdditionalGids are additional group ids set for the container's process.
        additional_gids: Option<Vec<u32>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "builder", getset(get = "pub"))]
        /// Username is the user name.
        username: Option<String>,
    }
);

make_pub!(
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[cfg_attr(
        feature = "builder",
        derive(derive_builder::Builder, getset::Getters),
        builder(default, pattern = "owned", setter(into, strip_option)),
        getset(get = "pub")
    )]
    /// LinuxCapabilities specifies the list of allowed capabilities that are kept for a process.
    /// <http://man7.org/linux/man-pages/man7/capabilities.7.html>
    struct LinuxCapabilities {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Bounding is the set of capabilities checked by the kernel.
        bounding: Option<Vec<Capability>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Effective is the set of capabilities checked by the kernel.
        effective: Option<Vec<Capability>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Inheritable is the capabilities preserved across execve.
        inheritable: Option<Vec<Capability>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        /// Permitted is the limiting superset for effective capabilities.
        permitted: Option<Vec<Capability>>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        //// Ambient is the ambient set of capabilities that are kept.
        ambient: Option<Vec<Capability>>,
    }
);

// Default container's linux capabilities:
// CAP_AUDIT_WRITE gives container ability to write to linux audit logs,
// CAP_KILL gives container ability to kill non root processes
// CAP_NET_BIND_SERVICE allows container to bind to ports below 1024
impl Default for LinuxCapabilities {
    fn default() -> Self {
        let audit_write = Capability::CAP_AUDIT_WRITE;
        let cap_kill = Capability::CAP_KILL;
        let net_bind = Capability::CAP_NET_BIND_SERVICE;
        let default_vec = vec![audit_write, cap_kill, net_bind];
        LinuxCapabilities {
            bounding: default_vec.clone().into(),
            effective: default_vec.clone().into(),
            inheritable: default_vec.clone().into(),
            permitted: default_vec.clone().into(),
            ambient: default_vec.into(),
        }
    }
}
