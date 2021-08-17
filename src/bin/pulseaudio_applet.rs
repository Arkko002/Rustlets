#[path = "../pulse_dbus/pulse_dbus_client.rs"]
mod pulse_dbus;

#[path = "../pulse_dbus/device.rs"]
mod device;

use crate::pulse_dbus::device::Device;
use crate::pulse_dbus::PulesDbusClient;
use gtk::ffi::{GtkStatusIcon, GtkStatusIconClass};
use gtk::{prelude::*, Adjustment};
use gtk::{Inhibit, Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

struct Model<'a> {
    client: PulesDbusClient<'a>,
}
#[derive(Msg)]
enum Msg {
    MuteChanged,
    VolumeChanged,
    SinkDetected,
    SourceDetected,
    Quit,
}

struct Win<'a> {
    model: Model<'a>,
    window: Window,
}

impl<'a> Win<'a> {}

impl<'a> Update for Win<'a> {
    type Model = Model<'a>;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        Model {
            client: PulesDbusClient::new().unwrap(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::MuteChanged => todo!(),
            Msg::VolumeChanged => todo!(),
            Msg::SinkDetected => todo!(),
            Msg::SourceDetected => todo!(),
        }
    }
}

impl<'a> Widget for Win<'a> {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let notebook = gtk::Notebook::new();

        let sink_list = create_device_listbox(&model.client.sinks).unwrap();
        let source_list = create_device_listbox(&model.client.sources).unwrap();

        notebook.add(&sink_list);
        notebook.add(&source_list);
        notebook.set_tab_label(&sink_list, Some(&gtk::Label::new(Some("Sinks"))));
        notebook.set_tab_label(&source_list, Some(&gtk::Label::new(Some("Sources"))));

        window.add(&notebook);

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        Win { model, window }
    }
}

fn create_device_listbox(devices: &Vec<pulse_dbus::device::Device>) -> Option<gtk::ListBox> {
    let device_listbox = gtk::ListBox::new();
    for ele in devices {
        let device_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // TODO Human readable names
        let name_label = gtk::Label::new(Some(&ele.name));

        let vol_scale = gtk::Scale::new(
            gtk::Orientation::Horizontal,
            Some(&Adjustment::new(
                ele.volume[0].into(),
                0.0,
                100.0,
                1.0,
                1.0,
                1.0,
            )),
        );

        // TODO Mute icon as label
        let mute_btn = gtk::Button::new();
        mute_btn.set_label("Mute");

        device_box.add(&name_label);
        device_box.add(&vol_scale);
        device_box.add(&mute_btn);

        device_listbox.add(&device_box);
    }

    Some(device_listbox)
}

fn main() {
    Win::run(()).unwrap();
}
