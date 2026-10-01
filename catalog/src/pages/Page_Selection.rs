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

use gpui::{App, AppContext as _, Context, Entity, IntoElement, Render, WeakEntity, Window};
use material3_gpui::prelude::*;

use super::{LogErr as _, gallery, showcase_group, specimen};

pub struct SelectionPage {
    segmented_single: Entity<SegmentedButtonRowState>,
    segmented_multiple: Entity<SegmentedButtonRowState>,
    segmented_disabled: Entity<SegmentedButtonRowState>,
    cb_a: Entity<CheckboxState>,
    cb_disabled: Entity<CheckboxState>,
    radios: Vec<Entity<RadioState>>,
    sw_dicts: Entity<SwitchState>,
    sw_sync: Entity<SwitchState>,
    sw_quiet: Entity<SwitchState>,
    sw_disabled_on: Entity<SwitchState>,
    sw_disabled_icon_on: Entity<SwitchState>,
}

impl SelectionPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let sw_dicts = Switch::new("sw-dicts")
            .checked(true)
            .on_change(move |checked, window, cx| {
                let message = if checked {
                    "Spell-check on"
                } else {
                    "Spell-check off"
                };
                show_snackbar(window, cx, Snackbar::new(message), None);
            })
            .build(cx);
        let sw_sync = Switch::new("sw-sync").build(cx);
        let sw_quiet = Switch::new("sw-quiet")
            .checked(true)
            .with_check_icon(true)
            .build(cx);
        let sw_disabled_on = Switch::new("sw-dis-on")
            .disabled(true)
            .checked(true)
            .build(cx);
        let sw_disabled_icon_on = Switch::new("sw-dis-icon-on")
            .with_check_icon(true)
            .checked(true)
            .disabled(true)
            .build(cx);

        let cb_a = Checkbox::new("cb-a").checked(true).build(cx);
        let cb_disabled = Checkbox::new("cb-dis")
            .checked(true)
            .disabled(true)
            .build(cx);

        let segmented_single = SegmentedButtonRow::new("segmented-single")
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);
        let segmented_multiple = SegmentedButtonRow::new("segmented-multiple")
            .selection_mode(SegmentedButtonSelectionMode::Multiple)
            .buttons([
                SegmentedButton::new("Walk")
                    .icon(IconName::new("directions_walk"))
                    .selected(true),
                SegmentedButton::new("Bike").icon(IconName::new("directions_bike")),
                SegmentedButton::new("Drive")
                    .icon(IconName::new("directions_car"))
                    .selected(true),
            ])
            .build(cx);
        let segmented_disabled = SegmentedButtonRow::new("segmented-disabled")
            .disabled(true)
            .buttons([
                SegmentedButton::new("Day").selected(true),
                SegmentedButton::new("Week"),
                SegmentedButton::new("Month"),
            ])
            .build(cx);

        cx.new(|cx| {
            let weak: WeakEntity<Self> = cx.entity().downgrade();
            let radios: Vec<Entity<RadioState>> = (0..3usize)
                .map(|ix| {
                    let weak = weak.clone();
                    RadioButton::new(("radio-plan", ix))
                        .selected(ix == 0)
                        .disabled(ix == 2)
                        .on_select(move |window, cx| {
                            weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                for (j, other) in page.radios.iter().enumerate() {
                                    if j != ix {
                                        other.update(
                                            cx,
                                            |radio: &mut RadioState,
                                             radio_cx: &mut Context<RadioState>| {
                                                radio.set_selected(false, window, radio_cx);
                                            },
                                        );
                                    }
                                }
                            })
                            .log_err();
                        })
                        .build(cx)
                })
                .collect();

            Self {
                segmented_single,
                segmented_multiple,
                segmented_disabled,
                cb_a,
                cb_disabled,
                radios,
                sw_dicts,
                sw_sync,
                sw_quiet,
                sw_disabled_on,
                sw_disabled_icon_on,
            }
        })
    }
}

impl Render for SelectionPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(
                cx,
                "Switches",
                [
                    specimen(cx, "Selected", self.sw_dicts.clone()),
                    specimen(cx, "Unselected", self.sw_sync.clone()),
                    specimen(cx, "With icon", self.sw_quiet.clone()),
                    specimen(cx, "Disabled", self.sw_disabled_on.clone()),
                    specimen(cx, "Disabled · icon", self.sw_disabled_icon_on.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Checkboxes",
                [
                    specimen(cx, "Enabled", self.cb_a.clone()),
                    specimen(cx, "Disabled", self.cb_disabled.clone()),
                ],
            ),
            showcase_group(
                cx,
                "Radio buttons",
                ["Selected", "Unselected", "Disabled"]
                    .into_iter()
                    .zip(self.radios.iter())
                    .map(|(label, radio)| specimen(cx, label, radio.clone())),
            ),
            showcase_group(
                cx,
                "Segmented buttons",
                [
                    self.segmented_single.clone().into_any_element(),
                    self.segmented_multiple.clone().into_any_element(),
                    self.segmented_disabled.clone().into_any_element(),
                ],
            ),
        ])
    }
}
