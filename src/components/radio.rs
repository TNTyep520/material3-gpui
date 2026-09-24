//! MD3 Radio（对应 material-web 的 `md-radio`）。
//!
//! 规格：外圈 20dp、边框 2dp、选中内点 10dp；40dp 圆形触摸目标 + 状态层。
//!
//! 交互动画移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3RadioButtonSkin`（Apache-2.0，© 2026 Glavo）：内点缩放由弹簧
//! （defaultEffects）驱动；选择控件只有状态层、无涟漪。
//!
//! ```ignore
//! RadioButton::new("plan-basic")
//!     .selected(plan == Plan::Basic)
//!     .on_select(|_, _| {})
//!     .build(cx)   // -> Entity<RadioState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::interaction::InteractiveSurface;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type SelectHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// MD3 单选按钮构建器（`.build(cx)` 产出 [`RadioState`]）。
pub struct RadioButton {
    id: ElementId,
    selected: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
}

/// 单选按钮的有状态部分。
pub struct RadioState {
    id: ElementId,
    selected: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
    /// 0 = 未选中，1 = 选中（内点缩放）。
    progress: Animatable,
    surface: InteractiveSurface,
}

impl RadioButton {
    /// 创建单选按钮构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            disabled: false,
            on_select: None,
        }
    }

    /// 初始选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 被点击（选中）时触发。
    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<RadioState> {
        let selected = self.selected;
        cx.new(|_| RadioState {
            id: self.id,
            selected,
            disabled: self.disabled,
            on_select: self.on_select,
            progress: Animatable::new(if selected { 1.0 } else { 0.0 }, 1.0e-3),
            surface: InteractiveSurface::new(),
        })
    }
}

impl RadioState {
    /// 当前选中态。
    pub fn selected(&self) -> bool {
        self.selected
    }

    /// 设置选中态（带动画）。
    pub fn set_selected(&mut self, selected: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected == selected {
            return;
        }
        self.selected = selected;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        self.progress
            .animate_to(if selected { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for RadioState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        running || surface_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for RadioState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() || self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let p = self.progress.value() as f32;

        let ring_color = if disabled {
            colors.disabled_content(&state_layer)
        } else if self.selected {
            colors.primary
        } else {
            colors.on_surface_variant
        };
        let dot_color = if disabled {
            colors.disabled_content(&state_layer)
        } else {
            colors.primary
        };
        let layer = if self.selected {
            colors.primary
        } else {
            colors.on_surface
        };

        let entity = cx.entity();
        let base = div()
            .id(self.id.clone())
            .size(px(40.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let base = if disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                layer,
                state_layer.pressed,
                gpui::px(999.),
            )
        };

        let base = if disabled {
            base
        } else {
            let select_entity = entity;
            base.on_click(move |_event, window, cx| {
                select_entity.update(cx, |state, cx| {
                    // 可取消勾选：点击已选中的项切换为未选中
                    state.set_selected(!state.selected, window, cx);
                    if let Some(handler) = state.on_select.clone() {
                        handler(window, cx);
                    }
                });
            })
        };

        base.child(
            div()
                .size(px(20.))
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded_full()
                .border_2()
                .border_color(if disabled {
                    ring_color
                } else {
                    lerp_color(colors.on_surface_variant, colors.primary, p)
                })
                .when(p > 0.0, |el| {
                    el.child(
                        div()
                            .size(px(10. * p))
                            .rounded_full()
                            .bg(dot_color.opacity(p)),
                    )
                }),
        )
    }
}
