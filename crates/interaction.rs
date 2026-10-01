//! 交互表面核心：状态层淡入淡出与涟漪。
//!
//! 行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3StateLayer` 与 `M3LabeledButtonSkinBase` 的交互生命周期
//! （Apache-2.0，© 2026 Glavo）：
//!
//! - 状态层（hover / pressed）以弹簧动画淡入淡出
//!   （进入用 fastEffects，退出用 defaultEffects）；
//! - 按压时在指针处生成涟漪（半径扩张、释放后淡出），
//!   指针移出组件时取消按压态并淡出涟漪。

use std::cell::RefCell;
use std::f32::consts::{FRAC_PI_2, PI};
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Canvas, Corners, Div, Entity, Hsla, InteractiveElement as _,
    IntoElement as _, MouseButton, ParentElement as _, PathBuilder, Pixels, Point, Stateful,
    StatefulInteractiveElement as _, Styled, canvas, div, point, px,
};

use crate::motion::{Animatable, AnimationDriver, MotionRole, MotionScheme, lerp_color};

/// hover 状态层在按压档位上的进度系数（0.08 / 0.10 = 0.8）。
const HOVER_PROGRESS: f64 = 0.8;

/// 组件边界的共享句柄：paint 阶段由捕获元素写入，
/// 事件处理器在下一帧之前读取。
#[derive(Clone, Debug, Default)]
pub struct BoundsHandle(Rc<RefCell<Bounds<Pixels>>>);

impl BoundsHandle {
    /// 创建边界句柄。
    pub fn new() -> Self {
        Self::default()
    }

    /// 读取最近一次捕获的边界（未捕获时为零）。
    pub fn get(&self) -> Bounds<Pixels> {
        *self.0.borrow()
    }

    /// 构造写入本句柄的 canvas 子元素（绝对定位铺满父容器）。
    pub fn capture_element(&self) -> Canvas<()> {
        let handle = self.0.clone();
        canvas(
            move |bounds, _window, _cx| {
                *handle.borrow_mut() = bounds;
            },
            |_, _, _, _| {},
        )
        .absolute()
        .inset_0()
    }
}

/// 状态层 + 涟漪的共享状态。组件把它作为字段内嵌：
/// 事件处理器调用 [`InteractiveSurface::on_press`] 等方法，
/// render 中通过 [`InteractiveSurface::overlay`] 生成覆盖层。
#[derive(Clone, Debug)]
pub struct InteractiveSurface {
    /// 组件边界句柄（paint 阶段捕获）。
    pub bounds: BoundsHandle,
    /// 是否处于 hover。
    pub hovered: bool,
    /// 是否处于按压。
    pub pressed: bool,
    /// 是否启用涟漪（m3fx 中 Switch/Checkbox/Radio 等选择控件
    /// 只有状态层、没有涟漪；默认启用）。
    pub ripple_enabled: bool,
    /// 涟漪最大半径；`None` 时自动取按压点到组件最远角的距离。
    /// Compose M3 的开关等控件用固定半径（StateLayerSize/2 = 20dp）。
    ripple_max_radius: Option<Pixels>,
    state_layer_opacity: Animatable,
    ripple: Option<Ripple>,
    driver: AnimationDriver,
}

#[derive(Clone, Copy, Debug)]
struct Ripple {
    /// 涟漪圆心（组件本地坐标）。
    origin: Point<Pixels>,
    /// 涟漪半径（px）。
    radius: Animatable,
    /// 涟漪不透明度系数（1.0 → 0.0，释放后衰减）。
    fade: Animatable,
}

impl Default for InteractiveSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveSurface {
    /// 创建交互表面状态。
    pub fn new() -> Self {
        Self {
            bounds: BoundsHandle::new(),
            hovered: false,
            pressed: false,
            ripple_enabled: true,
            ripple_max_radius: None,
            state_layer_opacity: Animatable::new(0.0, 1.0e-3),
            ripple: None,
            driver: AnimationDriver::default(),
        }
    }

    /// 设置涟漪最大半径（`None` 恢复自动：按压点到最远角的距离）。
    pub fn set_ripple_max_radius(&mut self, max_radius: Option<Pixels>) {
        self.ripple_max_radius = max_radius;
    }

    /// hover/armed 过渡进度（0..1）。对齐 m3fx selection.css:hover、
    /// armed、pressed 共用同一配色档,故按压不归零。
    /// 供组件派生交互变色(如开关拇指的 hover 变色)。
    pub fn hover_progress(&self) -> f64 {
        f64::clamp(self.state_layer_opacity.value() / HOVER_PROGRESS, 0.0, 1.0)
    }

    /// 内嵌驱动器（供 [`crate::motion::AnimatedComponent::driver_mut`] 返回）。
    pub fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }

    /// 推进状态层与涟漪动画；返回是否仍在动画中。
    pub fn step(&mut self, now: Instant) -> bool {
        self.state_layer_opacity.tick(now);
        let mut ripple_done = false;
        if let Some(ripple) = &mut self.ripple {
            let radius_running = ripple.radius.tick(now);
            let fade_running = ripple.fade.tick(now);
            if !radius_running && !fade_running && ripple.fade.value() <= 0.0 {
                ripple_done = true;
            }
        }
        if ripple_done {
            self.ripple = None;
        }
        self.is_animating()
    }

    /// 是否仍在动画中（状态层或涟漪）。
    pub fn is_animating(&self) -> bool {
        self.state_layer_opacity.is_running()
            || self.ripple.as_ref().is_some_and(|r| {
                r.radius.is_running() || r.fade.is_running() || r.fade.value() > 0.0
            })
    }

    /// 指针进入/离开。离开时同时取消按压态（对齐 m3fx 的 disarm-on-exit）。
    pub fn set_hovered(&mut self, hovered: bool, motion: &MotionScheme, now: Instant) {
        self.hovered = hovered;
        if !hovered {
            self.pressed = false;
        }
        self.animate_state_layer_to(self.target_opacity(), motion, now);
    }

    /// 指针按下（`position` 为窗口坐标）；在按压点生成涟漪。
    pub fn on_press(&mut self, position: Point<Pixels>, motion: &MotionScheme, now: Instant) {
        self.pressed = true;
        self.state_layer_opacity
            .animate_to(1.0, motion.spec(MotionRole::FastEffects), now);

        if !self.ripple_enabled {
            return;
        }
        // 最大半径：显式上限优先（如 Compose M3 开关的 20dp），
        // 否则取圆心到组件最远角的距离
        let bounds = self.bounds.get();
        let local_x = f32::from(position.x - bounds.origin.x);
        let local_y = f32::from(position.y - bounds.origin.y);
        let width = f32::from(bounds.size.width);
        let height = f32::from(bounds.size.height);
        let dx2 = |x: f32, y: f32| f64::from(x * x + y * y);
        let auto_radius = f64::max(
            f64::max(dx2(local_x, local_y), dx2(width - local_x, local_y)),
            f64::max(
                dx2(local_x, height - local_y),
                dx2(width - local_x, height - local_y),
            ),
        )
        .sqrt();
        let max_radius = self.ripple_max_radius.map(f64::from).unwrap_or(auto_radius);
        let mut ripple = Ripple {
            origin: point(px(local_x), px(local_y)),
            radius: Animatable::new(0.0, 1.0e-2),
            fade: Animatable::new(1.0, 1.0e-2),
        };
        // 涟漪扩散对齐 m3fx:defaultSpatial(defaultEffects 只用于淡出)
        ripple
            .radius
            .animate_to(max_radius, motion.spec(MotionRole::DefaultSpatial), now);
        self.ripple = Some(ripple);
    }

    /// 指针在组件内释放。
    pub fn on_release(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    /// 指针在组件外释放：取消按压并淡出涟漪。
    pub fn on_cancel(&mut self, motion: &MotionScheme, now: Instant) {
        self.pressed = false;
        self.animate_state_layer_to(self.target_opacity(), motion, now);
        self.fade_ripple(motion, now);
    }

    fn fade_ripple(&mut self, motion: &MotionScheme, now: Instant) {
        if let Some(ripple) = &mut self.ripple {
            ripple
                .fade
                .animate_to(0.0, motion.spec(MotionRole::DefaultEffects), now);
        }
    }

    /// 状态层不透明度目标（相对按压档位的进度）。
    fn target_opacity(&self) -> f64 {
        if self.pressed {
            1.0
        } else if self.hovered {
            HOVER_PROGRESS
        } else {
            0.0
        }
    }

    fn animate_state_layer_to(&mut self, target: f64, motion: &MotionScheme, now: Instant) {
        let role = if target > self.state_layer_opacity.value() {
            MotionRole::FastEffects
        } else {
            MotionRole::DefaultEffects
        };
        self.state_layer_opacity
            .animate_to(target, motion.spec(role), now);
    }

    /// 生成状态层与涟漪的渲染产物。
    ///
    /// `base_color` 为状态层/涟漪基色（通常是组件的 content/on-container
    /// 色）；`pressed_opacity` 为满档（按压）状态层不透明度，取自主题
    /// state layer 令牌（如 0.10）；hover 按 8%/10% 比例映射；
    /// `corner_radius` 为容器圆角——状态层自带同值圆角，
    /// 不依赖容器裁剪（对齐 m3fx `M3StateLayer` 自带容器形状）。
    pub fn overlay(
        &self,
        base_color: Hsla,
        pressed_opacity: f32,
        corner_radius: Pixels,
    ) -> InteractiveOverlay {
        self.overlay_with_corners(base_color, pressed_opacity, Corners::all(corner_radius))
    }

    pub fn overlay_with_corners(
        &self,
        base_color: Hsla,
        pressed_opacity: f32,
        corner_radius: Corners<Pixels>,
    ) -> InteractiveOverlay {
        let layer_alpha = f32::clamp(
            self.state_layer_opacity.value() as f32 * pressed_opacity,
            0.0,
            1.0,
        );
        InteractiveOverlay {
            state_layer_color: if layer_alpha > 0.0 {
                Some(lerp_color(
                    Hsla::transparent_black(),
                    base_color,
                    layer_alpha,
                ))
            } else {
                None
            },
            ripple: self.ripple_elements(base_color, pressed_opacity, corner_radius, false),
            corner_radius,
        }
    }

    /// 生成 Compose M3 风格的覆盖层：无轨道面状态层，涟漪以按压点为
    /// 圆心、不裁剪到容器形状（对齐 Compose `Switch` 等以无界涟漪
    /// `bounded = false` 作指示的控件；半径上限由
    /// [`Self::set_ripple_max_radius`] 控制，默认 20dp）。
    pub fn overlay_unclipped(&self, base_color: Hsla, ripple_opacity: f32) -> InteractiveOverlay {
        InteractiveOverlay {
            state_layer_color: None,
            ripple: self.ripple_elements(base_color, ripple_opacity, Corners::all(px(0.)), true),
            corner_radius: Corners::all(px(0.)),
        }
    }

    pub(crate) fn set_ripple_origin(&mut self, origin: Point<Pixels>) {
        if let Some(ripple) = &mut self.ripple {
            ripple.origin = origin;
        }
    }

    /// 按当前涟漪状态构建涟漪元素（同时最多一个涟漪，故返回 Option）。
    fn ripple_elements(
        &self,
        base_color: Hsla,
        ripple_opacity: f32,
        corner_radius: Corners<Pixels>,
        unclipped: bool,
    ) -> Option<AnyElement> {
        let ripple = self.ripple.as_ref()?;
        if ripple.fade.value() <= 0.0 || ripple.radius.value() <= 0.0 {
            return None;
        }
        let radius = ripple.radius.value() as f32;
        let alpha = f32::clamp(ripple.fade.value() as f32 * ripple_opacity, 0.0, 1.0);
        let color = lerp_color(Hsla::transparent_black(), base_color, alpha);
        Some(ripple_element(
            ripple.origin,
            radius,
            color,
            corner_radius,
            unclipped,
        ))
    }
}

/// 状态层与涟漪的渲染产物。
pub struct InteractiveOverlay {
    /// 状态层颜色（透明时为 None）。
    state_layer_color: Option<Hsla>,
    /// 涟漪圆元素（同时最多一个）。
    ripple: Option<AnyElement>,
    /// 状态层圆角（与容器形状一致）。
    corner_radius: Corners<Pixels>,
}

impl InteractiveOverlay {
    /// 把覆盖层子元素挂到容器上。状态层自带与容器一致的圆角，
    /// 不依赖容器裁剪（对齐 m3fx `M3StateLayer` 自带容器形状）。
    pub fn apply(self, container: Stateful<Div>) -> Stateful<Div> {
        let mut container = container;
        if let Some(color) = self.state_layer_color {
            container = container.child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded_tl(self.corner_radius.top_left)
                    .rounded_tr(self.corner_radius.top_right)
                    .rounded_bl(self.corner_radius.bottom_left)
                    .rounded_br(self.corner_radius.bottom_right)
                    .bg(color),
            );
        }
        if let Some(ripple) = self.ripple {
            container = container.child(ripple);
        }
        container
    }
}

/// 单个涟漪的自绘元素：paint 阶段把涟漪圆与容器圆角矩形求交集后绘制。
///
/// gpui 的 `overflow_hidden` 裁剪掩码是纯矩形、不识别圆角，旧实现
/// （超出容器的大圆 div + 容器裁剪）会让涟漪填色出现在圆角外的角部；
/// 这里改为直接绘制交集多边形，涟漪从数学上不会越出容器形状。
/// `unclipped` 时直接绘制整圆（Compose 无界涟漪，允许越出容器）。
/// 涟漪圆与容器角部圆弧均以多边形离散近似，在涟漪的不透明度
/// （≤10%）与组件尺度下视觉与精确形状无差。
fn ripple_element(
    origin: Point<Pixels>,
    radius: f32,
    color: Hsla,
    corner_radius: Corners<Pixels>,
    unclipped: bool,
) -> AnyElement {
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            // 闭包内的 origin 为容器本地坐标，先换算到窗口坐标
            let center = bounds.origin + origin;
            let circle = circle_polygon(center, radius);
            // 无界模式或圆完全落在容器形状内时直接画整圆，
            // 跳过圆角多边形构造与裁剪（涟漪扩张前半段均属此情形）
            let points = if unclipped
                || (corner_radius == Corners::all(corner_radius.top_left)
                    && circle_inside_rounded_rect(bounds, corner_radius.top_left, center, radius))
            {
                circle
            } else {
                let clip = rounded_rect_polygon(bounds, corner_radius);
                let Some(points) = clip_convex(&circle, &clip) else {
                    return;
                };
                points
            };
            let mut builder = PathBuilder::fill();
            builder.add_polygon(&points, true);
            if let Ok(path) = builder.build() {
                window.paint_path(path, color);
            }
        },
    )
    .absolute()
    .inset_0()
    .into_any_element()
}

/// 判断半径为 `radius` 的圆是否完整落在容器圆角矩形内部。
///
/// 判据：圆心到四边的距离均 ≥ 半径，且圆心到四个圆角圆心的距离
/// ≤ 角半径 − 半径（后者保证不越出角部弧线）。
fn circle_inside_rounded_rect(
    bounds: Bounds<Pixels>,
    corner_radius: Pixels,
    center: Point<Pixels>,
    radius: f32,
) -> bool {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    let r = f32::from(corner_radius).clamp(0.0, w.min(h) / 2.0);
    let (x0, y0) = (f32::from(bounds.origin.x), f32::from(bounds.origin.y));
    let (x1, y1) = (x0 + w, y0 + h);
    let (cx, cy) = (f32::from(center.x), f32::from(center.y));

    // 先看四条边
    if cx - radius < x0 || cx + radius > x1 || cy - radius < y0 || cy + radius > y1 {
        return false;
    }
    // 再看四个角部弧线：仅当圆心落在角部方块内才需要判断
    let corner_ok = |ccx: f32, ccy: f32, outside_x: bool, outside_y: bool| {
        if !(outside_x && outside_y) {
            return true;
        }
        let (dx, dy) = (cx - ccx, cy - ccy);
        (dx * dx + dy * dy).sqrt() + radius <= r
    };
    corner_ok(x0 + r, y0 + r, cx < x0 + r, cy < y0 + r)
        && corner_ok(x1 - r, y0 + r, cx > x1 - r, cy < y0 + r)
        && corner_ok(x0 + r, y1 - r, cx < x0 + r, cy > y1 - r)
        && corner_ok(x1 - r, y1 - r, cx > x1 - r, cy > y1 - r)
}

/// 圆的正多边形近似顶点（顶点序与 [`rounded_rect_polygon`] 同为顺时针）。
fn circle_polygon(center: Point<Pixels>, radius: f32) -> Vec<Point<Pixels>> {
    const CIRCLE_SEGMENTS: usize = 32;
    (0..CIRCLE_SEGMENTS)
        .map(|i| {
            let angle = std::f32::consts::TAU * i as f32 / CIRCLE_SEGMENTS as f32;
            point(
                center.x + px(radius * angle.cos()),
                center.y + px(radius * angle.sin()),
            )
        })
        .collect()
}

/// 圆角矩形的凸多边形近似（顺时针，四角圆弧各 [`ARC_SEGMENTS`] 段）。
fn rounded_rect_polygon(
    bounds: Bounds<Pixels>,
    corner_radius: impl Into<Corners<Pixels>>,
) -> Vec<Point<Pixels>> {
    const ARC_SEGMENTS: usize = 6;
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    // 圆角钳制到短边一半（shapes.full = 999px 依赖此钳制得到胶囊形）
    let corners = corner_radius
        .into()
        .map(|radius| f32::from(*radius).clamp(0.0, w.min(h) / 2.0));
    let (x0, y0) = (f32::from(bounds.origin.x), f32::from(bounds.origin.y));
    let (x1, y1) = (x0 + w, y0 + h);
    let mut points = Vec::with_capacity(4 * (ARC_SEGMENTS + 1));
    // 从右上角起顺时针遍历，每角给出圆心与起始角、沿外弧扫过 90°
    for (center_x, center_y, radius, start) in [
        (
            x1 - corners.top_right,
            y0 + corners.top_right,
            corners.top_right,
            -FRAC_PI_2,
        ),
        (
            x1 - corners.bottom_right,
            y1 - corners.bottom_right,
            corners.bottom_right,
            0.0,
        ),
        (
            x0 + corners.bottom_left,
            y1 - corners.bottom_left,
            corners.bottom_left,
            FRAC_PI_2,
        ),
        (
            x0 + corners.top_left,
            y0 + corners.top_left,
            corners.top_left,
            PI,
        ),
    ] {
        if radius == 0.0 {
            points.push(point(px(center_x), px(center_y)));
            continue;
        }
        for step in 0..=ARC_SEGMENTS {
            let angle = start + FRAC_PI_2 * step as f32 / ARC_SEGMENTS as f32;
            points.push(point(
                px(center_x + radius * angle.cos()),
                px(center_y + radius * angle.sin()),
            ));
        }
    }
    points
}

/// 点 `p` 相对有向边 a→b 的叉积分量（两多边形均为顺时针，>0 即在内侧）。
fn edge_cross(a: Point<Pixels>, b: Point<Pixels>, p: Point<Pixels>) -> f32 {
    let (ux, uy) = (f32::from(b.x - a.x), f32::from(b.y - a.y));
    let (vx, vy) = (f32::from(p.x - a.x), f32::from(p.y - a.y));
    ux * vy - uy * vx
}

/// Sutherland–Hodgman 多边形裁剪：返回 `subject ∩ clip`。
/// `clip` 必须为凸多边形；交集退化（不足三个顶点）时返回 `None`。
///
/// 两个缓冲区轮换复用，整个裁剪过程只分配两次（每条裁剪边不再新建
/// 中间 `Vec`）。
fn clip_convex(subject: &[Point<Pixels>], clip: &[Point<Pixels>]) -> Option<Vec<Point<Pixels>>> {
    let mut output: Vec<Point<Pixels>> = subject.to_vec();
    let mut scratch: Vec<Point<Pixels>> = Vec::with_capacity(subject.len() + clip.len());
    for i in 0..clip.len() {
        if output.is_empty() {
            return None;
        }
        let a = clip[i];
        let b = clip[(i + 1) % clip.len()];
        scratch.clear();
        for j in 0..output.len() {
            let (p, q) = (output[j], output[(j + 1) % output.len()]);
            let (dp, dq) = (edge_cross(a, b, p), edge_cross(a, b, q));
            let (p_in, q_in) = (dp >= 0.0, dq >= 0.0);
            if p_in != q_in {
                // 线段 p→q 与裁剪边所在直线的交点（dp、dq 异号，t ∈ [0,1]）
                let t = dp / (dp - dq);
                scratch.push(point(p.x + (q.x - p.x) * t, p.y + (q.y - p.y) * t));
            }
            if q_in {
                scratch.push(q);
            }
        }
        std::mem::swap(&mut output, &mut scratch);
    }
    (output.len() >= 3).then_some(output)
}

/// 一次性完成交互接线:事件绑定、覆盖层(状态层/涟漪)挂载与边界捕获。
/// button/chip/fab/icon_button/checkbox/radio 六组件共用此入口;
/// 需要无界涟漪的 Switch 走 [`InteractiveSurface::overlay_unclipped`]。
#[allow(clippy::too_many_arguments)]
pub fn wire<T: 'static>(
    surface: &InteractiveSurface,
    el: Stateful<Div>,
    entity: &Entity<T>,
    motion: &MotionScheme,
    access: impl Fn(&mut T) -> &mut InteractiveSurface + Copy + 'static,
    state_layer_color: Hsla,
    pressed_opacity: f32,
    corner_radius: Pixels,
) -> Stateful<Div> {
    let el = wire_events(el, entity, motion, access);
    let el = surface
        .overlay(state_layer_color, pressed_opacity, corner_radius)
        .apply(el);
    el.child(surface.bounds.capture_element())
}

/// 把 hover / press / release / cancel 事件接到内嵌
/// [`InteractiveSurface`] 的组件元素上。
///
/// `entity` 为组件自身的 Entity 句柄，`motion` 来自主题
/// （`theme.motion()`）。事件回调只改状态并触发重绘。
pub fn wire_events<T: 'static>(
    el: Stateful<Div>,
    entity: &Entity<T>,
    motion: &MotionScheme,
    access: impl Fn(&mut T) -> &mut InteractiveSurface + 'static + Copy,
) -> Stateful<Div> {
    let hover_entity = entity.clone();
    let press_entity = entity.clone();
    let release_entity = entity.clone();
    let cancel_entity = entity.clone();
    let motion_hover = *motion;
    let motion_press = *motion;
    let motion_release = *motion;
    let motion_cancel = *motion;
    el.on_hover(move |hovered, _window, cx| {
        hover_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).set_hovered(*hovered, &motion_hover, now);
            cx.notify();
        });
    })
    .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
        press_entity.update(cx, |state, cx| {
            let now = Instant::now();
            let position = event.position;
            access(state).on_press(position, &motion_press, now);
            cx.notify();
        });
    })
    .on_mouse_up(MouseButton::Left, move |_event, _window, cx| {
        release_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_release(&motion_release, now);
            cx.notify();
        });
    })
    .on_mouse_up_out(MouseButton::Left, move |_event, _window, cx| {
        cancel_entity.update(cx, |state, cx| {
            let now = Instant::now();
            access(state).on_cancel(&motion_cancel, now);
            cx.notify();
        });
    })
}
