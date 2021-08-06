mod pulse_dbus;

use crate::pulse_dbus::PulesDbusClient;

fn main() {
    let conn = PulesDbusClient::new().unwrap();
    conn.get_sinks().unwrap();
}
