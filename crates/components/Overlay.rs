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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/controls/M3OverlayPane.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3MenuSkin.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/skins/M3TooltipSkin.java

use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, AnyWindowHandle, App, AppContext as _, Bounds, Context,
    ElementId, Entity, Global, InteractiveElement as _, IntoElement, ParentElement as _, Pixels,
    Render, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::interaction::BoundsHandle;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::{ActiveTheme, Elevation};

type ActionHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Default)]
pub struct OverlayRegistry {
    hosts: HashMap<AnyWindowHandle, Entity<OverlayHostState>>,
}

impl Global for OverlayRegistry {}

pub fn host(window: &Window, cx: &mut App) -> Entity<OverlayHostState> {
    if !cx.has_global::<OverlayRegistry>() {
        cx.set_global(OverlayRegistry::default());
    }
    let handle: AnyWindowHandle = window.window_handle();
    if let Some(existing) = cx.global::<OverlayRegistry>().hosts.get(&handle) {
        return existing.clone();
    }
    let host = cx.new(|_| OverlayHostState::default());
    cx.global_mut::<OverlayRegistry>()
        .hosts
        .insert(handle, host.clone());
    host
}

pub fn show_snackbar(
    window: &Window,
    cx: &mut App,
    snackbar: Snackbar,
    duration: Option<Duration>,
) {
    let host_entity = host(window, cx);
    let duration = duration.unwrap_or(Duration::from_secs(5));
    host_entity.update(cx, |host, cx| {
        let id = host.next_id;
        host.next_id += 1;
        host.snacks.push(SnackState::new(id, snackbar, cx));
        cx.notify();
    });

    let timer_host = host_entity.clone();
    cx.spawn(async move |cx| {
        cx.background_executor().timer(duration).await;
        timer_host.update(cx, |host, cx| {
            host.dismiss_snack_top(cx);
        })
    })
    .detach_and_log_err(cx);
}

pub fn show_menu(window: &Window, cx: &mut App, menu: Entity<MenuState>, anchor: Bounds<Pixels>) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.menu = Some((menu, anchor));
        cx.notify();
    });
}

pub fn close_menu(window: &Window, cx: &mut App) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.menu = None;
        cx.notify();
    });
}

pub fn show_tooltip(
    window: &Window,
    cx: &mut App,
    text: impl Into<SharedString>,
    anchor: Bounds<Pixels>,
) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.tooltip = Some(Tooltip {
            title: None,
            text: text.into(),
            anchor,
        });
        cx.notify();
    });
}

pub fn show_rich_tooltip(
    window: &Window,
    cx: &mut App,
    title: impl Into<SharedString>,
    text: impl Into<SharedString>,
    anchor: Bounds<Pixels>,
) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.tooltip = Some(Tooltip {
            title: Some(title.into()),
            text: text.into(),
            anchor,
        });
        cx.notify();
    });
}

pub fn close_tooltip(window: &Window, cx: &mut App) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.tooltip = None;
        cx.notify();
    });
}

#[derive(Default)]
pub struct OverlayHostState {
    next_id: u64,
    snacks: Vec<SnackState>,
    menu: Option<(Entity<MenuState>, Bounds<Pixels>)>,
    tooltip: Option<Tooltip>,
    driver: AnimationDriver,
}

pub type SnackbarHost = OverlayHostState;

#[derive(IntoElement)]
pub struct TooltipBox {
    id: ElementId,
    anchor: AnyElement,
    text: SharedString,
    title: Option<SharedString>,
}

impl TooltipBox {
    pub fn new(
        id: impl Into<ElementId>,
        anchor: impl IntoElement,
        text: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            anchor: anchor.into_any_element(),
            text: text.into(),
            title: None,
        }
    }

    pub fn plain(id: impl Into<ElementId>, anchor: impl IntoElement, tip: PlainTooltip) -> Self {
        Self::new(id, anchor, tip.text)
    }

    pub fn rich(id: impl Into<ElementId>, anchor: impl IntoElement, tip: RichTooltip) -> Self {
        let mut box_element = Self::new(id, anchor, tip.text);
        box_element.title = Some(tip.title);
        box_element
    }
}

pub struct PlainTooltip {
    text: SharedString,
}

impl PlainTooltip {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

pub struct RichTooltip {
    title: SharedString,
    text: SharedString,
}

impl RichTooltip {
    pub fn new(title: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            text: text.into(),
        }
    }
}

impl RenderOnce for TooltipBox {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let bounds = BoundsHandle::new();
        let hover_bounds = bounds.clone();
        div()
            .id(self.id)
            .relative()
            .on_hover(move |hovered, window, cx| {
                if *hovered {
                    if let Some(title) = &self.title {
                        show_rich_tooltip(
                            window,
                            cx,
                            title.clone(),
                            self.text.clone(),
                            hover_bounds.get(),
                        );
                    } else {
                        show_tooltip(window, cx, self.text.clone(), hover_bounds.get());
                    }
                } else {
                    close_tooltip(window, cx);
                }
            })
            .child(self.anchor)
            .child(bounds.capture_element())
    }
}

impl OverlayHostState {
    fn dismiss_snack_top(&mut self, cx: &mut Context<Self>) {
        if let Some(snack) = self.snacks.last_mut() {
            snack.begin_exit(cx);
        }

        cx.notify();
    }
}

#[derive(Clone)]
struct Tooltip {
    title: Option<SharedString>,
    text: SharedString,
    anchor: Bounds<Pixels>,
}

impl AnimatedComponent for OverlayHostState {
    fn step(&mut self, now: Instant) -> bool {
        let mut animating = false;
        for snack in &mut self.snacks {
            snack.tick(now);
            animating |= snack.is_animating();
        }

        self.snacks.retain(|s| !s.removed);
        animating
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for OverlayHostState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.snacks.retain(|snack| !snack.removed);
        if self.snacks.iter().any(SnackState::is_animating) {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let style = SnackbarStyle::resolve(theme.token_set());
        let tooltip_style = TooltipStyle::resolve(theme.token_set());

        let snack_el = self.snacks.last().map(|snack| {
            let p = snack.progress.value() as f32;
            let bottom = style.bottom_offset - px(64.) * (1.0 - p);
            let bg = lerp_color(gpui::Hsla::transparent_black(), style.container_color, p);
            let fg = style.text_color.opacity(p);
            let action_label = snack.action_label.clone();
            let action_handler = snack.on_action.clone();
            let snack_id = snack.id;
            let action_click = action_handler.clone();
            div()
                .id(("snack", snack_id))
                .absolute()
                .bottom(bottom)
                .left(relative(0.5))
                .ml(style.width / -2.)
                .w(style.width)
                .min_h(style.min_height)
                .flex()
                .flex_none()
                .items_center()
                .gap(style.action_gap)
                .px(style.padding.0)
                .py(style.padding.1)
                .rounded(style.corner_radius)
                .bg(bg)
                .shadow(Elevation::Level3.shadows(colors.shadow))
                .child(
                    div()
                        .flex_1()
                        .text_size(theme.typography().body_medium.size)
                        .text_color(fg)
                        .child(snack.message.clone()),
                )
                .when_some(action_label.filter(|_| p > 0.9), move |el, label| {
                    el.child(
                        div()
                            .id(("snack-action", snack_id))
                            .cursor_pointer()
                            .px(px(8.))
                            .py(px(4.))
                            .rounded(theme.shapes().extra_small)
                            .text_size(theme.typography().label_large.size)
                            .text_color(style.action_color.opacity(p))
                            .hover(move |s| s.bg(style.action_color.opacity(0.08 * p)))
                            .on_click(move |_event, window, cx| {
                                if let Some(handler) = action_click.clone() {
                                    handler(window, cx);
                                }
                                let host_entity = host(window, cx);
                                host_entity.update(cx, |h, cx| {
                                    h.dismiss_snack_top(cx);
                                });
                            })
                            .child(label),
                    )
                })
        });

        let menu_el = self.menu.clone().map(|(menu_entity, anchor)| {
            div()
                .absolute()
                .left(anchor.origin.x)
                .top(anchor.origin.y + anchor.size.height + px(4.))
                .child(div().relative().child(menu_entity).with_animation(
                    "md3-menu-enter",
                    Animation::new(Duration::from_millis(150)),
                    move |el, delta| el.opacity(delta).top(px((delta - 1.) * 8.)),
                ))
                .on_mouse_down_out({
                    let host_entity = cx.entity();
                    move |_event: &gpui::MouseDownEvent, _window, cx| {
                        host_entity.update(cx, |host, cx| {
                            host.menu = None;
                            cx.notify();
                        });
                    }
                })
        });

        let tooltip_el = self.tooltip.clone().map(|tip| {
            div()
                .absolute()
                .left(tip.anchor.origin.x + tip.anchor.size.width / 2.0)
                .top(tip.anchor.origin.y + tip.anchor.size.height + tooltip_style.anchor_gap)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .min_h(tooltip_style.height)
                        .px(tooltip_style.horizontal_padding)
                        .py(px(8.))
                        .rounded(tooltip_style.corner_radius)
                        .bg(tooltip_style.container_color)
                        .text_size(tooltip_style.text.size)
                        .text_color(tooltip_style.text_color)
                        .when_some(tip.title, |el, title| {
                            el.child(
                                div()
                                    .text_size(theme.typography().title_small.size)
                                    .child(title),
                            )
                        })
                        .child(tip.text)
                        .with_animation(
                            "md3-tooltip-fade",
                            Animation::new(Duration::from_millis(120)),
                            |el, delta| el.opacity(delta),
                        ),
                )
        });

        div()
            .absolute()
            .inset_0()
            .flex_none()
            .when_some(snack_el, |el, s| el.child(s))
            .when_some(menu_el, |el, m| el.child(m))
            .when_some(tooltip_el, |el, t| el.child(t))
    }
}

pub struct Snackbar {
    message: SharedString,
    action_label: Option<SharedString>,
    on_action: Option<ActionHandler>,
}

impl Snackbar {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            action_label: None,
            on_action: None,
        }
    }

    pub fn action(mut self, label: impl Into<SharedString>) -> Self {
        self.action_label = Some(label.into());
        self
    }

    pub fn on_action(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_action = Some(Rc::new(handler));
        self
    }
}

struct SnackState {
    id: u64,
    message: SharedString,
    action_label: Option<SharedString>,
    on_action: Option<ActionHandler>,

    progress: Animatable,
    exiting: bool,
    removed: bool,
}

impl SnackState {
    fn new(id: u64, snackbar: Snackbar, cx: &mut Context<OverlayHostState>) -> Self {
        let spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
        let mut progress = Animatable::new(0.0, 1.0e-3);
        progress.animate_to(1.0, &spec, Instant::now());
        Self {
            id,
            message: snackbar.message,
            action_label: snackbar.action_label,
            on_action: snackbar.on_action,
            progress,
            exiting: false,
            removed: false,
        }
    }

    fn begin_exit(&mut self, cx: &mut Context<OverlayHostState>) {
        if self.exiting {
            return;
        }
        self.exiting = true;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultSpatial);
        self.progress.animate_to(0.0, &spec, Instant::now());
    }

    fn tick(&mut self, now: Instant) {
        if !self.progress.tick(now) && self.exiting {
            self.removed = true;
        }
    }

    fn is_animating(&self) -> bool {
        self.progress.is_running()
    }
}

pub struct MenuItem {
    label: SharedString,
    icon: Option<crate::icon::IconName>,
    on_click: Option<ActionHandler>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: crate::icon::IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

pub struct MenuState {
    items: Vec<MenuItem>,
}

impl MenuState {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn build(self, cx: &mut App) -> Entity<MenuState> {
        cx.new(|_| MenuState { items: self.items })
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for MenuState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let style = MenuStyle::resolve(theme.token_set());
        let label_style = style.item_text;

        div()
            .min_w(style.min_width)
            .py(style.vertical_padding)
            .rounded(style.corner_radius)
            .bg(style.container_color)
            .shadow(Elevation::Level2.shadows(style.shadow_color))
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(self.items.iter().enumerate().map(|(ix, item)| {
                        let label = item.label.clone();
                        let icon = item.icon.clone();
                        let handler = item.on_click.clone();
                        div()
                            .id(("menu-item", ix))
                            .min_h(style.item_height)
                            .flex()
                            .items_center()
                            .gap(style.item_gap)
                            .px(style.item_horizontal_padding)
                            .cursor_pointer()
                            .hover(move |s| s.bg(style.item_hover_opacity))
                            .on_click(move |_event, window, cx| {
                                let host_entity = host(window, cx);
                                host_entity.update(cx, |h, cx| {
                                    h.menu = None;
                                    cx.notify();
                                });
                                if let Some(handler) = handler.clone() {
                                    handler(window, cx);
                                }
                            })
                            .when_some(icon, |el, icon| {
                                el.child(
                                    crate::icon::Icon::new(icon)
                                        .size(px(20.))
                                        .color(style.item_icon_color),
                                )
                            })
                            .child(
                                div()
                                    .text_size(label_style.size)
                                    .line_height(label_style.line_height)
                                    .text_color(style.item_text_color)
                                    .child(label),
                            )
                    })),
            )
    }
}

pub use appearance::{MenuStyle, SnackbarStyle, TooltipStyle};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};

    #[derive(Clone, Copy, Debug)]
    pub struct SnackbarStyle {
        pub container_color: Hsla,

        pub text_color: Hsla,

        pub action_color: Hsla,

        pub min_height: Pixels,

        pub padding: (Pixels, Pixels),

        pub bottom_offset: Pixels,

        pub action_gap: Pixels,

        pub corner_radius: Pixels,

        pub width: Pixels,

        pub shadow_color: Hsla,

        pub text: crate::theme::TypeStyle,

        pub action: crate::theme::TypeStyle,
    }
    impl SnackbarStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            let tokens_sb = &tokens.component.snackbar;
            Self {
                container_color: colors.inverse_surface,
                text_color: colors.inverse_on_surface,
                action_color: colors.inverse_primary,
                min_height: px(tokens_sb.min_height),
                padding: (
                    px(tokens_sb.horizontal_padding),
                    px(tokens_sb.vertical_padding),
                ),
                bottom_offset: px(tokens_sb.bottom_offset),
                action_gap: px(tokens_sb.action_gap),
                corner_radius: tokens.shapes.extra_small,
                width: px(440.),
                shadow_color: colors.shadow,
                text: tokens.typography.body_medium,
                action: tokens.typography.label_large,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct MenuStyle {
        pub container_color: Hsla,

        pub item_hover_opacity: Hsla,

        pub item_text_color: Hsla,

        pub item_icon_color: Hsla,

        pub item_height: Pixels,

        pub vertical_padding: Pixels,

        pub item_horizontal_padding: Pixels,

        pub item_gap: Pixels,

        pub min_width: Pixels,

        pub corner_radius: Pixels,

        pub anchor_gap: Pixels,

        pub shadow_color: Hsla,

        pub item_text: crate::theme::TypeStyle,
    }
    impl MenuStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            let menu = &tokens.component.menu;
            Self {
                container_color: colors.surface_container,
                item_hover_opacity: colors.on_surface.opacity(0.08),
                item_text_color: colors.on_surface,
                item_icon_color: colors.on_surface_variant,
                item_height: px(menu.item_height),
                vertical_padding: px(menu.vertical_padding),
                item_horizontal_padding: px(menu.item_horizontal_padding),
                item_gap: px(12.),
                min_width: px(180.),
                corner_radius: px(menu.corner_radius),
                anchor_gap: px(menu.anchor_gap),
                shadow_color: colors.shadow,
                item_text: tokens.typography.label_large,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct TooltipStyle {
        pub container_color: Hsla,

        pub text_color: Hsla,

        pub height: Pixels,

        pub horizontal_padding: Pixels,

        pub anchor_gap: Pixels,

        pub corner_radius: Pixels,

        pub text: crate::theme::TypeStyle,
    }
    impl TooltipStyle {
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            let tooltip = &tokens.component.tooltip;
            Self {
                container_color: colors.inverse_surface,
                text_color: colors.inverse_on_surface,
                height: px(tooltip.height),
                horizontal_padding: px(tooltip.horizontal_padding),
                anchor_gap: px(6.),
                corner_radius: tokens.shapes.extra_small,
                text: tokens.typography.body_small,
            }
        }
    }
}
