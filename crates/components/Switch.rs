//! MD3 Switch(对齐 [m3fx](https://github.com/Glavo/m3fx) 的 `M3Switch` /
//! `M3SwitchSkin`,Apache-2.0,© 2026 Glavo)。
//!
//! 规格:轨道 52×32dp 胶囊、触摸目标 52×48dp;拇指 16dp(未选)/
//! 24dp(选中或带图标)/ 28dp(按住,fastSpatial 弹簧放大);图标 16dp。
//!
//! 交互(对齐 M3SwitchSkin 状态机):
//! - 点击切换;按住后水平拖动 ≥4dp 进入拖动,拇指直接跟随指针,
//!   释放按位置 ≥ 0.5 提交,状态真正变化才触发一次 on_change
//!   (拖动提交不看释放位置;指针拖出控件 disarm、拖回 re-arm);
//! - 按住即放大到 28dp,松开弹回;hover 时拇指变色
//!   (选中 → primary_container,未选 → on_surface_variant);
//! - 按压指示:跟随拇指中心的 40dp 状态圆(hover)+ 无界涟漪(按压)。
//!
//! API 只提供两种形态:`Switch::new(id)`(无图标)与
//! `Switch::new(id).with_check_icon(true)`(选中对勾 / 未选关闭图标)。
//!
//! ```ignore
//! Switch::new("wifi")
//!     .checked(true)
//!     .with_check_icon(true)
//!     .on_change(|checked, _, _| println!("wifi: {checked}"))
//!     .build(cx)   // -> Entity<SwitchState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, DispatchPhase, ElementId, Entity, Hsla, InteractiveElement as _,
    IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _,
    Pixels, Point, Render, StatefulInteractiveElement as _, Styled, Window, canvas, div,
    prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

// 几何常量(dp,对齐 M3Switch 的 CSS 令牌)。
/// 进入拖动所需的水平位移阈值(m3fx `DRAG_THRESHOLD`)。
const DRAG_THRESHOLD: f32 = 4.0;
/// 触摸目标高度(轨道垂直居中其中)。
const TOUCH_TARGET_HEIGHT: f32 = 48.0;
/// 按住时拇指直径。
const PRESSED_HANDLE_SIZE: f32 = 28.0;
/// 状态圆直径(中心跟随拇指)。
const STATE_LAYER_SIZE: f32 = 40.0;
/// hover 状态圆不透明度(按压指示由涟漪承担)。
const STATE_LAYER_OPACITY: f32 = 0.08;

/// MD3 开关构建器(`.build(cx)` 产出 [`SwitchState`])。
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    check_icon: bool,
    on_change: Option<ChangeHandler>,
}

/// 开关的有状态部分。
pub struct SwitchState {
    id: ElementId,
    checked: bool,
    disabled: bool,
    check_icon: bool,
    on_change: Option<ChangeHandler>,
    /// 拇指位置 0..1(0=未选端,1=选中端;拖动中直接跟随指针)。
    progress: Animatable,
    /// 按压过渡(0=常态,1=拇指放大到 28dp;fastSpatial 弹簧进出)。
    press_progress: Animatable,
    /// 上一帧的按压状态(用于检测按压沿并驱动放大弹簧)。
    pressed_observed: bool,
    /// 主键已按下(拖拽会话进行中;指针拖出控件不中断)。
    pressed: bool,
    /// 已越过拖动阈值,拇指进入跟随模式。
    dragging: bool,
    /// 按下时的指针 X(判定拖动阈值)。
    drag_start_x: Pixels,
    /// 按下时(指针X − 拇指中心X)的抓取偏移,拖动中保持不变。
    grab_offset: Pixels,
    surface: InteractiveSurface,
}

impl Switch {
    /// 创建开关构建器(默认无图标)。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            check_icon: false,
            on_change: None,
        }
    }

    /// 初始选中态。
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 带图标变体:选中态拇指内显示对勾、未选中态显示关闭图标
    /// (16dp,对齐 m3fx 双图标模式)。
    pub fn with_check_icon(mut self, check_icon: bool) -> Self {
        self.check_icon = check_icon;
        self
    }

    /// 设置切换回调(参数为新选中态;仅在状态真正变化时触发)。
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<SwitchState> {
        let checked = self.checked;
        let mut surface = InteractiveSurface::new();
        // m3fx:无界涟漪,最大半径 = stateLayerSize / 2 = 20dp
        surface.set_ripple_max_radius(Some(px(20.)));
        cx.new(|_| SwitchState {
            id: self.id,
            checked,
            disabled: self.disabled,
            check_icon: self.check_icon,
            on_change: self.on_change,
            progress: Animatable::new(if checked { 1.0 } else { 0.0 }, 1.0e-3),
            press_progress: Animatable::new(0.0, 1.0e-3),
            pressed_observed: false,
            pressed: false,
            dragging: false,
            drag_start_x: px(0.),
            grab_offset: px(0.),
            surface,
        })
    }
}

impl SwitchState {
    /// 当前选中态。
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// 替换切换回调(用于构造后接线)。
    pub fn set_on_change(&mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Rc::new(handler));
    }

    /// 直接设置选中态(带动画;程序化设置不触发 on_change)。
    pub fn set_checked(&mut self, checked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.checked == checked {
            return;
        }
        self.checked = checked;
        self.animate_thumb_to(if checked { 1.0 } else { 0.0 }, window, cx);
    }

    /// 以 fastSpatial 弹簧把拇指动画到目标位置(m3fx `animateThumbPosition`)。
    fn animate_thumb_to(&mut self, target: f64, window: &mut Window, cx: &mut Context<Self>) {
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.progress.animate_to(target, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    /// 提交选中态:拇指弹簧回端点;状态真正变化才触发一次 on_change。
    fn apply_selection(&mut self, select: bool, window: &mut Window, cx: &mut Context<Self>) {
        let changed = select != self.checked;
        self.checked = select;
        self.animate_thumb_to(if select { 1.0 } else { 0.0 }, window, cx);
        if changed && let Some(handler) = self.on_change.clone() {
            handler(select, window, cx);
        }
    }

    /// 主键按下:arm(拇指放大)+ 在按压点播放涟漪,记录拖拽会话参数。
    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        let now = Instant::now();
        let motion = *cx.theme().motion();
        self.surface.on_press(event.position, &motion, now);

        let bounds = self.surface.bounds.get();
        let local_x = event.position.x - bounds.origin.x;
        let center_x = px(TRACK_HEIGHT_LOCAL)
            + px(TRACK_WIDTH_LOCAL - TRACK_HEIGHT_LOCAL) * self.progress.value() as f32;
        self.pressed = true;
        self.dragging = false;
        self.drag_start_x = event.position.x;
        self.grab_offset = local_x - center_x;
        cx.notify();
    }

    /// 拖动跟随(m3fx:越过阈值后拇指直接跟随指针,无弹簧滞后)。
    fn drag_move(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if !self.pressed || self.disabled {
            return;
        }
        if !self.dragging {
            if f32::from(position.x - self.drag_start_x).abs() < DRAG_THRESHOLD {
                return;
            }
            self.dragging = true;
            self.progress.stop();
        }
        let bounds = self.surface.bounds.get();
        // 指针拖回控件内时 re-arm(m3fx:离开 disarm、回来 re-arm);
        // 状态变化与位置变化合并为末尾一次 notify
        let re_armed = if bounds.contains(&position) && !self.surface.pressed {
            self.surface.pressed = true;
            true
        } else {
            false
        };
        let local_x = position.x - bounds.origin.x;
        let handle_center = local_x - self.grab_offset;
        let pos = f32::from(handle_center - px(TRACK_HEIGHT_LOCAL))
            / f32::from(px(TRACK_WIDTH_LOCAL - TRACK_HEIGHT_LOCAL));
        let next = f64::from(pos.clamp(0.0, 1.0));
        // 位置变化不足半像素(20dp 行程上约 0.025)时不重绘
        let moved = (next - self.progress.value()).abs() >= 0.025 / 20.0;
        if moved {
            self.progress.snap_to(next);
        }
        if moved || re_armed {
            cx.notify();
        }
    }

    /// 主键释放:提交拖动或处理点击(m3fx releasedInside 语义)。
    fn drag_end(&mut self, position: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.pressed {
            return;
        }
        self.pressed = false;
        let now = Instant::now();
        let motion = *cx.theme().motion();
        self.surface.on_release(&motion, now);

        let inside = self.surface.bounds.get().contains(&position);
        if self.dragging {
            self.dragging = false;
            // 拖动提交不看释放位置
            self.apply_selection(self.progress.value() >= 0.5, window, cx);
        } else if inside {
            // 纯点击:切换
            self.apply_selection(!self.checked, window, cx);
        } else {
            // 控件外释放且未进入拖动:弹回当前端点
            self.animate_thumb_to(if self.checked { 1.0 } else { 0.0 }, window, cx);
        }
        cx.notify();
    }

    /// div 层释放回调(兜底;与窗口级监听幂等)。
    fn on_mouse_up(&mut self, event: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.drag_end(event.position, window, cx);
    }
}

/// 拖拽数学用的轨道尺寸(dp;与 CSS 令牌默认值一致)。
const TRACK_WIDTH_LOCAL: f32 = 52.0;
const TRACK_HEIGHT_LOCAL: f32 = 32.0;

impl AnimatedComponent for SwitchState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        let press_running = self.press_progress.tick(now);
        running || surface_running || press_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for SwitchState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 动画循环:仍有动画时调度下一帧
        if self.progress.is_running()
            || self.surface.is_animating()
            || self.press_progress.is_running()
        {
            self.schedule_next(window, cx);
        }

        // 按压沿检测:按住即以 fastSpatial 弹簧放大到 28dp,松开弹回
        // (m3fx arm/disarm,无长按计时器)
        if self.surface.pressed != self.pressed_observed {
            self.pressed_observed = self.surface.pressed;
            let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
            let target = if self.surface.pressed { 1.0 } else { 0.0 };
            self.press_progress
                .animate_to(target, &spec, Instant::now());
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let disabled = self.disabled;
        let style = crate::styles::selection::SwitchStyle::resolve(theme.token_set(), disabled);
        let state_layer = *theme.state_layer();
        let p = self.progress.value() as f32;
        let press = self.press_progress.value() as f32;

        let track_bg = lerp_color(style.track_off, style.track_on, p);

        // 拇指颜色(对齐 m3fx 双层交叉淡化):未选层(outline,hover/armed
        // → on_surface_variant)与选中层(on_primary,→ primary_container)
        // 各自按交互进度渐变,两层再按拇指位置连续交叉——跨过中线无跳变
        let hover_t = if disabled {
            0.0
        } else {
            self.surface.hover_progress() as f32
        };
        let handle_off = lerp_color(style.handle_off, colors.on_surface_variant, hover_t);
        let handle_on = lerp_color(style.handle_on, colors.primary_container, hover_t);
        let handle_bg = lerp_color(handle_off, handle_on, p);

        // 几何(对齐 M3SwitchSkin.layoutThumb):拇指中心 X =
        // 轨道高/2 → 轨道宽 − 轨道高/2;直径 16/24(带图标恒 24),
        // 按住弹簧放大到 28
        let (track_w, track_h) = (style.track_size.0, style.track_size.1);
        let rest_size = if self.check_icon {
            style.thumb_on
        } else {
            style.thumb_off + (style.thumb_on - style.thumb_off) * p
        };
        let thumb_size = rest_size + (px(PRESSED_HANDLE_SIZE) - rest_size) * press;
        let center_x = track_h / 2. + (track_w - track_h) * p;
        let thumb_x = center_x - thumb_size / 2.;
        let thumb_y = (px(TOUCH_TARGET_HEIGHT) - thumb_size) / 2.;

        // 状态圆:40dp、中心跟随拇指(hover 指示;按压指示由涟漪承担)
        let state_color = lerp_color(colors.on_surface, colors.primary, p);
        let state_opacity = if disabled {
            0.0
        } else {
            hover_t * STATE_LAYER_OPACITY
        };

        // 描边随选中淡出
        let border_alpha = (1.0 - p * 2.0).clamp(0.0, 1.0);
        let border_color = style
            .border_off
            .map(|c| lerp_color(c, Hsla::transparent_black(), 1.0 - border_alpha));

        // 图标(带图标变体):选中对勾 / 未选关闭,位置过半切换并以
        // V 形透明度交叉淡化(对齐 m3fx `updateIconOpacity`);
        // 颜色对齐 selection.css:选中 primary、未选 surface-container-highest,
        // 禁用均 38%(选中禁用为 on-surface)
        let (displayed_icon, icon_color) = if p >= 0.5 {
            (
                IconName::Check,
                if disabled {
                    colors.disabled_content(&state_layer)
                } else {
                    colors.primary
                },
            )
        } else {
            (
                IconName::Close,
                if disabled {
                    colors
                        .surface_container_highest
                        .opacity(state_layer.disabled_content)
                } else {
                    colors.surface_container_highest
                },
            )
        };
        let icon_alpha = if p >= 0.5 {
            2.0 * p - 1.0
        } else {
            1.0 - 2.0 * p
        }
        .clamp(0.0, 1.0);

        // 外层:触摸目标 52×48,不裁剪(状态圆/涟漪允许越出轨道)
        let entity = cx.entity();
        let mut root = div()
            .id(self.id.clone())
            .relative()
            .w(px(TRACK_WIDTH_LOCAL))
            .h(px(TOUCH_TARGET_HEIGHT))
            .flex_none()
            .when(!disabled, |el| el.cursor_pointer());

        if !disabled {
            let motion = *theme.motion();
            let hover_entity = entity.clone();
            root = root
                // hover 变化只改状态并重绘;下一帧调度由 render 统一负责
                .on_hover(move |hovered, _window, cx| {
                    hover_entity.update(cx, |state, cx| {
                        state.surface.set_hovered(*hovered, &motion, Instant::now());
                        cx.notify();
                    });
                })
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up));
        }

        // 状态圆(最底层,中心跟随拇指);全透明时不生成元素
        if state_opacity > 0.0 {
            root = root.child(
                div()
                    .absolute()
                    .left(center_x - px(STATE_LAYER_SIZE / 2.))
                    .top((px(TOUCH_TARGET_HEIGHT) - px(STATE_LAYER_SIZE)) / 2.)
                    .size(px(STATE_LAYER_SIZE))
                    .rounded_full()
                    .bg(state_color.opacity(state_opacity)),
            );
        }

        // 轨道(垂直居中于触摸目标)+ 描边层
        let track = div()
            .absolute()
            .left(px(0.))
            .top((px(TOUCH_TARGET_HEIGHT) - track_h) / 2.)
            .w(track_w)
            .h(track_h)
            .rounded_full()
            .bg(track_bg)
            .when_some(border_color, |el, color| {
                el.child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded_full()
                        .border_2()
                        .border_color(color),
                )
            });
        root = root.child(track);

        // 涟漪(无界,20dp,按压点为心)+ 边界捕获
        if !disabled {
            let ripple_color = lerp_color(colors.on_surface, colors.primary, p);
            root = self
                .surface
                .overlay_unclipped(ripple_color, state_layer.pressed)
                .apply(root);
            root = root.child(self.surface.bounds.capture_element());
        }

        // 拖拽会话中注册窗口级监听(paint 阶段):指针元素外仍持续跟踪
        if !disabled && self.pressed {
            let entity = entity;
            root = root.child(
                canvas(
                    |_, _, _| {},
                    move |_, _, window, _| {
                        let move_entity = entity.clone();
                        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
                            if phase != DispatchPhase::Bubble {
                                return;
                            }
                            move_entity.update(cx, |state, cx| state.drag_move(event.position, cx));
                        });
                        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
                            if phase != DispatchPhase::Bubble || event.button != MouseButton::Left {
                                return;
                            }
                            entity
                                .update(cx, |state, cx| state.drag_end(event.position, window, cx));
                        });
                    },
                )
                .absolute()
                .inset_0(),
            );
        }

        // 拇指(最顶层,带图标槽)
        let mut thumb = div()
            .absolute()
            .left(thumb_x)
            .top(thumb_y)
            .size(thumb_size)
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(handle_bg);
        if self.check_icon {
            thumb = thumb.when(icon_alpha > 0.0, |el| {
                el.child(
                    Icon::new(displayed_icon)
                        .size(style.icon_size)
                        .color(icon_color.opacity(icon_alpha)),
                )
            });
        }
        root.child(thumb)
    }
}
