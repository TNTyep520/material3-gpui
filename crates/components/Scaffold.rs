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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scaffold.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, Styled, Window, div,
    prelude::*, px,
};

use crate::prelude::ActiveTheme;

#[derive(IntoElement)]
pub struct Scaffold {
    id: ElementId,
    top_bar: Option<AnyElement>,
    bottom_bar: Option<AnyElement>,
    fab: Option<AnyElement>,
    snackbar_host: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Scaffold {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            top_bar: None,
            bottom_bar: None,
            fab: None,
            snackbar_host: None,
            children: Vec::new(),
        }
    }

    pub fn top_bar(mut self, top_bar: impl IntoElement) -> Self {
        self.top_bar = Some(top_bar.into_any_element());
        self
    }

    pub fn bottom_bar(mut self, bottom_bar: impl IntoElement) -> Self {
        self.bottom_bar = Some(bottom_bar.into_any_element());
        self
    }

    pub fn fab(mut self, fab: impl IntoElement) -> Self {
        self.fab = Some(fab.into_any_element());
        self
    }

    pub fn snackbar_host(mut self, host: impl IntoElement) -> Self {
        self.snackbar_host = Some(host.into_any_element());
        self
    }
}

impl ParentElement for Scaffold {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Scaffold {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let colors = theme.colors();

        let content = div()
            .relative()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(colors.surface)
            .children(self.children)
            .when_some(self.fab, |el, fab| {
                el.child(div().absolute().right(px(16.)).bottom(px(16.)).child(fab))
            })
            .when_some(self.snackbar_host, |el, host| {
                el.child(div().absolute().left(px(16.)).bottom(px(16.)).child(host))
            });

        div()
            .id(self.id)
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.surface)
            .when_some(self.top_bar, |el, top_bar| el.child(top_bar))
            .child(content)
            .when_some(self.bottom_bar, |el, bottom_bar| el.child(bottom_bar))
    }
}
