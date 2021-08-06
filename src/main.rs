pub mod pulse_dbus;

fn main() {
    let conn = pulse_dbus::start_dbus_client().unwrap();
}
