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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButtonMenu.kt

use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, IntoElement, Render, WeakEntity, Window, div,
    prelude::*, px,
};

use crate::components::{Fab, FabState, FilledIconToggleButton};
use crate::icon::IconName;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::ActiveTheme;

pub struct FabMenu {
    id: ElementId,
    expanded: bool,
    items: Vec<Entity<FabState>>,
}

pub type FloatingActionButtonMenu = FabMenu;
pub type FloatingActionButtonMenuItem = Fab;
pub type ToggleFloatingActionButton = FilledIconToggleButton;

impl FabMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            items: Vec::new(),
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn action(mut self, item: Entity<FabState>) -> Self {
        self.items.push(item);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<FabMenuState> {
        cx.new(|cx| {
            let weak: WeakEntity<FabMenuState> = cx.entity().downgrade();
            let toggle_add = Fab::new((self.id.clone(), "toggle-add"), IconName::Add)
                .on_click({
                    let weak = weak.clone();
                    move |_, window, cx| {
                        if let Err(err) =
                            weak.update(cx, |menu, cx| menu.set_expanded(true, window, cx))
                        {
                            eprintln!("material3-gpui: {err}");
                        }
                    }
                })
                .build(cx);
            let toggle_close = Fab::new((self.id.clone(), "toggle-close"), IconName::Close)
                .on_click(move |_, window, cx| {
                    if let Err(err) =
                        weak.update(cx, |menu, cx| menu.set_expanded(false, window, cx))
                    {
                        eprintln!("material3-gpui: {err}");
                    }
                })
                .build(cx);
            FabMenuState {
                id: self.id,
                expanded: self.expanded,
                progress: Animatable::new(if self.expanded { 1.0 } else { 0.0 }, 1.0e-3),
                items: self.items,
                toggle_add,
                toggle_close,
                driver: AnimationDriver::default(),
            }
        })
    }
}

pub struct FabMenuState {
    id: ElementId,
    expanded: bool,
    progress: Animatable,
    items: Vec<Entity<FabState>>,
    toggle_add: Entity<FabState>,
    toggle_close: Entity<FabState>,
    driver: AnimationDriver,
}

impl FabMenuState {
    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub fn set_expanded(&mut self, expanded: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.expanded == expanded {
            return;
        }
        self.expanded = expanded;
        let role = if expanded {
            MotionRole::DefaultSpatial
        } else {
            MotionRole::FastSpatial
        };
        let spec = *cx.theme().motion().spec(role);
        self.progress
            .animate_to(if expanded { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for FabMenuState {
    fn step(&mut self, now: Instant) -> bool {
        self.progress.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for FabMenuState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        let progress = self.progress.value() as f32;
        let item_count = self.items.len();

        let mut column = div().flex().flex_col().items_end().gap(px(4.));
        for (index, item) in self.items.iter().enumerate() {
            let sink = px(16.0 * (item_count - index) as f32 * (1.0 - progress));
            column = column.child(
                div()
                    .relative()
                    .top(sink)
                    .opacity(progress)
                    .child(item.clone()),
            );
        }

        let toggle = if self.expanded {
            self.toggle_close.clone()
        } else {
            self.toggle_add.clone()
        };

        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .items_end()
            .gap(px(8.))
            .child(column)
            .child(toggle)
    }
}
