use crate::support;
use imgui::{im_str, sys, ComboBox, Condition, ImString, MenuItem, Ui, Window, WindowFlags};
use std::ffi::CString;
#[derive(Default, Debug)]
pub struct State {
    show_provider_form: bool,
    show_navigator: bool,
    selected_provider: usize,
    selected_provider_old: usize,
    quit: bool,
    quits: bool,
    key: ImString,
    key_secret: ImString,
    bucket: ImString,
    region: ImString,
}

impl State {
    pub fn new() -> State {
        State {
            show_provider_form: true,
            show_navigator: false,
            selected_provider: 0,
            selected_provider_old: 0,
            quit: false,
            quits: false,
            key: ImString::with_capacity(256),
            key_secret: ImString::with_capacity(256),
            bucket: ImString::with_capacity(256),
            region: ImString::with_capacity(256),
        }
    }

    pub fn quit(&self) -> bool {
        self.quit
    }
}

pub fn init_and_run() {
    let system = support::init("Invikta");
    let state = State::new();
    let window_title = im_str!("Invikta");

    system.main_loop(
        |_, ui, state| {
            Window::new(window_title)
                .size([1024.0, 768.0], Condition::FirstUseEver)
                .flags(
                    WindowFlags::NO_COLLAPSE
                        | WindowFlags::MENU_BAR
                        | WindowFlags::NO_RESIZE
                        | WindowFlags::NO_TITLE_BAR
                        | WindowFlags::NO_MOVE,
                )
                .position([0.0, 0.0], Condition::Always)
                .build(ui, || {
                    show_menu(ui, state);
                    if state.show_provider_form {
                        show_provider_form(ui, state);
                    }
                });
        },
        state,
    );
}

fn show_provider_form<'a>(ui: &Ui<'a>, state: &mut State) {
    ComboBox::new(im_str!("Cloud Provider")).build_simple_string(
        &ui,
        &mut state.selected_provider,
        &[im_str!("AWS"), im_str!("ssss")],
    );
    if state.selected_provider != state.selected_provider_old {
        state.selected_provider_old = state.selected_provider;
        state.key.clear();
        state.key_secret.clear();
        state.bucket.clear();
        state.region.clear();
    }
    match state.selected_provider {
        0 => {
            ui.input_text(im_str!("Access Key"), &mut state.key).build();
            ui.input_text(im_str!("Secret Access Key"), &mut state.key_secret)
                .build();
            ui.input_text(im_str!("Region"), &mut state.region).build();
            ui.input_text(im_str!("Bucket"), &mut state.bucket).build();
        }
        1 => {
            ui.input_text(im_str!("GAccess Key"), &mut state.key)
                .build();
            ui.input_text(im_str!("GSecret Access Key"), &mut state.key_secret)
                .build();
            ui.input_text(im_str!("GRegion"), &mut state.region).build();
            ui.input_text(im_str!("GBucket"), &mut state.bucket).build();
        }
        _ => {}
    }

    if ui.button(im_str!("Connect"), [120.0, 0.0]) {
        state.show_provider_form = false;
    }
}

fn show_menu<'a>(ui: &Ui<'a>, state: &mut State) {
    if let Some(menu_bar) = ui.begin_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            MenuItem::new(im_str!("Quit")).build_with_ref(ui, &mut state.quit);
            menu.end(ui)
        }
        menu_bar.end(ui)
    }
}
