use anyhow::Error;
use flume::{Receiver, Sender};
use imgui::{Ui, *};
use indexmap::IndexMap;
use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::runtime::Handle;
use wfd::DialogParams;

use crate::{
    event_handler::{MainEvent, SaveGame},
    save_data::{mass_effect_2::Me2SaveGame, mass_effect_3::Me3SaveGame, SaveData},
};

mod support;

static NOTIFICATION_TIME: f64 = 1.5;

// States
#[derive(Default)]
struct ErrorState {
    errors: Vec<Error>,
    is_opened: bool,
}

#[derive(Default)]
struct NotificationState {
    string: ImString,
    close_time: f64,
}

#[derive(Default)]
struct State {
    errors: ErrorState,
    notification: Option<NotificationState>,
}

// Events
pub enum UiEvent {
    Error(Error),
    Notification(&'static str),
    OpenedSave(SaveGame),
}

// UI
pub fn run(event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>, handle: Handle) {
    let mut state = State::default();
    let mut save_game = None;

    // UI
    let system = support::init("Trilogy Save Editor", 1000.0, 700.0);
    system.main_loop(move |_, ui| {
        handle.block_on(async {
            rx.try_iter().for_each(|ui_event| match ui_event {
                UiEvent::Error(err) => {
                    state.errors.errors.push(err);
                    state.errors.is_opened = true;
                }
                UiEvent::Notification(string) => {
                    state.notification = Some(NotificationState {
                        string: ImString::new(string),
                        close_time: ui.time() + NOTIFICATION_TIME,
                    })
                }
                UiEvent::OpenedSave(opened_save_game) => {
                    save_game = Some(opened_save_game);
                }
            });

            let ui = Gui::new(ui, &event_addr);
            ui.draw(&mut state, &mut save_game).await;
        });
    });
}

pub struct Gui<'a> {
    ui: &'a Ui<'a>,
    event_addr: Sender<MainEvent>,
    bg_count: AtomicUsize,
}

impl<'a> Gui<'a> {
    fn new(ui: &'a Ui<'a>, event_addr: &Sender<MainEvent>) -> Self {
        Self { ui, event_addr: Sender::clone(event_addr), bg_count: AtomicUsize::new(0) }
    }

    async fn draw(&self, state: &mut State, save_game: &mut Option<SaveGame>) {
        let ui = self.ui;

        // Main window
        let window = Window::new(im_str!("###main"))
            .size(ui.io().display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .menu_bar(true)
            .collapsible(false);

        // Pop on drop
        let _colors = self
            .style_colors(match save_game {
                None => Theme::MassEffect3,
                Some(SaveGame::MassEffect1) => Theme::MassEffect1,
                Some(SaveGame::MassEffect2(_)) => Theme::MassEffect2,
                Some(SaveGame::MassEffect3(_)) => Theme::MassEffect3,
            })
            .await;
        let _style = ui.push_style_var(StyleVar::WindowRounding(0.0));

        // Window
        if let Some(_t) = window.begin(ui) {
            // Main menu bar
            if let Some(_t) = ui.begin_menu_bar() {
                if ui.button(im_str!("Open")) {
                    self.open_save().await;
                }
                if ui.button(im_str!("Save")) {
                    self.save_save(save_game).await;
                }
            }

            // Error popup
            {
                let ErrorState { errors, is_opened } = &mut state.errors;
                if *is_opened {
                    ui.open_popup(im_str!("Error###error"));
                }

                if let Some(_t) = PopupModal::new(im_str!("Error###error"))
                    .always_auto_resize(true)
                    .begin_popup(ui)
                {
                    errors.iter().for_each(|error| {
                        ui.text(error.to_string());
                    });
                    ui.separator();

                    if ui.button_with_size(im_str!("OK"), [70.0, 0.0]) {
                        *is_opened = false;
                        errors.clear();
                        ui.close_current_popup();
                    }
                }
            }

            // Notification
            self.draw_nofification_overlay(&mut state.notification);

            match save_game {
                None => ui.text(im_str!("Rien ici")),
                Some(SaveGame::MassEffect1) => todo!(),
                Some(SaveGame::MassEffect2(save_game)) => self.draw_mass_effect_2(save_game).await,
                Some(SaveGame::MassEffect3(save_game)) => self.draw_mass_effect_3(save_game).await,
            };
        }
    }

    fn draw_nofification_overlay(&self, notification: &mut Option<NotificationState>) {
        if let Some(NotificationState { string, close_time }) = notification {
            let ui = self.ui;
            let time = ui.time();

            let _style = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
            let window = Window::new(im_str!("###notification"))
                .position([ui.io().display_size[0] / 2.0, 50.0], Condition::Always)
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .always_auto_resize(true);

            if let Some(_t) = window.begin(ui) {
                ui.text(&string);

                let remaining = (*close_time - time) / NOTIFICATION_TIME;
                ProgressBar::new(remaining as f32)
                    .overlay_text(&ImString::new("time_bar"))
                    .size([-0.0001, 2.0])
                    .build(ui);
            }

            if *close_time < time {
                *notification = None;
            }
        }
    }

    async fn draw_mass_effect_2(&self, save_game: &mut Me2SaveGame) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("me2-tabs")).begin(ui) {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                if let Some(_t) = ChildWindow::new("mass_effect_2").size([0.0, 0.0]).begin(ui) {
                    save_game.draw_raw_ui(self, "Mass Effect 2");
                }
            }
        }
    }

    async fn draw_mass_effect_3(&self, save_game: &mut Me3SaveGame) {
        let ui = self.ui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("me3-tabs")).begin(ui) {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(ui) {
                if let Some(_t) = ChildWindow::new("mass_effect_3").size([0.0, 0.0]).begin(ui) {
                    save_game.draw_raw_ui(self, "Mass Effect 3");
                }
            }
        }
    }

    // Edit boxes
    pub fn draw_edit_string(&self, ident: &str, value: &mut ImString) {
        self.draw_colored_bg(ident, || {
            self.ui.input_text(&ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_bool(&self, ident: &str, value: &mut bool) {
        let ui = self.ui;

        self.draw_colored_bg(ident, || {
            let _width = ui.push_item_width(100.0);
            ui.checkbox(&ImString::new(ident), value);
        });
    }

    pub fn draw_edit_i32(&self, ident: &str, value: &mut i32) {
        let ui = self.ui;

        self.draw_colored_bg(ident, || {
            let _width = ui.push_item_width(100.0);
            InputInt::new(ui, &ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_f32(&self, ident: &str, value: &mut f32) {
        let ui = self.ui;

        self.draw_colored_bg(ident, || {
            let _width = ui.push_item_width(100.0);
            InputFloat::new(ui, &ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_enum(&self, ident: &str, current_item: &mut usize, items: &[&ImStr]) {
        let ui = self.ui;

        self.draw_colored_bg(ident, || {
            let _width = ui.push_item_width(200.0);
            ComboBox::new(&ImString::new(ident)).build_simple_string(ui, current_item, items);
        });
    }

    pub fn draw_edit_color(&self, ident: &str, color: &mut [f32; 4]) {
        let ui = self.ui;

        self.draw_colored_bg(ident, || {
            let _width = ui.push_item_width(200.0);
            ColorEdit::new(&ImString::new(ident), color).build(ui);
        });
    }

    // View widgets
    pub fn draw_struct<F>(&self, ident: &str, fields: F)
    where
        F: FnOnce(),
    {
        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(self.ui) {
            fields();
        }
    }

    pub fn draw_vec<T>(&self, ident: &str, list: &mut Vec<T>)
    where
        T: SaveData + Default,
    {
        let ui = self.ui;

        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(ui) {
            if !list.is_empty() {
                // Item
                let mut remove = None;
                for (i, item) in list.iter_mut().enumerate() {
                    if ui.small_button(&im_str!("remove##x-{}", i)) {
                        remove = Some(i);
                    }
                    ui.same_line();
                    item.draw_raw_ui(self, &i.to_string());
                }

                // Remove
                if let Some(i) = remove {
                    list.remove(i);
                }

                // Add
                if ui.button(&im_str!("add##add-{}", ident)) {
                    // Ça ouvre automatiquement le tree node de l'élément ajouté
                    TreeNode::new(&ImString::new(&list.len().to_string()))
                        .opened(true, Condition::Always)
                        .build(ui, || {});

                    list.push(T::default());
                }
            } else {
                ui.text("Empty");
            }
        }
    }

    pub fn draw_bitarray(&self, ident: &str, list: &mut Vec<bool>) {
        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(self.ui) {
            if !list.is_empty() {
                let mut clipper = ListClipper::new(list.len() as i32).begin(self.ui);
                while clipper.step() {
                    for i in clipper.display_start()..clipper.display_end() {
                        list[i as usize].draw_raw_ui(self, &i.to_string());
                    }
                }
            }
        }
    }

    pub fn draw_indexmap<K, V>(&self, ident: &str, list: &mut IndexMap<K, V>)
    where
        K: SaveData + Eq + Hash + Default,
        V: SaveData + Default,
    {
        let ui = self.ui;

        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(ui) {
            if !list.is_empty() {
                // Item
                let mut remove = None;
                for i in 0..list.len() {
                    if ui.small_button(&im_str!("remove##x-{}", i)) {
                        remove = Some(i);
                    }
                    ui.same_line();

                    TreeNode::new(&ImString::new(i.to_string())).build(ui, || {
                        if let Some((key, value)) = list.get_index_mut(i) {
                            key.draw_raw_ui(self, "##k");
                            value.draw_raw_ui(self, "##v");
                        }
                    });
                }

                // Remove
                if let Some(i) = remove {
                    list.shift_remove_index(i);
                }

                // Add
                if ui.button(&im_str!("add##add-{}", ident)) {
                    // Ça ouvre automatiquement le tree node de l'élément ajouté
                    TreeNode::new(&ImString::new(&list.len().to_string()))
                        .opened(true, Condition::Always)
                        .build(ui, || {});

                    // FIXME: Ajout d'un nouvel élément si K = 0i32 déjà présent
                    list.entry(K::default()).or_default();
                }
            } else {
                ui.text("Empty");
            }
        }
    }

    // Helpers
    fn draw_colored_bg<F>(&self, id: &str, inner: F)
    where
        F: FnOnce(),
    {
        let _bg = self.bg_colors();
        if let Some(_t) = ChildWindow::new(id).size([0.0, 19.0]).begin(self.ui) {
            inner();
        }
    }

    fn bg_colors(&self) -> ColorStackToken<'a> {
        let bg_dark = [0.1, 0.1, 0.1, 1.0];
        let bg_light = [0.15, 0.15, 0.15, 1.0];

        let bgs = [bg_dark, bg_light];

        let bg_count = self.bg_count.fetch_add(1, Ordering::AcqRel);
        self.ui.push_style_color(StyleColor::ChildBg, bgs[bg_count % bgs.len()])
    }

    // Style
    async fn style_colors(&self, game_theme: Theme) -> [ColorStackToken<'a>; 17] {
        let ui = self.ui;
        let theme = match game_theme {
            Theme::MassEffect1 | Theme::MassEffect2 | Theme::MassEffect3 => ColorTheme {
                bg_color: [0.40, 0.0, 0.0, 1.0],
                color: [0.53, 0.0, 0.0, 1.0],
                active_color: [0.68, 0.0, 0.0, 1.0],
                hover_color: [0.86, 0.0, 0.0, 1.0],
            },
        };

        [
            ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]),
            ui.push_style_color(StyleColor::TitleBgActive, theme.active_color),
            ui.push_style_color(StyleColor::FrameBg, theme.bg_color),
            ui.push_style_color(StyleColor::FrameBgActive, theme.active_color),
            ui.push_style_color(StyleColor::FrameBgHovered, theme.hover_color),
            ui.push_style_color(StyleColor::TextSelectedBg, theme.active_color),
            ui.push_style_color(StyleColor::Button, theme.bg_color),
            ui.push_style_color(StyleColor::ButtonActive, theme.active_color),
            ui.push_style_color(StyleColor::ButtonHovered, theme.hover_color),
            ui.push_style_color(StyleColor::Tab, theme.color),
            ui.push_style_color(StyleColor::TabActive, theme.active_color),
            ui.push_style_color(StyleColor::TabHovered, theme.hover_color),
            ui.push_style_color(StyleColor::Header, theme.bg_color),
            ui.push_style_color(StyleColor::HeaderActive, theme.active_color),
            ui.push_style_color(StyleColor::HeaderHovered, theme.hover_color),
            ui.push_style_color(StyleColor::CheckMark, [1.0, 1.0, 1.0, 1.0]),
            ui.push_style_color(StyleColor::PlotHistogram, [1.0, 1.0, 1.0, 1.0]),
        ]
    }

    // Actions
    async fn open_save(&self) {
        let result = wfd::open_dialog(DialogParams {
            file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
            ..Default::default()
        });

        if let Ok(result) = result {
            let _ =
                self.event_addr.send_async(MainEvent::OpenSave(result.selected_file_path)).await;
        }
    }

    async fn save_save(&self, save_game: &Option<SaveGame>) {
        if let Some(save_game) = save_game {
            let default_ext = match save_game {
                SaveGame::MassEffect1 => todo!(),
                SaveGame::MassEffect2(_) | SaveGame::MassEffect3(_) => "pcsav",
            };

            let result = wfd::save_dialog(DialogParams {
                default_extension: default_ext,
                file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
                ..Default::default()
            });

            if let Ok(result) = result {
                let _ = self
                    .event_addr
                    .send_async(MainEvent::SaveSave((result.selected_file_path, save_game.clone())))
                    .await;
            }
        }
    }
}

enum Theme {
    MassEffect1,
    MassEffect2,
    MassEffect3,
}

struct ColorTheme {
    bg_color: [f32; 4],
    color: [f32; 4],
    active_color: [f32; 4],
    hover_color: [f32; 4],
}