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

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Styled, Window, div,
    prelude::*, px,
};

use crate::theme::ActiveTheme;

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    items: Vec<AnyElement>,
    selected: usize,
    on_select: Option<SelectHandler>,
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
}

impl ParentElement for Carousel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.items.extend(elements);
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        div()
            .id(self.id)
            .w_full()
            .h(px(232.))
            .flex_none()
            .flex()
            .gap(px(8.))
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let is_selected = index == selected;
                let is_neighbor = (index as i64 - selected as i64).abs() == 1;
                let handler = self.on_select.clone();
                div()
                    .id(("md3-carousel-item", index))
                    .h_full()
                    .flex_none()
                    .rounded(px(28.))
                    .overflow_hidden()
                    .bg(colors.surface_container_high)
                    .when(is_selected, |el| el.flex_1().min_w_0())
                    .when(!is_selected, |el| {
                        el.w(px(if is_neighbor { 120. } else { 56. }))
                    })
                    .when_some(handler, |el, handler| {
                        el.cursor_pointer().on_click(move |_, window, cx| {
                            handler(index, window, cx);
                        })
                    })
                    .child(
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(item),
                    )
            }))
    }
}
