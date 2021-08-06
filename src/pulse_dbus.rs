use std::{os::unix::net::UnixStream, time::Duration};

use zbus::{Connection, Error, Proxy};

//#[dbus_proxy]
//trait Address {
//fn notify(
//&self,
//address: &str,
//) -> zbus::Result<u32>;
//}

struct PulesDbusClient {
    conn: Connection,
}

impl PulesDbusClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let adr: String = PulesDbusClient::get_server_address()?;

        let soc_stream = UnixStream::connect(adr)?;
        Ok(Self {
            conn: Connection::new_unix_client(soc_stream, false)?,
        })
    }

    fn get_server_address() -> Result<String, Box<dyn std::error::Error>> {
        let conn = Connection::new_session()?;

        let proxy = Proxy::new(
            &conn,
            "org.PulseAudio1",
            "/org/pulseaudio/server_lookup1",
            "org.PulseAudio.ServerLookup1",
        )?;

        let address_raw: String = proxy.get_property("Address")?;
        let address_option: Option<String> = address_raw.split("=").last().map(String::from);

        match address_option {
            Some(adr) => Ok(adr),
            None => Err(Box::new(Error::Address(String::from(
                "Invalid socket address returned by ServerLookup",
            )))),
        }
    }

    pub fn get_sinks(&self) {}
}
