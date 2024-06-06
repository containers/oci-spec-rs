use crate::error::OciSpecError;

use derive_builder::Builder;
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// ContainerState represents the state of a container.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerState {
    /// Creating indicates that the container is being created,
    Creating,

    /// Created indicates that the runtime has finished the create operation,
    /// but the container exists but has not been run yet.
    Created,

    /// Running indicates that the container process has executed the
    /// user-specified program but has not exited
    Running,

    /// Paused indicates that the container exists, but all processes are paused.
    Paused,

    /// StateStopped indicates that the container process has exited,
    /// and does not have a created or running process.
    Stopped,

    /// Unknown
    #[default]
    Unknown,
}

impl Display for ContainerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerState::Creating => write!(f, "creating"),
            ContainerState::Created => write!(f, "created"),
            ContainerState::Running => write!(f, "running"),
            ContainerState::Paused => write!(f, "paused"),
            ContainerState::Stopped => write!(f, "stopped"),
            ContainerState::Unknown => write!(f, "unknown"),
        }
    }
}

/// State holds information about the runtime state of the container.
#[derive(
    Builder,
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Getters,
    MutGetters,
    Setters,
    PartialEq,
    Serialize,
)]
#[serde(rename_all = "camelCase")]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "OciSpecError")
)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct State {
    // version is the version of the specification that is supported.
    #[serde(default, rename = "ociVersion")]
    version: String,

    // id is the container ID
    #[serde(default)]
    id: String,

    // status is the runtime status of the container.
    #[serde(default)]
    status: ContainerState,

    // pid is the process ID for the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pid: Option<i32>,

    // bundle is the path to the container's bundle directory.
    #[serde(default)]
    bundle: String,

    // annotations are key values associated with the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    annotations: Option<HashMap<String, String>>,
}

/// SeccompFdName is the name of the seccomp notify file descriptor.
#[allow(dead_code)]
pub const SECCOMP_FD_NAME: &str = "seccompFd";

/// ContainerProcessState holds information about the state of a container process.
#[derive(
    Builder,
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Getters,
    MutGetters,
    Setters,
    PartialEq,
    Serialize,
)]
#[serde(rename_all = "camelCase")]
#[builder(
    default,
    pattern = "owned",
    setter(into, strip_option),
    build_fn(error = "OciSpecError")
)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ContainerProcessState {
    /// version is the version of the specification that is supported.
    #[serde(default, rename = "ociVersion")]
    version: String,

    /// fds is a string array containing the names of the file descriptors passed.
    /// The index of the name in this array corresponds to index of the file
    /// descriptor in the `SCM_RIGHTS` array.
    #[serde(default)]
    fds: Vec<String>,

    /// pid is the process ID as seen by the runtime.
    #[serde(default)]
    pid: i32,

    /// opaque metadata.
    #[serde(default)]
    metadata: Option<String>,

    /// state of the container.
    #[serde(default)]
    state: State,
}
