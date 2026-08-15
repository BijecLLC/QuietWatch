#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceKind {
    Input,
    Output,
    Loopback,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub kind: DeviceKind,
    pub is_default: bool,
}

impl AudioDevice {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        kind: DeviceKind,
        is_default: bool,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            is_default,
        }
    }
}
