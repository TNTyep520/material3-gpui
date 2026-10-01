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

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, Entity, IntoElement, Render, Styled,
    Window, div, prelude::*, px,
};

use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::ActiveTheme;

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

const LARGE_ITEM_WIDTH: f32 = 320.;
const MEDIUM_ITEM_WIDTH: f32 = 120.;
const SMALL_ITEM_WIDTH: f32 = 56.;

type ItemFactory = Rc<dyn Fn() -> AnyElement>;

pub struct Carousel {
    id: ElementId,
    items: Vec<ItemFactory>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

pub struct CarouselState {
    id: ElementId,
    items: Vec<ItemFactory>,
    selected: usize,
    position: Animatable,
    on_select: Option<SelectHandler>,
    driver: AnimationDriver,
}

impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_select: None,
        }
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn item(mut self, item: impl Fn() -> AnyElement + 'static) -> Self {
        self.items.push(Rc::new(item));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<CarouselState> {
        let position = self.selected as f64;
        cx.new(|_| CarouselState {
            id: self.id,
            items: self.items,
            selected: self.selected,
            position: Animatable::new(position, 1.0e-3),
            on_select: self.on_select,
            driver: AnimationDriver::default(),
        })
    }
}

impl CarouselState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn set_selected(&mut self, selected: usize, window: &mut Window, cx: &mut Context<Self>) {
        let selected = selected.min(self.items.len().saturating_sub(1));
        if self.selected == selected {
            return;
        }
        self.selected = selected;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
        self.position
            .animate_to(selected as f64, &spec, Instant::now());
        if self.position.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    fn item_width(&self, index: usize, position: f32) -> f32 {
        let distance = (index as f32 - position).abs();
        if distance <= 1. {
            MEDIUM_ITEM_WIDTH + (1. - distance) * (LARGE_ITEM_WIDTH - MEDIUM_ITEM_WIDTH)
        } else if distance < 2. {
            SMALL_ITEM_WIDTH + (2. - distance) * (MEDIUM_ITEM_WIDTH - SMALL_ITEM_WIDTH)
        } else {
            SMALL_ITEM_WIDTH
        }
    }
}

impl AnimatedComponent for CarouselState {
    fn step(&mut self, now: Instant) -> bool {
        self.position.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for CarouselState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.position.is_running() {
            self.schedule_next(window, cx);
        }
        let colors = cx.theme().colors();
        let position = self.position.value() as f32;
        let entity = cx.entity();
        div()
            .id(self.id.clone())
            .w_full()
            .min_w_0()
            .h(px(232.))
            .flex_none()
            .flex()
            .gap(px(8.))
            .overflow_hidden()
            .children(self.items.iter().enumerate().map(|(index, item)| {
                let is_selected = index == self.selected;
                let handler = self.on_select.clone();
                let select_entity = entity.clone();
                let width = px(self.item_width(index, position));
                div()
                    .id(("md3-carousel-item", index))
                    .h_full()
                    .flex_none()
                    .w(width)
                    .rounded(px(28.))
                    .overflow_hidden()
                    .bg(colors.surface_container_high)
                    .opacity(if is_selected {
                        1.
                    } else {
                        (1. - (index as f32 - position).abs() * 0.25).clamp(0.55, 1.)
                    })
                    .cursor_pointer()
                    .on_click(move |_, window, cx| {
                        select_entity.update(cx, |state, cx| {
                            state.set_selected(index, window, cx);
                        });
                        if let Some(handler) = &handler {
                            handler(index, window, cx);
                        }
                    })
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(item()),
                    )
            }))
    }
}
