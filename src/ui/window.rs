use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
    actions::{RelmAction, RelmActionGroup},
    adw,
    gtk::{
        self,
        prelude::*,
        {gio, glib},
    },
    main_application,
};
use std::{collections::HashMap, convert::identity};

use crate::{modules::ModuleOption, ui::{
    about::AboutDialog,
    load::{LoadOutput, ReloadOutput},
}};
use crate::ui::{load::reload, rebuild::rebuild_dialog::RebuildInput};
use crate::{
    config::{APP_ID, PROFILE},
    ui::rebuild::rebuild_dialog::{RebuildInit, RebuildModel},
};
use nix_data::config::configfile::NixDataConfig;

pub struct App {
    config: NixDataConfig,
    rebuild_dialog: Controller<RebuildModel>,
    // error_dialog: Controller<ErrorDialogModel>,
    moduleconfig: String,

    current_config: HashMap<String, ModuleOption>,
    modified_config: HashMap<String, ModuleOption>,
}

pub struct AppInit {
    pub load: LoadOutput,
}

#[derive(Debug)]
pub enum AppMsg {
    // OpenModulePage,
    // CloseModulePage,
    // SetModuleOption,
    // ApplyChanges,
    Rebuild,
    Reload,
    Quit,
}

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = AppInit;
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About GTK Rust Template" => AboutAction,
            }
        }
    }

    view! {
        main_window = adw::ApplicationWindow::new(&main_application()) {
            set_visible: true,

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                glib::Propagation::Stop
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/net/bleur/GtkRustTemplate/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                    Some("devel")
                } else {
                    None
                },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    }
                },

                gtk::Label {
                    set_label: "Hello world!",
                    add_css_class: "title-header",
                    set_vexpand: true,
                },

                gtk::Button {
                    set_label: "rebuild",
                    add_css_class: "suggested-action",
                    connect_clicked[sender] => move |_a| {
                        sender.input(AppMsg::Rebuild)
                    }
                },
            }

        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let LoadOutput {
            config,
            moduleconfig,
            modulepath,
            flakepath,
            modules,
            current_config,
        } = init.load;

        let rebuild_dialog = RebuildModel::builder()
            .transient_for(&root)
            .launch(RebuildInit {
                flakepath,
                modulepath,
                generations: config.generations,
            })
            .forward(sender.input_sender(), identity);

        let model = Self {
            config,
            moduleconfig,
            rebuild_dialog,
            current_config,
            modified_config: HashMap::new(),
        };

        let widgets = view_output!();

        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            RelmAction::<AboutAction>::new_stateless(move |_| {
                AboutDialog::builder().launch(()).detach();
            })
        };

        actions.add_action(shortcuts_action);
        actions.add_action(about_action);
        actions.register_for_widget(&widgets.main_window);

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
            AppMsg::Rebuild => self.rebuild_dialog.emit(RebuildInput::Rebuild(
                self.modified_config.clone(),
                self.moduleconfig.clone(),
            )),
            AppMsg::Reload => match reload(&self.config) {
                Ok(ReloadOutput {
                    modules,
                    current_config,
                    moduleconfig,
                }) => {
                    self.current_config = current_config;
                    self.moduleconfig = moduleconfig;
                    self.modified_config.clear();

                    // self.main_leaflet.set_visible_child(&self.main_box);
                    // self.modulepage.emit(ModulePageInput::ShowApply(false));
                }
                Err(e) => {
                    // self.error_dialog.emit(ErrorDialogInput::Show(
                    //     "Failed to reload current module configuration".to_string(),
                    //     e.to_string(),
                    // ));
                }
            },
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
