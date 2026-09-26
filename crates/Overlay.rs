//! 窗口级弹层系统：Snackbar / Menu / Tooltip。
//!
//! 架构对应 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3OverlayPane` / `M3Snackbar` / `M3Menu` / `M3Tooltip` 及其
//! presenter（Apache-2.0，© 2026 Glavo）。gpui 没有 Scene 级 popup，
//! 本模块用"根容器挂载 + window 坐标绝对定位"实现等效机制：
//!
//! 1. 应用在窗口根视图挂载 [`host`] 返回的 OverlayHost 实体：
//!    `div().child(content).child(overlay::host(window, cx))`
//! 2. 任意事件处理器调用 [`show_snackbar`] / [`show_menu`] /
//!    [`show_tooltip`]，内容以 window 坐标绝对定位渲染在最上层。
//!
//! Snackbar 进入/退出使用 defaultSpatial 弹簧；Menu/Tooltip 首期为
//! 即时显隐。

use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyWindowHandle, App, AppContext as _, Bounds, Context, Entity, Global,
    InteractiveElement as _, IntoElement, ParentElement as _, Pixels, Render, SharedString,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px,
    relative,
};

use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::{ActiveTheme, Elevation};

/// 动作回调类型。
type ActionHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// 全局弹层注册表：每个窗口一个 OverlayHost。
#[derive(Default)]
pub struct OverlayRegistry {
    hosts: HashMap<AnyWindowHandle, Entity<OverlayHostState>>,
}

impl Global for OverlayRegistry {}

/// 获取（或首次创建）当前窗口的 OverlayHost 实体。
///
/// 应用须把返回的实体挂到窗口根视图（作为最后一个子元素），
/// 否则弹层内容不会显示。
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

/// 显示 Snackbar。
///
/// `duration` 为自动消失时长；传 `None` 使用默认（5 秒）。
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

    // 自动消失定时器
    let timer_host = host_entity.clone();
    cx.spawn(async move |cx| {
        cx.background_executor().timer(duration).await;
        timer_host.update(cx, |host, cx| {
            host.dismiss_snack_top(cx);
        })
    })
    .detach_and_log_err(cx);
}

/// 在锚点下方显示菜单。
pub fn show_menu(window: &Window, cx: &mut App, menu: Entity<MenuState>, anchor: Bounds<Pixels>) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.menu = Some((menu, anchor));
        cx.notify();
    });
}

/// 关闭当前菜单。
pub fn close_menu(window: &Window, cx: &mut App) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.menu = None;
        cx.notify();
    });
}

/// 在锚点下方显示工具提示（每个窗口同时只有一个）。
pub fn show_tooltip(
    window: &Window,
    cx: &mut App,
    text: impl Into<SharedString>,
    anchor: Bounds<Pixels>,
) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.tooltip = Some(Tooltip {
            text: text.into(),
            anchor,
        });
        cx.notify();
    });
}

/// 关闭工具提示。
pub fn close_tooltip(window: &Window, cx: &mut App) {
    let host_entity = host(window, cx);
    host_entity.update(cx, |host, cx| {
        host.tooltip = None;
        cx.notify();
    });
}

/// OverlayHost 的有状态部分：持有当前窗口全部弹层。
#[derive(Default)]
pub struct OverlayHostState {
    next_id: u64,
    snacks: Vec<SnackState>,
    menu: Option<(Entity<MenuState>, Bounds<Pixels>)>,
    tooltip: Option<Tooltip>,
    driver: AnimationDriver,
}

impl OverlayHostState {
    fn dismiss_snack_top(&mut self, cx: &mut Context<Self>) {
        if let Some(snack) = self.snacks.last_mut() {
            snack.begin_exit(cx);
        }
        // 移除在 render 的 step 中完成（等待退出动画）
        cx.notify();
    }
}

/// 工具提示数据。
#[derive(Clone)]
struct Tooltip {
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
        // 移除已完成退出动画的 snack
        self.snacks.retain(|s| !s.removed);
        animating
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for OverlayHostState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // render 只读状态:动画推进统一由帧回调里的 step 负责
        // （与其他组件一致，避免进度依赖 render 频率）
        self.snacks.retain(|snack| !snack.removed);
        if self.snacks.iter().any(SnackState::is_animating) {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let tokens = theme.component().snackbar;

        // Snackbar：底部居中（一次显示一条，后进优先）
        let snack_el = self.snacks.last().map(|snack| {
            let p = snack.progress.value() as f32;
            let bottom = px(tokens.bottom_offset) - px(64.) * (1.0 - p);
            let bg = lerp_color(gpui::Hsla::transparent_black(), colors.inverse_surface, p);
            let fg = colors.inverse_on_surface.opacity(p);
            let action_label = snack.action_label.clone();
            let action_handler = snack.on_action.clone();
            let snack_id = snack.id;
            let action_click = action_handler.clone();
            div()
                .id(("snack", snack_id))
                .absolute()
                .bottom(bottom)
                .left(relative(0.5))
                .ml(px(-220.))
                .w(px(440.))
                .min_h(px(tokens.min_height))
                .flex()
                .flex_none()
                .items_center()
                .gap(px(tokens.action_gap))
                .px(px(tokens.horizontal_padding))
                .py(px(tokens.vertical_padding))
                .rounded(theme.shapes().extra_small)
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
                            .text_color(colors.primary.opacity(p))
                            .hover(move |s| s.bg(colors.primary.opacity(0.08 * p)))
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

        // Menu：锚点下方
        let menu_el = self.menu.clone().map(|(menu_entity, anchor)| {
            div()
                .absolute()
                .left(anchor.origin.x)
                .top(anchor.origin.y + anchor.size.height + px(4.))
                .child(menu_entity)
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

        // Tooltip：锚点下方
        let tooltip_el = self.tooltip.clone().map(|tip| {
            div()
                .absolute()
                .left(tip.anchor.origin.x + tip.anchor.size.width / 2.0)
                .top(tip.anchor.origin.y + tip.anchor.size.height + px(6.))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .h(px(theme.component().tooltip.height))
                        .px(px(theme.component().tooltip.horizontal_padding))
                        .rounded(theme.shapes().extra_small)
                        .bg(colors.inverse_surface)
                        .text_size(theme.typography().body_small.size)
                        .text_color(colors.inverse_on_surface)
                        .child(tip.text),
                )
        });

        // 根容器以绝对定位铺满窗口（无背景、自身不拦截点击），
        // 弹层子元素的百分比/绝对定位以整个窗口为基准
        div()
            .absolute()
            .inset_0()
            .flex_none()
            .when_some(snack_el, |el, s| el.child(s))
            .when_some(menu_el, |el, m| el.child(m))
            .when_some(tooltip_el, |el, t| el.child(t))
    }
}

/// Snackbar 构建器。
pub struct Snackbar {
    message: SharedString,
    action_label: Option<SharedString>,
    on_action: Option<ActionHandler>,
}

impl Snackbar {
    /// 创建 Snackbar（消息必填）。
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            action_label: None,
            on_action: None,
        }
    }

    /// 动作按钮文本（如 "UNDO"）。
    pub fn action(mut self, label: impl Into<SharedString>) -> Self {
        self.action_label = Some(label.into());
        self
    }

    /// 动作回调。
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
    /// 0 = 隐藏，1 = 完全显示；进入/退出共用。
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

/// 菜单项。
pub struct MenuItem {
    label: SharedString,
    icon: Option<crate::icon::IconName>,
    on_click: Option<ActionHandler>,
}

impl MenuItem {
    /// 创建菜单项。
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: None,
        }
    }

    /// 设置图标。
    pub fn icon(mut self, icon: crate::icon::IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 设置点击回调。
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

/// MD3 菜单（经 [`show_menu`] 显示）。
pub struct MenuState {
    items: Vec<MenuItem>,
}

impl MenuState {
    /// 创建菜单。
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 追加菜单项。
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// 构建菜单实体（随后经 [`show_menu`] 显示）。
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
        let colors = theme.colors();
        let tokens = theme.component().menu;
        let label_style = theme.typography().label_large;

        div()
            .min_w(px(180.))
            .py(px(tokens.vertical_padding))
            .rounded(px(tokens.corner_radius))
            .bg(colors.surface_container)
            .shadow(Elevation::Level2.shadows(colors.shadow))
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(self.items.iter().enumerate().map(|(ix, item)| {
                        let label = item.label.clone();
                        let icon = item.icon;
                        let handler = item.on_click.clone();
                        div()
                            .id(("menu-item", ix))
                            .h(px(tokens.item_height))
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .px(px(tokens.item_horizontal_padding))
                            .cursor_pointer()
                            .hover(move |s| s.bg(colors.on_surface.opacity(0.08)))
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
                                        .color(colors.on_surface_variant),
                                )
                            })
                            .child(
                                div()
                                    .text_size(label_style.size)
                                    .line_height(label_style.line_height)
                                    .text_color(colors.on_surface)
                                    .child(label),
                            )
                    })),
            )
    }
}
