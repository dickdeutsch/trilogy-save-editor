use imgui::{
    im_str, ColorEdit, ComboBox, Condition, ImStr, ImString, InputFloat, InputInt, ListClipper,
};
use indexmap::IndexMap;
use std::{any::Any, fmt::Display, hash::Hash};

use crate::{
    gui::imgui_utils::Table,
    save_data::{
        shared::{plot::BoolSlice, Guid},
        RawUi,
    },
};

use super::{imgui_utils::TreeNode, Gui};

impl<'ui> Gui<'ui> {
    // Edit boxes
    pub fn draw_edit_string(&self, ident: &str, value: &mut ImString) -> bool {
        self.ui.input_text(&ImString::new(ident), value).resize_buffer(true).build()
    }

    pub fn draw_edit_bool(&self, ident: &str, value: &mut bool) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        ui.checkbox(&ImString::new(ident), value);
        width.pop(ui);
    }

    pub fn draw_edit_u8(&self, ident: &str, value: &mut u8) {
        let ui = self.ui;

        let width = ui.push_item_width(97.0);
        let mut new_value = *value as i32;
        if InputInt::new(ui, &ImString::new(ident), &mut new_value).build() {
            *value = new_value.clamp(0, u8::MAX as i32) as u8;
        }
        width.pop(ui);
    }

    pub fn draw_edit_i32(&self, ident: &str, value: &mut i32) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        InputInt::new(ui, &ImString::new(ident), value).build();
        width.pop(ui);
    }

    pub fn draw_edit_f32(&self, ident: &str, value: &mut f32) {
        let ui = self.ui;

        let width = ui.push_item_width(120.0);
        InputFloat::new(ui, &ImString::new(ident), value).build();
        width.pop(ui);
    }

    pub fn draw_edit_enum(&self, ident: &str, current_item: &mut usize, items: &[&ImStr]) -> bool {
        let ui = self.ui;

        let width = ui.push_item_width(200.0);
        let edited =
            ComboBox::new(&ImString::new(ident)).build_simple_string(ui, current_item, items);
        width.pop(ui);
        edited
    }

    pub fn draw_edit_color(&self, ident: &str, color: &mut [f32; 4]) {
        let ui = self.ui;

        let width = ui.push_item_width(200.0);
        ColorEdit::new(&ImString::new(ident), color).build(ui);
        width.pop(ui);
    }

    pub fn draw_edit_guid(&self, ident: &str, guid: &mut Guid) {
        let ui = self.ui;
        {
            let width = ui.push_item_width(65.0);
            ui.input_text(&im_str!("##{}-part1", ident), &mut guid.part1)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part2", ident), &mut guid.part2)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part3", ident), &mut guid.part3)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(36.0);
            ui.input_text(&im_str!("##{}-part4", ident), &mut guid.part4)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
        ui.same_line();
        ui.text("-");
        {
            ui.same_line();
            let width = ui.push_item_width(93.0);
            ui.input_text(&im_str!("{}##{}-part5", ident, ident), &mut guid.part5)
                .chars_hexadecimal(true)
                .build();
            width.pop(ui);
        }
    }

    // View widgets
    pub fn draw_text(&self, text: &ImStr, label: Option<&ImStr>) {
        let ui = self.ui;

        if let Some(label) = label {
            ui.label_text(label, text)
        } else {
            ui.text(text);
        }
    }

    pub fn draw_option<T: RawUi>(&self, ident: &str, has_value: &mut Option<T>) {
        let ui = self.ui;

        if let Some(value) = has_value {
            ui.align_text_to_frame_padding();
            let remove = ui.small_button(&im_str!("remove##rm-option-{}", ident));

            ui.same_line();
            value.draw_raw_ui(self, ident);

            // Remove
            if remove {
                *has_value = None;
            }
        } else {
            self.draw_text(im_str!("None"), Some(&ImString::new(ident)));
        }
    }

    pub fn draw_struct(&self, ident: &str, fields: &mut [Box<dyn FnMut() + '_>]) {
        let ui = self.ui;
        TreeNode::new(ident).build(ui, || {
            Table::new(&ImString::new(ident), 1).build(ui, || {
                for draw_field in fields {
                    Table::next_row();
                    draw_field();
                }
            });
        });
    }

    pub fn draw_boolvec(&self, ident: &str, list: &mut BoolSlice) -> Option<()> {
        let ui = self.ui;

        let _tree_node = TreeNode::new(ident).push(ui)?;
        let _table = Table::new(&ImString::new(ident), 1).begin(ui)?;

        if !list.is_empty() {
            let mut clipper = ListClipper::new(list.len() as i32).begin(ui);
            while clipper.step() {
                for i in clipper.display_start()..clipper.display_end() {
                    Table::next_row();
                    list.get_mut(i as usize).unwrap().draw_raw_ui(self, &i.to_string());
                }
            }
        } else {
            Table::next_row();
            ui.text("Empty");
        }
        Some(())
    }

    pub fn draw_vec<T>(&self, ident: &str, list: &mut Vec<T>) -> Option<()>
    where
        T: RawUi + Default,
    {
        let ui = self.ui;

        let _tree_node = TreeNode::new(ident).push(ui)?;
        let _table = Table::new(&ImString::new(ident), 1).begin(ui)?;

        if !list.is_empty() {
            // Item
            let mut remove = None;
            for (i, item) in list.iter_mut().enumerate() {
                Table::next_row();
                ui.align_text_to_frame_padding();
                if ui.small_button(&im_str!("remove##rm-vec-{}", i)) {
                    remove = Some(i);
                }
                ui.same_line();
                item.draw_raw_ui(self, &i.to_string());
            }

            // Remove
            if let Some(i) = remove {
                list.remove(i);
            }
        } else {
            Table::next_row();
            ui.text("Empty");
        }

        // Add
        Table::next_row();
        if ui.button(im_str!("add")) {
            // Ça ouvre automatiquement le tree node de l'élément ajouté
            imgui::TreeNode::new(&im_str!("{}", list.len()))
                .opened(true, Condition::Always)
                .push(ui);

            list.push(T::default());
        }
        Some(())
    }

    pub fn draw_vec_no_edit<T: RawUi>(&self, ident: &str, list: &mut Vec<T>) -> Option<()> {
        let ui = self.ui;

        let _tree_node = TreeNode::new(ident).push(ui)?;
        let _table = Table::new(&ImString::new(ident), 1).begin(ui)?;

        if !list.is_empty() {
            // Item
            for (i, item) in list.iter_mut().enumerate() {
                Table::next_row();
                item.draw_raw_ui(self, &i.to_string());
            }
        } else {
            Table::next_row();
            ui.text("Empty");
        }
        Some(())
    }

    pub fn draw_indexmap<K, V>(&self, ident: &str, list: &mut IndexMap<K, V>) -> Option<()>
    where
        K: RawUi + Eq + Hash + Default + Display + 'static,
        V: RawUi + Default,
    {
        let ui = self.ui;

        let _tree_node = TreeNode::new(ident).push(ui)?;
        let _table = Table::new(&ImString::new(ident), 1).begin(ui)?;

        if !list.is_empty() {
            // Item
            let mut remove = None;
            for i in 0..list.len() {
                Table::next_row();
                ui.align_text_to_frame_padding();
                if ui.small_button(&im_str!("remove##rm-indexmap-{}", i)) {
                    remove = Some(i);
                }
                ui.same_line();

                if let Some((key, value)) = list.get_index_mut(i) {
                    TreeNode::new(&format!("{}##{}", key, i)).build(ui, || {
                        Table::new(&im_str!("table-{}", i), 1).build(ui, || {
                            Table::next_row();
                            key.draw_raw_ui(self, "id##key");
                            Table::next_row();
                            value.draw_raw_ui(self, "value##value");
                        });
                    });
                }
            }

            // Remove
            if let Some(i) = remove {
                list.shift_remove_index(i);
            }
        } else {
            Table::next_row();
            ui.text("Empty");
        }

        // Add
        Table::next_row();
        if ui.button(im_str!("add")) {
            let mut new_k = K::default();

            // i32 exception to fix K = 0 already in the hashmap
            if let Some(new_k_as_i32) = (&mut new_k as &mut dyn Any).downcast_mut::<i32>() {
                *new_k_as_i32 = -1;
            }

            // Ça ouvre automatiquement le tree node de l'élément ajouté
            imgui::TreeNode::new(&im_str!("{}", list.len()))
                .opened(true, Condition::Always)
                .push(ui);

            list.entry(new_k).or_default();
        }
        Some(())
    }
}