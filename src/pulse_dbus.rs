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
    volume: u32,
    mute: bool,
}

pub struct Source<'a> {
    path: ObjectPath<'a>,
    name: String,
    volume: u32,
    mute: bool,
}

impl<'a> PulesDbusClient<'a> {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let adr: String = PulesDbusClient::get_server_address()?;

        let soc_stream = UnixStream::connect(adr)?;
        let conn = Connection::new_unix_client(soc_stream, false)?;

        let proxy = Proxy::new(
            &conn,
            "org.PulseAudio1",
            "/org/pulseaudio/server_lookup1",
            "org.PulseAudio.ServerLookup1",
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

    // TODO Generics for devices
    fn get_sinks(
        proxy: &Proxy,
        conn: &Connection,
    ) -> Result<Vec<Sink<'a>>, Box<dyn std::error::Error>> {
        let sink_paths: Vec<ObjectPath> = proxy.get_property("Sinks")?;

        let sink_vec: Vec<Sink> = Vec::new();
        for sink_path in sink_paths {
            let proxy = Proxy::new(
                &conn,
                "org.PulseAudio.Core1.Device",
                &sink_path,
                "org.PulseAudio.Core1.Device",
            )?;

            sink_vec.push(Sink {
                path: sink_path,
                name: proxy.get_property("Name")?,
                volume: proxy.get_property("Volume")?,
                mute: proxy.get_property("Mute")?,
            })
        }

        Ok(sink_vec)
    }

    fn get_sources(
        proxy: &Proxy,
        conn: &Connection,
    ) -> Result<Vec<Source<'a>>, Box<dyn std::error::Error>> {
        let sources_path: Vec<ObjectPath> = proxy.get_property("Sources")?;

        let source_vec: Vec<Source> = Vec::new();
        for source_path in sources_path {
            let proxy = Proxy::new(
                &conn,
                "org.PulseAudio.Core1.Device",
                &source_path,
                "org.PulseAudio.Core1.Device",
            )?;

            source_vec.push(Source {
                path: source_path,
                name: proxy.get_property("Name")?,
                volume: proxy.get_property("Volume")?,
                mute: proxy.get_property("Mute")?,
            })
        }

        Ok(source_vec)
    }
}
