#[cfg(test)]
use super::*;

#[test]
fn serialize_and_deserialize_spec() {
    let spec: Spec = Default::default();
    let json_string = serde_json::to_string(&spec).unwrap();
    let new_spec = serde_json::from_str(&json_string).unwrap();
    assert_eq!(spec, new_spec);
}

#[test]
fn test_linux_device_cgroup_to_string() {
    cfg_if::cfg_if!(
        if #[cfg(feature = "builder")] {
            let ldc = LinuxDeviceCgroupBuilder::default().
                allow(true).
                typ(LinuxDeviceType::B).
                access("rwm".to_string()).
                build().expect("build device cgroup");
        } else {
            let ldc = LinuxDeviceCgroup {
                allow: true,
                typ: Some(LinuxDeviceType::B),
                major: None,
                minor: None,
                access: Some("rwm".into()),
            };
        }
    );
    assert_eq!(ldc.to_string(), "b *:* rwm");

    cfg_if::cfg_if!(
        if #[cfg(feature = "builder")] {
            let ldc = LinuxDeviceCgroupBuilder::default()
                .allow(true)
                .typ(LinuxDeviceType::B)
                .major(1)
                .minor(9)
                .access("rwm".to_string())
                .build().expect("build device cgroup");
        } else {
            let ldc = LinuxDeviceCgroup {
                allow: true,
                typ: Some(LinuxDeviceType::B),
                major: Some(1),
                minor: Some(9),
                access: Some("rwm".into()),
            };
        }
    );
    assert_eq!(ldc.to_string(), "b 1:9 rwm");
}
