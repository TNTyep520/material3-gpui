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
    App, AppContext as _, Entity, IntoElement, ParentElement as _, Render, Styled, Window, div,
    prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{full_width_group, gallery, showcase_group};

type DismissHandler = std::rc::Rc<dyn Fn(&mut App)>;

pub struct SheetsPage {
    pub(crate) open_button: Entity<ButtonState>,
    pub(crate) sheet_open: bool,
    on_dismiss: Option<DismissHandler>,
    backdrop: Entity<BackdropScaffoldState>,
    bottom_drawer: Entity<BottomDrawerState>,
}

impl SheetsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let open_button = Button::new("open-sheet-btn", "Share this page")
            .filled()
            .build(cx);
        let backdrop = BackdropScaffold::new("backdrop-demo")
            .height(px(320.))
            .back_content(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .px(px(16.))
                    .py(px(12.))
                    .child("Front layer reveals the back layer")
                    .child("Tap the handle to conceal")
                    .into_any_element()
            })
            .front_content(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .px(px(16.))
                    .py(px(16.))
                    .child("Front layer")
                    .child("Tap the handle to reveal the back layer.")
                    .into_any_element()
            })
            .build(cx);
        let bottom_drawer = BottomDrawer::new("bottom-drawer-demo")
            .height(px(260.))
            .content(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .px(px(16.))
                    .py(px(16.))
                    .child("Bottom drawer content")
                    .child("Tap the handle to open or close.")
                    .into_any_element()
            })
            .build(cx);
        cx.new(|cx| {
            cx.observe(&backdrop, |_, _, cx| cx.notify()).detach();
            cx.observe(&bottom_drawer, |_, _, cx| cx.notify()).detach();
            Self {
                open_button,
                sheet_open: false,
                on_dismiss: None,
                backdrop,
                bottom_drawer,
            }
        })
    }

    pub fn set_on_dismiss(&mut self, on_dismiss: impl Fn(&mut App) + 'static) {
        self.on_dismiss = Some(std::rc::Rc::new(on_dismiss));
    }
}

const SHARE_ACTIONS: [(&str, &str); 4] = [
    ("Send to proof readers", "menu"),
    ("Copy poster link", "info"),
    ("Edit title", "edit"),
    ("Delete", "delete"),
];

impl Render for SheetsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let sheet_open = self.sheet_open;
        let on_dismiss = self.on_dismiss.clone();

        gallery(
            [showcase_group(
                cx,
                "Modal bottom sheet",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(div().text_color(colors.on_surface_variant).child(
                        "Tap the button to open a modal bottom sheet with a drag \
                             handle. Tap the scrim to dismiss.",
                    ))
                    .child(self.open_button.clone())
                    .into_any_element()],
            )]
            .into_iter()
            .chain(std::iter::once(
                div()
                    .when(sheet_open, |el| {
                        el.child(
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
                                                .child(
                                                    "Everyone here can also open \
                                                     & comment",
                                                ),
                                        ),
                                )
                                .child(material3_gpui::List::new().children(
                                    SHARE_ACTIONS.into_iter().enumerate().map(
                                        |(ix, (title, icon))| {
                                            material3_gpui::ListItem::new(("sheet-item", ix), title)
                                                .leading_icon(IconName::new(icon))
                                        },
                                    ),
                                ))
                                .when_some(on_dismiss, |el, on_dismiss| {
                                    el.on_dismiss(move |_window, cx| on_dismiss(cx))
                                }),
                        )
                    })
                    .into_any_element(),
            ))
            .chain([
                full_width_group(
                    cx,
                    "Backdrop scaffold",
                    [div()
                        .w_full()
                        .child(self.backdrop.clone())
                        .into_any_element()],
                )
                .into_any_element(),
                full_width_group(
                    cx,
                    "Bottom drawer",
                    [div()
                        .w_full()
                        .child(self.bottom_drawer.clone())
                        .into_any_element()],
                )
                .into_any_element(),
            ])
            .collect::<Vec<_>>(),
        )
    }
}
