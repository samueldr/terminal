use gtk::{Application, glib};
use vte4::prelude::*;

mod misc;
mod terminal;

const APP_ID: &str = "com.samueldr.terminal";

const LOGGER: glib::GlibLogger = glib::GlibLogger::new(
    glib::GlibLoggerFormat::Plain,
    glib::GlibLoggerDomain::CrateTarget,
);

fn start(app: &gtk::Application) {
    terminal::create(app);
}

fn main() -> glib::ExitCode {
    log::set_logger(&LOGGER).expect("logger already set");
    log::set_max_level(log::LevelFilter::Debug);

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(start);

    app.run()
}
