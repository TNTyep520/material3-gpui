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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3NavigationBarSkin.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3NavigationRailSkin.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3NavigationDrawerSkin.java

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

trait NavSelection: 'static {
    fn selected_index(&self) -> usize;

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>)
    where
        Self: Sized;
}

impl NavSelection for NavigationBarState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationRailState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationDrawerState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

#[derive(Clone)]
pub struct NavigationItemSpec {
    pub label: SharedString,

    pub icon: Option<IconName>,

    pub badge: Option<SharedString>,

    pub enabled: bool,
    on_click: Option<ItemClickHandler>,
}

type ItemClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;

pub type NavigationBarItem = NavigationItemSpec;

pub type NavigationRailItem = NavigationItemSpec;

pub type NavigationDrawerItem = NavigationItemSpec;

impl NavigationItemSpec {
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            label: label.into(),
            icon: Some(icon),
            badge: None,
            enabled: true,
            on_click: None,
        }
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

#[allow(clippy::too_many_arguments)]
fn navigation_item<T: NavSelection>(
    id: impl Into<ElementId>,
    spec: &NavigationItemSpec,
    selected: bool,
    horizontal: bool,
    indicator_offset: Option<f32>,
    item: &NavigationItemStyle,
    entity: &Entity<T>,
    on_change: Option<&ChangeHandler>,
    ix: usize,
) -> gpui::Stateful<gpui::Div> {
    let icon_color = if selected {
        item.selected_icon_color
    } else {
        item.unselected_icon_color
    };
    let label_color = if selected {
        item.selected_label_color
    } else {
        item.unselected_label_color
    };
    let (indicator_w, indicator_h) = item.indicator_size;
    let hover = item.hover_opacity;
    let pressed = item.pressed_opacity;
    let indicator_color = item.indicator_color;
    let click_entity = entity.clone();
    let on_change = on_change.cloned();
    let item_on_click = spec.on_click.clone();
    let enabled = spec.enabled;
    let interaction_group = SharedString::from(format!("navigation-{:?}-{ix}", entity.entity_id()));

    let pill = indicator_offset.map(|offset| {
        div()
            .absolute()
            .left(relative(0.5 + offset))
            .ml(-indicator_w / 2.0)
            .top(px(0.))
            .w(indicator_w)
            .h(indicator_h)
            .flex_none()
            .rounded(item.indicator_radius)
            .bg(indicator_color)
    });

    let base = div()
        .id(id)
        .group(interaction_group.clone())
        .relative()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(item_gap_from(horizontal)))
        .when(enabled, |el| el.cursor_pointer())
        .text_color(label_color)
        .child(
            div()
                .relative()
                .w_full()
                .h(indicator_h)
                .flex()
                .items_center()
                .justify_center()
                .when_some(pill, |el, pill| el.child(pill))
                .child(
                    div()
                        .id("navigation-state-layer")
                        .absolute()
                        .left(relative(0.5))
                        .ml(-indicator_w / 2.0)
                        .top(px(0.))
                        .w(indicator_w)
                        .h(indicator_h)
                        .rounded(item.indicator_radius)
                        .group_hover(interaction_group.clone(), move |style| {
                            style.bg(icon_color.opacity(hover))
                        })
                        .group_active(interaction_group, move |style| {
                            style.bg(icon_color.opacity(pressed))
                        }),
                )
                .when_some(spec.icon, |el, icon| {
                    el.child(Icon::new(icon).size(item.icon_size).color(icon_color))
                }),
        )
        .when(enabled, |el| {
            el.on_click(move |_, window, cx| {
                click_entity.update(cx, |state, cx| {
                    let changed = ix != state.selected_index();
                    state.select_index(ix, window, cx);
                    if changed && let Some(handler) = on_change.clone() {
                        handler(ix, window, cx);
                    }
                });
                if let Some(handler) = &item_on_click {
                    handler(window, cx);
                }
            })
        });

    let base = item.label.apply(base);
    base.child(spec.label.clone())
}

fn item_gap_from(horizontal: bool) -> f32 {
    if horizontal { 16. } else { 4. }
}

pub struct NavigationBar {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
}

pub struct NavigationBarState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
        }
    }

    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<NavigationBarState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationBarState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationBarState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
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

impl AnimatedComponent for NavigationBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let bar = NavigationBarStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let indicator_pos = self.indicator.value() as f32;
        let selected = self.selected;
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w_full()
            .h(bar.height)
            .flex()
            .flex_none()
            .overflow_hidden()
            .bg(bar.container_color)
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;

                let offset = if is_selected {
                    Some(indicator_pos - ix as f32)
                } else {
                    None
                };
                let item_el = navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    false,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                );
                item_el
                    .flex_1()
                    .h_full()
                    .justify_center()
                    .px(bar.item_padding)
            }))
    }
}

pub struct NavigationRail {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
}

use crate::components::fab::FabState;

pub struct NavigationRailState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
            header: None,
        }
    }

    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn header(mut self, header: Entity<FabState>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<NavigationRailState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationRailState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            header: self.header,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationRailState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
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

impl AnimatedComponent for NavigationRailState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationRailState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let rail = NavigationRailStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let selected = self.selected;
        let header = self.header.clone();
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w(rail.width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .items_center()
            .pt(rail.top_padding)
            .gap(rail.item_gap)
            .bg(rail.container_color)
            .overflow_y_scroll()
            .children(header.map(|h| div().pb(px(16.)).child(h)))
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;

                let offset = if is_selected { Some(0.0) } else { None };
                navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    true,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                )
                .w_full()
                .py(rail.item_gap)
            }))
    }
}

pub enum DrawerEntry {
    Section(SharedString),

    Item(NavigationItemSpec),
}

pub struct NavigationDrawer {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
}

pub struct NavigationDrawerState {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
    driver: AnimationDriver,
}

impl NavigationDrawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
            selected: 0,
            modal: false,
            on_change: None,
        }
    }

    pub fn entry(mut self, entry: DrawerEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.entries.push(DrawerEntry::Item(item));
        self
    }

    pub fn section(mut self, title: impl Into<SharedString>) -> Self {
        self.entries.push(DrawerEntry::Section(title.into()));
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<NavigationDrawerState> {
        cx.new(|_| NavigationDrawerState {
            id: self.id,
            entries: self.entries,
            selected: self.selected,
            modal: self.modal,
            on_change: self.on_change,
            driver: AnimationDriver::default(),
        })
    }
}

macro_rules! navigation_drawer_variant {
    ($name:ident, $modal:expr) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的导航抽屉。")]
        pub struct $name(NavigationDrawer);

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(NavigationDrawer::new(id).modal($modal))
            }

            pub fn item(mut self, item: NavigationDrawerItem) -> Self {
                self.0 = self.0.item(item);
                self
            }

            pub fn section(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.section(title);
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

            pub fn build(self, cx: &mut App) -> Entity<NavigationDrawerState> {
                self.0.build(cx)
            }
        }
    };
}

navigation_drawer_variant!(ModalNavigationDrawer, true);
navigation_drawer_variant!(PermanentNavigationDrawer, false);
navigation_drawer_variant!(DismissibleNavigationDrawer, false);

pub type ModalDrawerSheet = ModalNavigationDrawer;

pub type PermanentDrawerSheet = PermanentNavigationDrawer;

pub type DismissibleDrawerSheet = DismissibleNavigationDrawer;

impl AnimatedComponent for NavigationDrawerState {
    fn step(&mut self, _now: Instant) -> bool {
        false
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl NavigationDrawerState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self
            .entries
            .iter()
            .filter(|entry| matches!(entry, DrawerEntry::Item(_)))
            .count();
        if index >= item_count || index == self.selected {
            return;
        }
        self.selected = index;
        cx.notify();
    }
}

impl Render for NavigationDrawerState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let drawer = NavigationDrawerStyle::resolve(theme.token_set(), self.modal);
        let item = NavigationItemStyle::resolve(theme.token_set());
        let width = drawer.drawer_width(self.modal);
        let entity = cx.entity();
        let mut item_ix = 0usize;

        let mut column = div()
            .id(self.id.clone())
            .w(width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .gap(px(4.))
            .p(drawer.padding)
            .bg(drawer.container_color)
            .overflow_y_scroll()
            .when(self.modal, |el| {
                el.shadow(drawer.modal_elevation.shadows(drawer.shadow_color))
            });

        for entry in self.entries.iter() {
            match entry {
                DrawerEntry::Section(title) => {
                    let el = drawer
                        .section_header
                        .apply(div())
                        .px(drawer.item_horizontal_padding)
                        .pt(px(16.))
                        .text_color(drawer.section_header_color)
                        .child(title.clone());
                    column = column.child(el);
                }
                DrawerEntry::Item(spec) => {
                    let ix = item_ix;
                    let is_selected = ix == self.selected;
                    let on_change = self.on_change.clone();
                    let click_entity = entity.clone();
                    let icon_color = if is_selected {
                        item.selected_icon_color
                    } else {
                        item.unselected_icon_color
                    };
                    let label_color = if is_selected {
                        item.selected_label_color
                    } else {
                        item.unselected_label_color
                    };
                    let hover = item.hover_opacity;
                    let pressed = item.pressed_opacity;

                    let el = div()
                        .id((SharedString::from(format!("{}-item", self.id)), ix))
                        .w_full()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.))
                        .h(px(56.))
                        .rounded(item.indicator_radius)
                        .px(drawer.item_horizontal_padding)
                        .cursor_pointer()
                        .text_color(label_color)
                        .when(is_selected, |el| el.bg(item.indicator_color))
                        .when(!is_selected, |el| {
                            el.hover(move |s| s.bg(icon_color.opacity(hover)))
                        })
                        .active(move |s| s.bg(icon_color.opacity(pressed)))
                        .when_some(spec.icon, |el, icon| {
                            el.child(
                                div()
                                    .flex_none()
                                    .child(Icon::new(icon).size(item.icon_size).color(icon_color)),
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(item.label.apply(div()).child(spec.label.clone())),
                        )
                        .when_some(spec.badge.clone(), |el, badge| {
                            el.child(
                                crate::components::Badge::new((
                                    SharedString::from(format!("{}-badge", self.id)),
                                    ix,
                                ))
                                .label(badge),
                            )
                        })
                        .on_click(move |_, window, cx| {
                            click_entity.update(cx, |state, cx| {
                                let changed = ix != state.selected;
                                state.select(ix, window, cx);
                                if changed && let Some(handler) = on_change.clone() {
                                    handler(ix, window, cx);
                                }
                            })
                        });
                    column = column.child(el);
                    item_ix += 1;
                }
            }
        }
        column
    }
}

pub use appearance::{
    NavigationBarStyle, NavigationDrawerStyle, NavigationItemStyle, NavigationRailStyle,
};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct NavigationItemStyle {
        pub indicator_size: (Pixels, Pixels),

        pub indicator_radius: Pixels,

        pub indicator_color: Hsla,

        pub selected_icon_color: Hsla,

        pub unselected_icon_color: Hsla,

        pub selected_label_color: Hsla,

        pub unselected_label_color: Hsla,

        pub icon_size: Pixels,

        pub label: crate::theme::TypeStyle,

        pub hover_opacity: f32,

        pub pressed_opacity: f32,
    }
    impl NavigationItemStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            Self {
                indicator_size: (px(64.), px(32.)),
                indicator_radius: tokens.shapes.full,
                indicator_color: colors.secondary_container,
                selected_icon_color: colors.on_secondary_container,
                unselected_icon_color: colors.on_surface_variant,
                selected_label_color: colors.on_surface,
                unselected_label_color: colors.on_surface_variant,
                icon_size: px(24.),
                label: tokens.typography.label_medium,
                hover_opacity: crate::theme::HOVER_OPACITY,
                pressed_opacity: crate::theme::PRESSED_OPACITY,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct NavigationBarStyle {
        pub container_color: Hsla,

        pub height: Pixels,

        pub item_gap: Pixels,

        pub item_padding: Pixels,
    }
    impl NavigationBarStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            Self {
                container_color: tokens.colors.surface_container,
                height: px(80.),
                item_gap: px(4.),
                item_padding: px(8.),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct NavigationRailStyle {
        pub container_color: Hsla,

        pub width: Pixels,

        pub item_gap: Pixels,

        pub top_padding: Pixels,
    }
    impl NavigationRailStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            Self {
                container_color: tokens.colors.surface_container,
                width: px(80.),
                item_gap: px(4.),
                top_padding: px(8.),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct NavigationDrawerStyle {
        pub container_color: Hsla,

        pub width: Pixels,

        pub modal_width: Pixels,

        pub section_header_color: Hsla,

        pub padding: Pixels,

        pub item_horizontal_padding: Pixels,

        pub shadow_color: Hsla,

        pub modal_elevation: crate::theme::Elevation,

        pub section_header: crate::theme::TypeStyle,
    }
    impl NavigationDrawerStyle {
        pub fn resolve(tokens: &TokenSet, _modal: bool) -> Self {
            Self {
                container_color: tokens.colors.surface_container_low,
                width: px(360.),
                modal_width: px(360.),
                section_header_color: tokens.colors.on_surface_variant,
                padding: px(12.),
                item_horizontal_padding: px(12.),
                shadow_color: tokens.colors.shadow,
                modal_elevation: crate::theme::Elevation::Level1,
                section_header: tokens.typography.title_small,
            }
        }

        pub fn drawer_width(&self, modal: bool) -> Pixels {
            if modal { self.modal_width } else { self.width }
        }
    }
}
