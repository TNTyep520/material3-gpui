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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material/material/src/commonMain/kotlin/androidx/compose/material/BackdropScaffold.kt

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

pub struct BackdropScaffold {
    id: ElementId,
    back_content: ContentFactory,
    front_content: ContentFactory,
    peek_height: gpui::Pixels,
    height: gpui::Pixels,
    revealed: bool,
}

pub struct BackdropScaffoldState {
    id: ElementId,
    back_content: ContentFactory,
    front_content: ContentFactory,
    toggle: ToggleHandler,
    peek_height: gpui::Pixels,
    height: gpui::Pixels,
    revealed: bool,
    progress: Animatable,
    driver: AnimationDriver,
}

impl BackdropScaffold {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            back_content: Rc::new(|| div().into_any_element()),
            front_content: Rc::new(|| div().into_any_element()),
            peek_height: px(56.),
            height: px(420.),
            revealed: false,
        }
    }

    pub fn back_content(mut self, content: impl Fn() -> AnyElement + 'static) -> Self {
        self.back_content = Rc::new(content);
        self
    }

    pub fn front_content(mut self, content: impl Fn() -> AnyElement + 'static) -> Self {
        self.front_content = Rc::new(content);
        self
    }

    pub fn peek_height(mut self, height: gpui::Pixels) -> Self {
        self.peek_height = height;
        self
    }

    pub fn height(mut self, height: gpui::Pixels) -> Self {
        self.height = height;
        self
    }

    pub fn revealed(mut self, revealed: bool) -> Self {
        self.revealed = revealed;
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<BackdropScaffoldState> {
        let revealed = self.revealed;
        cx.new(|cx| {
            let weak: WeakEntity<BackdropScaffoldState> = cx.entity().downgrade();
            let toggle: ToggleHandler = Rc::new(move |window, cx| {
                weak.update(cx, |state, cx| {
                    let next = !state.revealed;
                    state.set_revealed(next, window, cx);
                })
                .ok();
            });
            BackdropScaffoldState {
                id: self.id,
                back_content: self.back_content,
                front_content: self.front_content,
                toggle,
                peek_height: self.peek_height,
                height: self.height,
                revealed,
                progress: Animatable::new(if revealed { 1.0 } else { 0.0 }, 1.0e-3),
                driver: AnimationDriver::default(),
            }
        })
    }
}

impl BackdropScaffoldState {
    pub fn revealed(&self) -> bool {
        self.revealed
    }

    pub fn set_revealed(&mut self, revealed: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.revealed == revealed {
            return;
        }
        self.revealed = revealed;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
        self.progress
            .animate_to(if revealed { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for BackdropScaffoldState {
    fn step(&mut self, now: Instant) -> bool {
        self.progress.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for BackdropScaffoldState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let colors = theme.colors();
        let p = self.progress.value() as f32;
        let reveal_height = f32::from(self.height) - f32::from(self.peek_height);
        let back_height = self.peek_height + px(reveal_height * p);
        let front_offset = px(reveal_height * p);
        let toggle = self.toggle.clone();

        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .h(self.height)
            .flex_none()
            .overflow_hidden()
            .bg(colors.surface)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(back_height)
                    .bg(colors.primary)
                    .text_color(colors.on_primary)
                    .child((self.back_content)()),
            )
            .child(
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(front_offset)
                    .bottom_0()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .rounded_tl(px(28.))
                    .rounded_tr(px(28.))
                    .bg(colors.surface)
                    .shadow(Elevation::Level1.shadows(colors.shadow))
                    .text_color(colors.on_surface)
                    .child(
                        div()
                            .id((self.id.clone(), "backdrop-handle"))
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
                    .child((self.front_content)()),
            )
    }
}
