use std::os::unix::net::UnixStream;
use zbus::{export::zvariant::ObjectPath, Connection, Error, Proxy};

pub struct PulesDbusClient<'a> {
    conn: Connection,
    pub sinks: Vec<Sink<'a>>,
    pub sources: Vec<Source<'a>>,
}

pub struct Sink<'a> {
    path: ObjectPath<'a>,
    name: String,
    volume: Vec<u32>,
    mute: bool,
}

pub struct Source<'a> {
    path: ObjectPath<'a>,
    name: String,
    volume: Vec<u32>,
    mute: bool,
}

impl<'a> PulesDbusClient<'a> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let adr: String = PulesDbusClient::get_server_address()?;

        let soc_stream = UnixStream::connect(adr)?;
        let conn = Connection::new_unix_client(soc_stream, false)?;

        let proxy = Proxy::new(
            &conn,
            "org.PulseAudio.Core1",
            "/org/pulseaudio/core1",
            "org.PulseAudio.Core1",
        )?;

        let sinks = PulesDbusClient::get_sinks(&proxy, &conn)?;
        let sources = PulesDbusClient::get_sources(&proxy, &conn)?;

        Ok(Self {
            conn,
            sinks,
            sources,
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

    fn get_sinks(
        core_proxy: &Proxy,
        conn: &Connection,
    ) -> Result<Vec<Sink<'a>>, Box<dyn std::error::Error>> {
        let sink_paths: Vec<ObjectPath> = core_proxy.get_property("Sinks")?;

        let mut sink_vec: Vec<Sink> = Vec::new();
        for sink_path in sink_paths {
            let proxy_path = sink_path.clone();
            let proxy = Proxy::new(
                &conn,
                "org.PulseAudio.Core1.Device",
                &proxy_path,
                "org.PulseAudio.Core1.Device",
            )?;

            let name: String = proxy.get_property("Name")?;
            let volume: Vec<u32> = proxy.get_property("Volume")?;
            let mute: bool = proxy.get_property("Mute")?;

            sink_vec.push(Sink {
                path: sink_path,
                name,
                volume,
                mute,
            });
        }

        Ok(sink_vec)
    }

    fn get_sources(
        core_proxy: &Proxy,
        conn: &Connection,
    ) -> Result<Vec<Source<'a>>, Box<dyn std::error::Error>> {
        let source_paths: Vec<ObjectPath> = core_proxy.get_property("Sources")?;

        let mut source_vec: Vec<Source> = Vec::new();
        for source_path in source_paths {
            let proxy_path = source_path.clone();
            let proxy = Proxy::new(
                &conn,
                "org.PulseAudio.Core1.Device",
                &proxy_path,
                "org.PulseAudio.Core1.Device",
            )?;

            let name: String = proxy.get_property("Name")?;
            let volume: Vec<u32> = proxy.get_property("Volume")?;
            let mute: bool = proxy.get_property("Mute")?;

            source_vec.push(Source {
                path: source_path,
                name,
                volume,
                mute,
            })
        }

        Ok(source_vec)
    }
}
