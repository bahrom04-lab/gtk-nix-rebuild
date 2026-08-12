use gettextrs::gettext;
use gtk::prelude::ApplicationExt;
use gtk::{gio, glib};
use gtk_nix_rebuild::config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use relm4::{RelmApp, gtk, main_application};
use tracing::error;

use gtk_nix_rebuild::ui::load::load;
use gtk_nix_rebuild::ui::window::{App, AppInit};

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

fn setup_locale() {
    // setup gettext
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("GTK Rust Template"));
    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);
    let data = res
        .lookup_data(
            "/io/github/bahrom04-lab/style.css",
            gio::ResourceLookupFlags::NONE,
        )
        .unwrap();
    relm4::set_global_css(&glib::GString::from_utf8_checked(data.to_vec()).unwrap());
}

fn main() {
    // Enable logging
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    gtk::init().unwrap();
    gtk::Window::set_default_icon_name(APP_ID);
    setup_locale();

    let app = main_application();
    app.set_resource_base_path(Some("/io/github/bahrom04-lab/"));

    // let mut actions = RelmActionGroup::<AppActionGroup>::new();
    // let quit_action = {
    //     let app = app.clone();
    //     RelmAction::<QuitAction>::new_stateless(move |_| {
    //         app.quit();
    //     })
    // };
    // actions.add_action(quit_action);
    // actions.register_for_main_application();
    // app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    let app = RelmApp::from_app(app);
    match load() {
        Ok(load) => app.run::<App>(AppInit { load }),
        Err(e) => {
            error!("Failed to load: {}", e);
            std::process::exit(1);
        }
    }
}
