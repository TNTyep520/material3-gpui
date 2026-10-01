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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ExposedDropdownMenu.kt

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Window, div,
    prelude::*, px,
};
use std::rc::Rc;

type ExpandedChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::OutlinedTextFieldTokens;

#[derive(Clone, Copy, Debug)]
pub struct ExposedDropdownMenuStyle {
    pub menu_top_offset: Pixels,

    pub menu_reserved_height: Pixels,
}

impl ExposedDropdownMenuStyle {
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            menu_top_offset: OutlinedTextFieldTokens::CONTAINER_HEIGHT.pixels(),

            menu_reserved_height: px(220.),
        }
    }
}

#[derive(IntoElement)]
pub struct ExposedDropdownMenu {
    id: ElementId,
    field: AnyElement,
    menu: Option<AnyElement>,
    expanded: bool,
    on_expanded_change: Option<ExpandedChangeHandler>,
}

pub type ExposedDropdownMenuBox = ExposedDropdownMenu;

impl ExposedDropdownMenu {
    pub fn new(id: impl Into<ElementId>, field: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            field: field.into_any_element(),
            menu: None,
            expanded: false,
            on_expanded_change: None,
        }
    }

    pub fn menu(mut self, menu: impl IntoElement) -> Self {
        self.menu = Some(menu.into_any_element());
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn on_expanded_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_expanded_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ExposedDropdownMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = ExposedDropdownMenuStyle::resolve(cx.theme().token_set());
        let expanded = self.expanded;
        let anchor = div()
            .id((self.id.clone(), "anchor"))
            .child(self.field)
            .when_some(self.on_expanded_change, |el, handler| {
                el.cursor_pointer()
                    .on_click(move |_, window, cx| handler(!expanded, window, cx))
            });
        div()
            .id(self.id)
            .relative()
            .w_full()
            .when(expanded, |el| el.pb(style.menu_reserved_height))
            .child(anchor)
            .when(expanded, |el| {
                el.when_some(self.menu, |el, menu| {
                    el.child(
                        div()
                            .absolute()
                            .top(style.menu_top_offset)
                            .left_0()
                            .right_0()
                            .child(menu),
                    )
                })
            })
    }
}
