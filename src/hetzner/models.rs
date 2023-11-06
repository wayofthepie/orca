use std::fmt::Display;

pub enum HCloudServerType {
    CAX11,
}

impl Display for HCloudServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HCloudServerType::CAX11 => "cax11",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum HCloudLocation {
    FSN1,
}

impl Display for HCloudLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HCloudLocation::FSN1 => "fsn1",
        };
        write!(f, "{}", s)
    }
}
