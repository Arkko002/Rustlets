#[path = "./device.rs"]
pub mod device;

use std::os::unix::net::UnixStream;
use zbus::dbus_proxy;
use zbus::export::zvariant::ObjectPath;
use zbus::Connection;
use zbus::Error;
use zbus::SignalReceiver;

use self::device::{Device, DeviceType};

pub struct PulesDbusClient<'a> {
    conn: Connection,
    reciver: SignalReceiver<'a, 'a>,
    pub sinks: Vec<Device<'a>>,
    pub sources: Vec<Device<'a>>,
}

#[dbus_proxy(
    default_service = "org.PulseAudio1",
    interface = "org.PulseAudio.ServerLookup1",
    default_path = "/org/pulseaudio/server_lookup1"
)]
trait PulseServer {
    #[dbus_proxy(property)]
    fn address(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    default_service = "org.PulseAudio.Core1",
    interface = "org.PulseAudio.Core1",
    default_path = "/org/pulseaudio/core1"
)]
trait PulseCore {
    #[dbus_proxy(property)]
    fn sinks(&self) -> zbus::Result<Vec<ObjectPath>>;
    #[dbus_proxy(property)]
    fn sources(&self) -> zbus::Result<Vec<ObjectPath>>;
}

#[dbus_proxy(
    default_service = "org.PulseAudio.Core1.Device",
    interface = "org.PulseAudio.Core1.Device"
)]
trait PulseDevice {
    #[dbus_proxy(property)]
    fn volume(&self) -> zbus::Result<Vec<u32>>;
    #[dbus_proxy(property)]
    fn mute(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    #[dbus_proxy(signal)]
    fn volume_updated(&self, old: Vec<u32>, new: Vec<u32>) -> Result<()>;
    #[dbus_proxy(signal)]
    fn mute_updated(&self, old: bool, new: bool) -> Result<()>;
}

impl<'a> PulesDbusClient<'a> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let adr: String = PulesDbusClient::get_server_address()?;

        let soc_stream = UnixStream::connect(adr)?;
        let conn = Connection::new_unix_client(soc_stream, false)?;

        let proxy = PulseCoreProxy::new(&conn)?;
        let reciver = SignalReceiver::new(conn);

        let sinks = PulesDbusClient::get_devices(&proxy, &conn, DeviceType::Sink)?;
        let sources = PulesDbusClient::get_devices(&proxy, &conn, DeviceType::Source)?;

        Ok(Self {
            conn,
            reciver,
            sinks,
            sources,
        })
    }

    fn get_server_address() -> Result<String, Box<dyn std::error::Error>> {
        let conn = Connection::new_session()?;

        let proxy = PulseServerProxy::new(&conn)?;

        let address_raw: String = proxy.address()?;
        let address_option: Option<String> = address_raw.split("=").last().map(String::from);

        match address_option {
            Some(adr) => Ok(adr),
            None => Err(Box::new(Error::Address(String::from(
                "Invalid socket address returned by ServerLookup",
            )))),
        }
    }

    fn get_devices(
        core_proxy: &PulseCoreProxy,
        conn: &Connection,
        device_type: DeviceType,
    ) -> Result<Vec<Device<'a>>, Box<dyn std::error::Error>> {
        let paths: Vec<ObjectPath>;
        match device_type {
            DeviceType::Sink => paths = core_proxy.get_property("Sinks")?,
            DeviceType::Source => paths = core_proxy.get_property("Sources")?,
        }

        let mut devices_list: Vec<Device> = Vec::new();
        for path in paths {
            let proxy_path = path.clone();
            let proxy = PulseDeviceProxy::new_for_path(&conn, &proxy_path)?;

            let name = proxy.name()?;
            let volume = proxy.volume()?;
            let mute = proxy.mute()?;

            let device = Device {
                path,
                device_type,
                name,
                volume,
                mute,
            };

            // TODO
            //proxy.connect_signal("VolumeUpdate", device.set_volume(volume));

            devices_list.push(device);
        }

        Ok(devices_list)
    }
}
