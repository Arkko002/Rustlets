#[path = "../pulse_dbus.rs"]
mod pulse_dbus;

use crate::pulse_dbus::PulesDbusClient;
use gtk::ffi::{GtkStatusIcon, GtkStatusIconClass};
use gtk::prelude::*;
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

        let sink_list = gtk::ListBox::new();
        for ele in &model.client.sinks {
            let btn = gtk::Button::new();
            btn.set_label(&ele.name);
            sink_list.add(&btn);
        }

        let source_list = gtk::ListBox::new();
        for ele in &model.client.sources {
            let btn = gtk::Button::new();
            btn.set_label(&ele.name);
            source_list.add(&btn);
        }

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

fn main() {
    Win::run(()).unwrap();
}
