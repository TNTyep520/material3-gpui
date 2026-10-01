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

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, ClickEvent, Hsla, IntoElement, MouseButton, ParentElement as _, RenderOnce, Rgba,
    StatefulInteractiveElement as _, Styled, Window, WindowControlArea, div, prelude::*, px,
};
use material3_gpui::icon::{Icon, IconName};
use material3_gpui::prelude::ActiveTheme;

const HEIGHT: f32 = 48.0;

const BUTTON_SIZE: f32 = 44.0;

const CLOSE_RED: Rgba = Rgba {
    r: 232.0 / 255.0,
    g: 60.0 / 255.0,
    b: 60.0 / 255.0,
    a: 1.0,
};

const MAC_TRAFFIC_LIGHT_INSET: f32 = 48.0;

#[derive(IntoElement)]
pub struct CustomTitleBar;

impl RenderOnce for CustomTitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = *cx.theme().colors();
        let is_windows = cfg!(target_os = "windows");
        let is_mac = cfg!(target_os = "macos");
        let is_linux = !is_windows && !is_mac;

        let dragging = Rc::new(Cell::new(false));

        let title_bar = div()
            .id("custom-title-bar")
            .relative()
            .h(px(HEIGHT))
            .w_full()
            .flex_none()
            .flex()
            .items_center()
            .px(px(12.))
            .gap(px(12.))
            .bg(colors.surface)
            .when(is_windows, |el| {
                el.window_control_area(WindowControlArea::Drag)
            })
            .when(is_linux, |el| {
                let down = dragging.clone();
                let up = dragging.clone();
                let mv = dragging.clone();
                el.on_mouse_down(MouseButton::Left, move |_, _, _| down.set(true))
                    .on_mouse_up(MouseButton::Left, move |_, _, _| up.set(false))
                    .on_mouse_move(move |_, window, _| {
                        if mv.get() {
                            mv.set(false);
                            window.start_window_move();
                        }
                    })
                    .on_click(move |event, window, _| {
                        if event.click_count() == 2 {
                            window.zoom_window();
                        }
                    })
            });

        let title_bar = title_bar.when(is_mac, |el| {
            el.child(div().flex_none().w(px(MAC_TRAFFIC_LIGHT_INSET)))
        });

        let title_bar = title_bar.child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(14.))
                .text_color(colors.on_surface)
                .child("Material 3 · Catalog"),
        );

        let controls = if is_mac {
            div()
        } else {
            div()
                .flex_none()
                .flex()
                .items_center()
                .gap(px(4.))
                .child(window_button(
                    "titlebar-minimize",
                    WindowControlArea::Min,
                    colors.on_surface_variant,
                    move |_event, window, _cx| window.minimize_window(),
                    is_windows,
                ))
                .child(window_button(
                    "titlebar-close",
                    WindowControlArea::Close,
                    CLOSE_RED.into(),
                    move |_event, window, _cx| window.remove_window(),
                    is_windows,
                ))
        };

        title_bar.child(controls)
    }
}

fn window_button(
    id: &'static str,
    area: WindowControlArea,
    icon_color: Hsla,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    windows_native: bool,
) -> impl IntoElement {
    let base = div()
        .id(id)
        .size(px(BUTTON_SIZE))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .occlude()
        .hover(move |s| s.bg(icon_color.opacity(0.10)))
        .active(move |s| s.bg(icon_color.opacity(0.18)));
    let base = if windows_native {
        base.window_control_area(area)
    } else {
        base.on_click(move |event, window, cx| {
            cx.stop_propagation();
            on_click(event, window, cx)
        })
    };

    let icon = if id == "titlebar-close" {
        Icon::new(IconName::new("close"))
            .size(px(20.))
            .color(icon_color)
    } else {
        Icon::new(IconName::new("remove"))
            .size(px(20.))
            .color(icon_color)
    };
    base.child(icon)
}
