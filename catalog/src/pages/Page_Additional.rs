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

use super::{LogErr as _, gallery, showcase_group};

pub struct AdditionalPage {
    weak: WeakEntity<Self>,
    toolbar_add: Entity<IconButtonState>,
    toolbar_edit: Entity<IconButtonState>,
    fab_action_add: Entity<FabState>,
    fab_action_settings: Entity<FabState>,
    fab_toggle_open: Entity<FabState>,
    fab_toggle_close: Entity<FabState>,
    rail_home: Entity<ButtonState>,
    rail_settings: Entity<ButtonState>,
    search_field: Entity<TextFieldState>,
    secure_field: Entity<TextFieldState>,
    reset_button: Entity<ButtonState>,
    query: String,
    fab_menu_expanded: bool,
    dismissed: bool,
    date: DatePickerState,
    time: TimePickerState,
    range: (f32, f32),
}

impl AdditionalPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();

            let toolbar_add = IconButton::new("tool-add", IconName::Add)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Toolbar: add"), None);
                })
                .build(cx);
            let toolbar_edit = IconButton::new("tool-edit", IconName::Edit)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Toolbar: edit"), None);
                })
                .build(cx);

            let fab_action_add = Fab::new("fab-action-add", IconName::Add)
                .color(FabColor::Secondary)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Menu: new item"), None);
                })
                .build(cx);
            let fab_action_settings = Fab::new("fab-action-settings", IconName::Settings)
                .color(FabColor::Secondary)
                .on_click(|_, window, cx| {
                    show_snackbar(window, cx, Snackbar::new("Menu: settings"), None);
                })
                .build(cx);
            let fab_toggle_open = {
                let weak = weak.clone();
                Fab::new("fab-toggle-open", IconName::Add)
                    .on_click(move |_, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.fab_menu_expanded = true;
                            cx.notify();
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            let fab_toggle_close = {
                let weak = weak.clone();
                Fab::new("fab-toggle-close", IconName::Close)
                    .on_click(move |_, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.fab_menu_expanded = false;
                            cx.notify();
                        })
                        .log_err();
                    })
                    .build(cx)
            };

            let rail_home = Button::new("rail-home", "Home").text().build(cx);
            let rail_settings = Button::new("rail-settings", "Settings").text().build(cx);

            let search_field = {
                let weak = weak.clone();
                TextField::new("search-query", "Search")
                    .leading_icon(IconName::Search)
                    .on_value_change(move |value, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            if page.query != value {
                                page.query = value.to_string();
                                cx.notify();
                            }
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            let secure_field = SecureTextField::new("secure", "Passphrase")
                .value("secret")
                .build(cx);

            let reset_button = {
                let weak = weak.clone();
                Button::new("dismiss-reset", "Reset card")
                    .outlined()
                    .on_click(move |_, _, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.dismissed = false;
                            cx.notify();
                        })
                        .log_err();
                    })
                    .build(cx)
            };

            Self {
                weak,
                toolbar_add,
                toolbar_edit,
                fab_action_add,
                fab_action_settings,
                fab_toggle_open,
                fab_toggle_close,
                rail_home,
                rail_settings,
                search_field,
                secure_field,
                reset_button,
                query: String::new(),
                fab_menu_expanded: false,
                dismissed: false,
                date: DatePickerState::today(),
                time: TimePickerState::new(10, 30),
                range: (0.2, 0.78),
            }
        })
    }
}

impl Render for AdditionalPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let weak = self.weak.clone();

        let toolbar = FloatingToolbar::new("toolbar")
            .children([self.toolbar_add.clone(), self.toolbar_edit.clone()]);

        let fab_menu = FabMenu::new("fab-menu")
            .expanded(self.fab_menu_expanded)
            .action(self.fab_action_add.clone())
            .action(self.fab_action_settings.clone());
        let fab_toggle = if self.fab_menu_expanded {
            self.fab_toggle_close.clone()
        } else {
            self.fab_toggle_open.clone()
        };

        let wide_rail = WideNavigationRail::new("wide-rail")
            .children([self.rail_home.clone(), self.rail_settings.clone()]);

        let swiping = Card::new()
            .filled()
            .w(px(280.))
            .max_w_full()
            .p(px(16.))
            .child("Swipe me away");
        let date_label = self
            .date
            .selected_day
            .map(|day| format!("{:04}-{:02}-{:02}", self.date.year, self.date.month, day))
            .unwrap_or_else(|| "Select date".into());
        let time_label = format!("{:02}:{:02}", self.time.hour, self.time.minute);
        let range_label = format!("{:.0}% – {:.0}%", self.range.0 * 100., self.range.1 * 100.);

        gallery([
            showcase_group(
                cx,
                "Indicators",
                [
                    LoadingIndicator::new("loading")
                        .size(px(40.))
                        .into_any_element(),
                    WavyProgressIndicator::new("wavy")
                        .value(0.64)
                        .into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Pickers & search",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .w_full()
                    .min_w_0()
                    .child(SearchBar::new("search", self.search_field.clone()))
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(if self.query.is_empty() {
                                "Type to search".to_string()
                            } else {
                                format!("Query: {}", self.query)
                            }),
                    )
                    .child(self.secure_field.clone())
                    .child(
                        DatePicker::new("date")
                            .value(date_label)
                            .state(self.date)
                            .on_date_change({
                                let weak = weak.clone();
                                move |year, month, day, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.date = DatePickerState::new(year, month, Some(day));
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            })
                            .on_month_change({
                                let weak = weak.clone();
                                move |year, month, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        let day = page.date.selected_day;
                                        page.date = DatePickerState::new(year, month, day);
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            }),
                    )
                    .child(
                        TimePicker::new("time")
                            .value(time_label)
                            .state(self.time)
                            .on_time_change({
                                let weak = weak.clone();
                                move |state, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.time = state;
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            }),
                    )
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Toolbars & navigation",
                [
                    toolbar.into_any_element(),
                    div()
                        .flex()
                        .flex_col()
                        .items_end()
                        .gap(px(8.))
                        .child(fab_menu)
                        .child(fab_toggle)
                        .into_any_element(),
                    wide_rail.into_any_element(),
                ],
            ),
            showcase_group(
                cx,
                "Dismiss & scroll",
                [div()
                    .flex()
                    .flex_wrap()
                    .items_center()
                    .gap(px(24.))
                    .w_full()
                    .min_w_0()
                    .child(if self.dismissed {
                        self.reset_button.clone().into_any_element()
                    } else {
                        SwipeToDismissBox::new("dismiss", swiping)
                            .background(div().bg(colors.error_container))
                            .on_value_change({
                                let weak = weak.clone();
                                move |_, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.dismissed = true;
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            })
                            .into_any_element()
                    })
                    .child(
                        div()
                            .relative()
                            .h(px(72.))
                            .w(px(280.))
                            .max_w_full()
                            .child("Scrollable stack")
                            .child(Scrollbar::new("scrollbar").position(0.35))
                            .into_any_element(),
                    )
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Range slider",
                [div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(range_label),
                    )
                    .child(
                        RangeSlider::new("range-tune", self.range.0, self.range.1)
                            .on_value_change({
                                move |next, _, cx| {
                                    weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                        page.range = next;
                                        cx.notify();
                                    })
                                    .log_err();
                                }
                            })
                            .into_any_element(),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
