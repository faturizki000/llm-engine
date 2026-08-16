/// Model compatibility checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatInfo {
    pub major: u64,
    pub minor: u64,
    pub compatible: bool,
}

impl CompatInfo {
    pub fn new(major: u64, minor: u64, compatible: bool) -> Self {
        Self {
            major,
            minor,
            compatible,
        }
    }

    pub fn from_version(version: &str) -> Self {
        let mut parts = version.split('.');
        let major = parts.next().unwrap_or("0").parse().unwrap_or(0);
        let minor = parts.next().unwrap_or("0").parse().unwrap_or(0);
        Self::new(major, minor, true)
    }
}
