use gio::prelude::*;
use gtk4::{prelude::*, Application};
use once_cell::sync::Lazy;
use std::env;

mod launcher;
mod ui;
mod actions;
mod loader;

use loader::{util::Config, Loader};


static CONFIG: Lazy<Config> = Lazy::new(|| {
    Loader::load_config()
});



#[tokio::main]
async fn main() {
    Loader::load_resources();
    let sherlock_flags = Loader::load_flags();

    env::set_var("GSK_RENDERER", &CONFIG.appearance.gsk_renderer);
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());

    application.connect_activate(move |app| {
        let launchers = Loader::load_launchers(&sherlock_flags);
        Loader::load_icon_theme(&CONFIG.appearance.icon_paths);
        Loader::load_css(&sherlock_flags);

        let app_clone = app.clone();
        let mut window = ui::window::window(&app_clone);
        window = ui::search::search(window, launchers);
        window.show();
    });

    application.run();
}



