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
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div,
    prelude::FluentBuilder as _, px,
};

use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content};

type DismissHandler = std::rc::Rc<dyn Fn(&mut App)>;

pub struct BottomSheetPage {
    pub(crate) open_button: Entity<ButtonState>,
    pub(crate) sheet_open: bool,
    on_dismiss: Option<DismissHandler>,
}

impl BottomSheetPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let open_button = Button::new("open-sheet-btn", "Share this page")
            .filled()
            .build(cx);
        cx.new(|_| Self {
            open_button,
            sheet_open: false,
            on_dismiss: None,
        })
    }

    pub fn set_on_dismiss(&mut self, on_dismiss: impl Fn(&mut App) + 'static) {
        self.on_dismiss = Some(std::rc::Rc::new(on_dismiss));
    }

    pub fn sheet_open(&self) -> bool {
        self.sheet_open
    }

    pub fn sheet(&self, cx: &App) -> AnyElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let on_dismiss = self.on_dismiss.clone();

        ModalBottomSheet::new("sheet-demo")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        typography
                            .title_large
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child("Share with the studio"),
                    )
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child("Everyone here can also open & comment"),
                    ),
            )
            .child(
                List::new().children(SHARE_ACTIONS.into_iter().enumerate().map(
                    |(ix, (title, icon))| {
                        ListItem::new(("sheet-item", ix), title).leading_icon(IconName::new(icon))
                    },
                )),
            )
            .when_some(on_dismiss, |el, on_dismiss| {
                el.on_dismiss(move |_window, cx| on_dismiss(cx))
            })
            .into_any_element()
    }
}

const SHARE_ACTIONS: [(&str, &str); 4] = [
    ("Send to proof readers", "menu"),
    ("Copy poster link", "info"),
    ("Edit title", "edit"),
    ("Delete", "delete"),
];

impl ComponentPage for BottomSheetPage {
    fn description(&self) -> &'static str {
        "Bottom sheets are surfaces containing supplementary content, anchored to the \
         bottom of the screen."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[
            ExampleMeta {
                name: "ModalBottomSheetSample",
                description: "Bottom Sheet examples",
            },
            ExampleMeta {
                name: "SimpleBottomSheetScaffoldSample",
                description: "Bottom Sheet examples",
            },
        ]
    }

    fn example(&self, index: usize, cx: &App) -> AnyElement {
        match index {
            0 => {
                let theme = cx.theme();
                let colors = *theme.colors();
                example_content([
                    div()
                        .text_color(colors.on_surface_variant)
                        .child(
                            "Tap the button to open a modal bottom sheet with a drag \
                             handle. Tap the scrim to dismiss.",
                        )
                        .into_any_element(),
                    self.open_button.clone().into_any_element(),
                ])
            }
            1 => {
                let theme = cx.theme();
                let colors = *theme.colors();
                let typography = *theme.typography();
                example_content([div()
                    .w_full()
                    .min_w_0()
                    .h(px(280.))
                    .overflow_hidden()
                    .rounded(px(24.))
                    .child(
                        BottomSheetScaffold::new("sheet-scaffold")
                            .sheet_peek_height(px(64.))
                            .sheet_content(
                                div()
                                    .p(px(16.))
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        typography
                                            .title_medium
                                            .apply(div())
                                            .text_color(colors.on_surface)
                                            .child("Sheet peek content"),
                                    )
                                    .child(
                                        typography
                                            .body_medium
                                            .apply(div())
                                            .text_color(colors.on_surface_variant)
                                            .child(
                                                "The sheet peeks above the scaffold \
                                                 content and expands on drag.",
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .p(px(16.))
                                    .text_color(colors.on_surface_variant)
                                    .child("Scaffold content behind the sheet"),
                            ),
                    )
                    .into_any_element()])
            }
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
