use zbus::export::zvariant::ObjectPath;

#[derive(Copy, Clone)]
pub enum DeviceType {
    Sink,
    Source,
}

pub struct Device<'a> {
    pub path: ObjectPath<'a>,
    pub device_type: DeviceType,
    pub volume: Vec<u32>,
    pub mute: bool,
    pub name: String,
}

impl<'a> Device<'a> {
    /// Set the device's mute.
    pub fn set_mute(&mut self, mute: bool) {
        self.mute = mute;
    }

    /// Set the device's volume.
    pub fn set_volume(&mut self, volume: Vec<u32>) {
        self.volume = volume;
    }
}
