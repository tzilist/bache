use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceName(String);

impl fmt::Display for InstanceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl InstanceName {
    pub fn new(instance_name: String) -> Self {
        Self(instance_name)
    }
}

impl From<String> for InstanceName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for InstanceName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
