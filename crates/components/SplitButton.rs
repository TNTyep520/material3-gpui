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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SplitButton.kt

use crate::components::overlay::show_menu;
use crate::components::{Button, ButtonVariant, IconButton, IconButtonVariant};
use crate::icon::IconName;
use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::SplitButtonSmallTokens;
use gpui::{
    App, ClickEvent, ElementId, Entity, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px, size,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, Debug)]
pub struct SplitButtonStyle {
    pub between_space: Pixels,
}

impl SplitButtonStyle {
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            between_space: SplitButtonSmallTokens::BETWEEN_SPACE.pixels(),
        }
    }
}

#[derive(IntoElement)]
pub struct SplitButton {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    trailing: IconName,
    on_click: Option<ClickHandler>,
    on_trailing_click: Option<ClickHandler>,
    menu: Option<Entity<crate::components::overlay::MenuState>>,
}

pub type SplitButtonLayout = SplitButton;

impl SplitButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Filled,
            trailing: IconName::new("chevron_right"),
            on_click: None,
            on_trailing_click: None,
            menu: None,
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }
    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }
    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing = icon;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_trailing_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_trailing_click = Some(Box::new(handler));
        self
    }

    pub fn menu(mut self, menu: Entity<crate::components::overlay::MenuState>) -> Self {
        self.menu = Some(menu);
        self
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = SplitButtonStyle::resolve(cx.theme().token_set());
        let mut primary =
            Button::new((self.id.clone(), "primary"), self.label).variant(self.variant);
        if let Some(handler) = self.on_click {
            primary = primary.on_click(handler);
        }
        let primary = primary.build(cx);
        let mut trailing =
            IconButton::new((self.id, "menu"), self.trailing).variant(IconButtonVariant::Standard);
        let menu = self.menu;
        let on_trailing_click = self.on_trailing_click;
        trailing = trailing.on_click(move |event, window, cx| {
            if let Some(handler) = &on_trailing_click {
                handler(event, window, cx);
            }
            if let Some(menu) = &menu {
                let anchor = gpui::Bounds {
                    origin: event.position(),
                    size: size(px(0.), px(0.)),
                };
                show_menu(window, cx, menu.clone(), anchor);
            }
        });
        let trailing = trailing.build(cx);
        div()
            .flex()
            .items_center()
            .gap(style.between_space)
            .child(primary)
            .child(trailing)
    }
}
