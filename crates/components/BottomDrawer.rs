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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material/material/src/commonMain/kotlin/androidx/compose/material/Drawer.kt

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, Entity, IntoElement, Render, Styled,
    WeakEntity, Window, div, prelude::*, px,
};

use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::{ActiveTheme, Elevation};

type ContentFactory = Rc<dyn Fn() -> AnyElement>;
type ToggleHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub struct BottomDrawer {
    id: ElementId,
    content: ContentFactory,
    height: gpui::Pixels,
    open: bool,
}

pub struct BottomDrawerState {
    id: ElementId,
    content: ContentFactory,
    toggle: ToggleHandler,
    height: gpui::Pixels,
    open: bool,
    progress: Animatable,
    driver: AnimationDriver,
}

impl BottomDrawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: Rc::new(|| div().into_any_element()),
            height: px(320.),
            open: false,
        }
    }

    pub fn content(mut self, content: impl Fn() -> AnyElement + 'static) -> Self {
        self.content = Rc::new(content);
        self
    }

    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<BottomDrawerState> {
        let open = self.open;
        cx.new(|cx| {
            let weak: WeakEntity<BottomDrawerState> = cx.entity().downgrade();
            let toggle: ToggleHandler = Rc::new(move |window, cx| {
                weak.update(cx, |state, cx| {
                    let next = !state.open;
                    state.set_open(next, window, cx);
                })
                .ok();
            });
            BottomDrawerState {
                id: self.id,
                content: self.content,
                toggle,
                height: self.height,
                open,
                progress: Animatable::new(if open { 1.0 } else { 0.0 }, 1.0e-3),
                driver: AnimationDriver::default(),
            }
        })
    }
}

impl BottomDrawerState {
    pub fn open(&self) -> bool {
        self.open
    }

    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.open == open {
            return;
        }
        self.open = open;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
        self.progress
            .animate_to(if open { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for BottomDrawerState {
    fn step(&mut self, now: Instant) -> bool {
        self.progress.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for BottomDrawerState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let colors = theme.colors();
        let p = self.progress.value() as f32;
        let panel_top = px(f32::from(self.height) * (1.0 - p));
        let toggle = self.toggle.clone();

        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .h(self.height)
            .flex_none()
            .overflow_hidden()
            .rounded(px(16.))
            .bg(colors.surface_container_low)
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(panel_top)
                    .bottom_0()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .rounded_tl(px(28.))
                    .rounded_tr(px(28.))
                    .bg(colors.surface_container)
                    .shadow(Elevation::Level1.shadows(colors.shadow))
                    .text_color(colors.on_surface)
                    .child(
                        div()
                            .id((self.id.clone(), "drawer-handle"))
                            .mt(px(10.))
                            .mx_auto()
                            .w(px(36.))
                            .h(px(4.))
                            .flex_none()
                            .rounded_full()
                            .cursor_pointer()
                            .bg(colors.on_surface_variant)
                            .on_click(move |_, window, cx| toggle(window, cx)),
                    )
                    .child((self.content)()),
            )
    }
}
