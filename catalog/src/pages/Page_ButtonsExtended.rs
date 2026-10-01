// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gpui::{
    App, AppContext as _, Context, Entity, IntoElement, Render, WeakEntity, Window, div,
    prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, gallery, showcase_group, specimen};

pub struct ButtonsExtendedPage {
    weak: WeakEntity<Self>,
    toggle_a: Entity<ToggleButtonState>,
    toggle_b: Entity<ToggleButtonState>,
    toggle_disabled: Entity<ToggleButtonState>,
    group_day: Entity<ButtonState>,
    group_week: Entity<ButtonState>,
    group_month: Entity<ButtonState>,
    group_add: Entity<ButtonState>,
    group_remove: Entity<ButtonState>,
    segmented: Entity<SegmentedButtonRowState>,
    dropdown_field: Entity<ButtonState>,
    menu: Entity<MenuState>,
    dropdown_expanded: bool,
}

impl ButtonsExtendedPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();

            let menu =
                MenuState::new()
                    .item(MenuItem::new("Save as copy").icon(IconName::Edit).on_click(
                        |window, cx| {
                            close_menu(window, cx);
                            show_snackbar(window, cx, Snackbar::new("Copied to drafts"), None);
                        },
                    ))
                    .item(MenuItem::new("Duplicate row").icon(IconName::Add))
                    .item(
                        MenuItem::new("Discard")
                            .icon(IconName::Delete)
                            .on_click(|window, cx| {
                                close_menu(window, cx);
                                show_snackbar(window, cx, Snackbar::new("Discarded"), None);
                            }),
                    )
                    .build(cx);

            let toggle_a = ToggleButton::new("toggle-bold", "Emphasis")
                .icon(IconName::Star)
                .build(cx);
            let toggle_b = ToggleButton::new("toggle-favorite", "Starred")
                .icon(IconName::Favorite)
                .checked(true)
                .build(cx);
            let toggle_disabled = ToggleButton::new("toggle-disabled", "Locked")
                .disabled(true)
                .build(cx);

            let group_day = Button::new("group-1", "Day").tonal().build(cx);
            let group_week = Button::new("group-2", "Week").outlined().build(cx);
            let group_month = Button::new("group-3", "Month").outlined().build(cx);
            let group_add = Button::new("v-1", "Add").filled().build(cx);
            let group_remove = Button::new("v-2", "Remove").outlined().build(cx);

            let segmented = SegmentedButtonRow::new("extended-segmented")
                .buttons([
                    SegmentedButton::new("Day").selected(true),
                    SegmentedButton::new("Week"),
                    SegmentedButton::new("Month"),
                ])
                .build(cx);

            let dropdown_field = Button::new("exposed-field", "Save to…")
                .outlined()
                .build(cx);

            Self {
                weak,
                toggle_a,
                toggle_b,
                toggle_disabled,
                group_day,
                group_week,
                group_month,
                group_add,
                group_remove,
                segmented,
                dropdown_field,
                menu,
                dropdown_expanded: false,
            }
        })
    }
}

impl Render for ButtonsExtendedPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = self.weak.clone();

        let group = ButtonGroup::new("actions").children([
            self.group_day.clone(),
            self.group_week.clone(),
            self.group_month.clone(),
        ]);
        let vertical = ButtonGroup::new("vertical-actions")
            .vertical()
            .children([self.group_add.clone(), self.group_remove.clone()]);
        let dropdown = ExposedDropdownMenu::new("exposed-menu", self.dropdown_field.clone())
            .menu(self.menu.clone())
            .expanded(self.dropdown_expanded)
            .on_expanded_change(move |expanded, _, cx| {
                weak.update(cx, |page, cx| {
                    page.dropdown_expanded = expanded;
                    cx.notify();
                })
                .log_err();
            });

        gallery([
            showcase_group(
                cx,
                "Toggle states",
                [
                    specimen(cx, "Unselected", self.toggle_a.clone()),
                    specimen(cx, "Selected", self.toggle_b.clone()),
                    specimen(cx, "Disabled", self.toggle_disabled.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Button groups",
                [
                    group.into_any_element(),
                    div().h(px(160.)).child(vertical).into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Menu buttons",
                [
                    SplitButton::new("split", "Save")
                        .filled()
                        .menu(self.menu.clone())
                        .into_any_element(),
                    dropdown.into_any_element(),
                ],
            ),
            showcase_group(cx, "Segmented", [self.segmented.clone().into_any_element()]),
        ])
    }
}
