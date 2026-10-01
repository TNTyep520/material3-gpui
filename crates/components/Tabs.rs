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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3TabBarSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

pub struct Tab {
    pub label: SharedString,

    pub icon: Option<IconName>,

    pub enabled: bool,
    leading_icon: bool,
}

impl Tab {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            enabled: true,
            leading_icon: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self.leading_icon = true;
        self
    }
}

pub struct LeadingIconTab(Tab);

impl LeadingIconTab {
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self(Tab::new(label).leading_icon(icon))
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }
}

impl From<LeadingIconTab> for Tab {
    fn from(tab: LeadingIconTab) -> Self {
        tab.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabRowVariant {
    #[default]
    Primary,

    Secondary,
}

pub struct TabBar {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    variant: TabRowVariant,
    scrollable: bool,
    on_change: Option<ChangeHandler>,
}

pub struct TabBarState {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    variant: TabRowVariant,
    scrollable: bool,
    on_change: Option<ChangeHandler>,

    indicator: Animatable,
    driver: AnimationDriver,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected: 0,
            variant: TabRowVariant::default(),
            scrollable: false,
            on_change: None,
        }
    }

    pub fn tab(mut self, tab: impl Into<Tab>) -> Self {
        self.tabs.push(tab.into());
        self
    }

    pub fn tabs(mut self, tabs: impl IntoIterator<Item = impl Into<Tab>>) -> Self {
        self.tabs.extend(tabs.into_iter().map(Into::into));
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn variant(mut self, variant: TabRowVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<TabBarState> {
        let selected = self.selected.min(self.tabs.len().saturating_sub(1));
        cx.new(|_| TabBarState {
            id: self.id,
            tabs: self.tabs,
            selected,
            variant: self.variant,
            scrollable: self.scrollable,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

pub type TabRow = TabBar;

macro_rules! tab_row_variant {
    ($name:ident, $variant:ident, $scrollable:expr) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的标签行。")]
        pub struct $name(TabBar);

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(
                    TabBar::new(id)
                        .variant(TabRowVariant::$variant)
                        .scrollable($scrollable),
                )
            }

            pub fn tab(mut self, tab: impl Into<Tab>) -> Self {
                self.0 = self.0.tab(tab);
                self
            }

            pub fn tabs(mut self, tabs: impl IntoIterator<Item = impl Into<Tab>>) -> Self {
                self.0 = self.0.tabs(tabs);
                self
            }

            pub fn selected(mut self, index: usize) -> Self {
                self.0 = self.0.selected(index);
                self
            }

            pub fn on_change(
                mut self,
                handler: impl Fn(usize, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_change(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<TabBarState> {
                self.0.build(cx)
            }
        }
    };
}

tab_row_variant!(PrimaryTabRow, Primary, false);
tab_row_variant!(SecondaryTabRow, Secondary, false);
tab_row_variant!(PrimaryScrollableTabRow, Primary, true);
tab_row_variant!(SecondaryScrollableTabRow, Secondary, true);

impl TabBarState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.tabs.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for TabBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for TabBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let selected = self.selected;
        let on_change = self.on_change.clone();
        let entity = cx.entity();
        let style = TabBarStyle::resolve(
            theme.token_set(),
            self.tabs.iter().any(|tab| tab.icon.is_some()),
        );
        let label_style = style.label;
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let height = style.bar_height(has_icons);
        let indicator_pos = self.indicator.value() as f32;

        let primary = style.selected_item_color;
        let on_surface_variant = style.unselected_item_color;
        let surface = style.container_color;
        let outline_variant = style.divider_color;

        div()
            .id(self.id.clone())
            .w_full()
            .flex()
            .overflow_hidden()
            .when(self.scrollable, |el| el.overflow_x_scroll())
            .bg(surface)
            .border_b_1()
            .border_color(outline_variant)
            .children(self.tabs.iter().enumerate().map(|(ix, tab)| {
                let is_selected = ix == selected;
                let fg = if is_selected {
                    primary
                } else {
                    on_surface_variant
                };
                let layer = if is_selected {
                    primary
                } else {
                    colors.on_surface
                };
                let on_change = on_change.clone();
                let click_entity = entity.clone();
                let tab_el = div()
                    .id((SharedString::from(format!("{}-tab", self.id)), ix))
                    .relative()
                    .flex_1()
                    .when(self.scrollable, |el| el.flex_none().w(px(120.)))
                    .h(height)
                    .flex()
                    .flex_col()
                    .when(tab.leading_icon, |el| el.flex_row())
                    .items_center()
                    .justify_center()
                    .gap(style.gap)
                    .when(tab.enabled, |el| el.cursor_pointer())
                    .text_color(fg)
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
                    .when(tab.enabled, |el| {
                        el.on_click(move |_, window, cx| {
                            click_entity.update(cx, |state, cx| {
                                let changed = ix != state.selected;
                                state.select(ix, window, cx);
                                if changed && let Some(handler) = on_change.clone() {
                                    handler(ix, window, cx);
                                }
                            });
                        })
                    })
                    .when_some(tab.icon.clone(), |el, icon| {
                        el.child(Icon::new(icon).size(style.icon_size))
                    });
                let tab_el = label_style.apply(tab_el).child(tab.label.clone());

                tab_el.when(is_selected, |el| {
                    el.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left(relative(0.5 + indicator_pos - ix as f32))
                            .w(px(48.))
                            .ml(px(-24.))
                            .flex()
                            .justify_center()
                            .child(
                                div()
                                    .h(if self.variant == TabRowVariant::Secondary {
                                        px(2.)
                                    } else {
                                        px(3.)
                                    })
                                    .w(px(48.))
                                    .rounded_tl(px(3.))
                                    .rounded_tr(px(3.))
                                    .bg(primary),
                            ),
                    )
                })
            }))
    }
}

pub use appearance::TabBarStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct TabBarStyle {
        pub container_color: Hsla,

        pub divider_color: Hsla,

        pub selected_item_color: Hsla,

        pub unselected_item_color: Hsla,

        pub indicator_color: Hsla,

        pub indicator_size: (Pixels, Pixels),

        pub height: Pixels,

        pub height_with_icon: Pixels,

        pub icon_size: Pixels,

        pub gap: Pixels,

        pub hover_opacity: f32,

        pub pressed_opacity: f32,

        pub label: crate::theme::TypeStyle,
    }
    impl TabBarStyle {
        pub fn resolve(tokens: &TokenSet, _has_icons: bool) -> Self {
            let colors = &tokens.colors;
            Self {
                container_color: colors.surface,
                divider_color: colors.outline_variant,
                selected_item_color: colors.primary,
                unselected_item_color: colors.on_surface_variant,
                indicator_color: colors.primary,
                indicator_size: (px(3.), px(48.)),
                height: px(48.),
                height_with_icon: px(64.),
                icon_size: px(24.),
                gap: px(4.),
                hover_opacity: crate::theme::HOVER_OPACITY,
                pressed_opacity: crate::theme::PRESSED_OPACITY,
                label: tokens.typography.label_large,
            }
        }
    }
    impl TabBarStyle {
        pub fn bar_height(&self, has_icons: bool) -> Pixels {
            if has_icons {
                self.height_with_icon
            } else {
                self.height
            }
        }
    }
}
