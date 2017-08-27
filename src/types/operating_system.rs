use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum OperatingSystem {
    Android,
    iOS,
    Desktop,
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            OperatingSystem::Android => "Android",
            OperatingSystem::iOS => "iOS",
            OperatingSystem::Desktop => "Desktop",
        };

        write!(f, "{}", printable)
    }
}