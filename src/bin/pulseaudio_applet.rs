#[path = "../pulse_dbus.rs"]
mod pulse_dbus;

use crate::pulse_dbus::PulesDbusClient;
use gtk::{prelude::*, Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.rustlets.PulesAudio")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("PulseAudio Rustlet")
            .build();

        win.show_all();
    });

    app.run();
}
