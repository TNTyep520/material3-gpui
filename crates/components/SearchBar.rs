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
    App, ElementId, Entity, IntoElement, ParentElement as _, RenderOnce, Styled, Window, div,
    prelude::*, px,
};

use crate::components::TextFieldState;
use crate::theme::ActiveTheme;

#[derive(IntoElement)]
pub struct SearchBar {
    id: ElementId,
    field: Entity<TextFieldState>,
    expanded: bool,
    suggestions: Vec<gpui::AnyElement>,
}
impl SearchBar {
    pub fn new(id: impl Into<ElementId>, field: Entity<TextFieldState>) -> Self {
        Self {
            id: id.into(),
            field,
            expanded: false,
            suggestions: Vec::new(),
        }
    }
    pub fn expanded(mut self, e: bool) -> Self {
        self.expanded = e;
        self
    }
}

impl ParentElement for SearchBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.suggestions.extend(elements);
    }
}
impl RenderOnce for SearchBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .min_h(px(56.))
            .when(!self.expanded, |el| el.rounded_full())
            .when(self.expanded, |el| el.rounded(px(28.)))
            .bg(c.surface_container_high)
            .overflow_hidden()
            .flex()
            .flex_col()
            .text_color(c.on_surface)
            .child(self.field)
            .when(self.expanded, |el| el.children(self.suggestions))
    }
}
