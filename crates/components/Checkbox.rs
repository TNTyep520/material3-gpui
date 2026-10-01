//! MD3 Checkbox（对应 material-web 的 `md-checkbox`）。
//!
//! 规格：复选框 18×18dp、圆角 2dp、边框 2dp；40dp 圆形触摸目标 + 状态层。
//!
//! 交互动画移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3CheckBoxSkin`（Apache-2.0，© 2026 Glavo）：勾选填充与勾图标
//! 由弹簧（defaultEffects）驱动；选择控件只有状态层、无涟漪。
//!
//! ```ignore
//! Checkbox::new("agree")
//!     .checked(true)
//!     .on_change(|checked, _, _| {})
//!     .build(cx)   // -> Entity<CheckboxState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

/// AndroidX ToggleableState 对应的三态复选框值。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleableState {
    /// 未选中。
    #[default]
    Off,
    /// 已选中。
    On,
    /// 部分选中。
    Indeterminate,
}

/// MD3 复选框构建器（`.build(cx)` 产出 [`CheckboxState`]）。
pub struct Checkbox {
    id: ElementId,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    error: bool,
    on_change: Option<ChangeHandler>,
}

/// 复选框的有状态部分。
pub struct CheckboxState {
    id: ElementId,
    checked: bool,
    indeterminate: bool,
    disabled: bool,
    error: bool,
    on_change: Option<ChangeHandler>,
    /// 0 = 未勾选，1 = 已勾选。
    progress: Animatable,
    surface: InteractiveSurface,
}

impl Checkbox {
    /// 创建复选框构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            indeterminate: false,
            disabled: false,
            error: false,
            on_change: None,
        }
    }

    /// 初始勾选态。
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self.indeterminate = false;
        self
    }

    /// 设置三态值；部分选中时显示横线。
    pub fn toggleable_state(mut self, state: ToggleableState) -> Self {
        self.checked = state == ToggleableState::On;
        self.indeterminate = state == ToggleableState::Indeterminate;
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置 AndroidX 对应的 enabled 状态。
    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    /// 错误状态（使用 error 配色）。
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// 勾选状态变化回调，参数为新的 checked 值。
    pub fn on_change(mut self, handler: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 设置勾选状态变化回调；传递新的 checked 值。
    pub fn on_checked_change(
        self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change(handler)
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<CheckboxState> {
        let checked = self.checked;
        cx.new(|_| CheckboxState {
            id: self.id,
            checked,
            indeterminate: self.indeterminate,
            disabled: self.disabled,
            error: self.error,
            on_change: self.on_change,
            progress: Animatable::new(
                if checked || self.indeterminate {
                    1.0
                } else {
                    0.0
                },
                1.0e-3,
            ),
            surface: InteractiveSurface::new(),
        })
    }
}

/// AndroidX TriStateCheckbox 对应的三态复选框构建器。
pub struct TriStateCheckbox(Checkbox);

impl TriStateCheckbox {
    /// 创建初始未选中的三态复选框。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(Checkbox::new(id))
    }

    /// 设置 Off、On 或 Indeterminate 状态。
    pub fn state(mut self, state: ToggleableState) -> Self {
        self.0 = self.0.toggleable_state(state);
        self
    }

    /// 设置启用状态。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }

    /// 设置点击回调；调用者可以据此控制下一个三态值。
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.0 = self.0.on_change(move |_, window, cx| handler(window, cx));
        self
    }

    /// 构建可渲染的三态复选框实体。
    pub fn build(self, cx: &mut App) -> Entity<CheckboxState> {
        self.0.build(cx)
    }
}

impl CheckboxState {
    /// 当前勾选态。
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// 返回当前三态值。
    pub fn toggleable_state(&self) -> ToggleableState {
        if self.indeterminate {
            ToggleableState::Indeterminate
        } else if self.checked {
            ToggleableState::On
        } else {
            ToggleableState::Off
        }
    }

    /// 设置勾选态（带动画）。
    pub fn set_checked(&mut self, checked: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.checked == checked && !self.indeterminate {
            return;
        }
        self.checked = checked;
        self.indeterminate = false;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        self.progress
            .animate_to(if checked { 1.0 } else { 0.0 }, &spec, Instant::now());
        if self.progress.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for CheckboxState {
    fn step(&mut self, now: Instant) -> bool {
        let running = self.progress.tick(now);
        let surface_running = self.surface.step(now);
        running || surface_running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for CheckboxState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.progress.is_running() || self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let p = self.progress.value() as f32;
        let style = CheckboxStyle::resolve(theme.token_set(), self.error, self.disabled);

        let accent = if self.error {
            colors.error
        } else {
            colors.primary
        };
        let on_accent = if self.error {
            colors.on_error
        } else {
            colors.on_primary
        };
        let outline = if self.error {
            colors.error
        } else {
            colors.on_surface_variant
        };

        // 状态层颜色（40dp 圆形触摸目标）
        let layer = if self.checked || self.indeterminate {
            accent
        } else {
            colors.on_surface
        };

        // 勾选填充/边框/图标随进度插值
        let (box_bg, box_border, mark_color) = if disabled {
            if self.checked || self.indeterminate {
                (
                    Some(colors.disabled_content(&state_layer)),
                    None,
                    Some(colors.surface),
                )
            } else {
                (None, Some(colors.disabled_content(&state_layer)), None)
            }
        } else {
            let bg = lerp_color(gpui::Hsla::transparent_black(), accent, p);
            let border = lerp_color(outline, accent, p);
            let mark = lerp_color(gpui::Hsla::transparent_black(), on_accent, p);
            (Some(bg), Some(border), Some(mark))
        };

        let entity = cx.entity();
        let base = div()
            .id(self.id.clone())
            .size(style.touch_target)
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
            let toggle_entity = entity;
            base.on_click(move |_event, window, cx| {
                toggle_entity.update(cx, |state, cx| {
                    let next = !state.checked;
                    state.set_checked(next, window, cx);
                    if let Some(handler) = state.on_change.clone() {
                        handler(next, window, cx);
                    }
                });
            })
        };

        base.child(
            div()
                .size(style.box_size)
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded(style.corner_radius)
                .when_some(box_bg, |el, bg| el.bg(bg))
                .when_some(box_border, |el, color| {
                    el.border(style.border_width).border_color(color)
                })
                .when_some(mark_color.filter(|_| p > 0.0), |el, color| {
                    el.child(
                        Icon::new(if self.indeterminate {
                            IconName::Remove
                        } else {
                            IconName::Check
                        })
                        .size(style.mark_size)
                        .color(color.opacity(p)),
                    )
                }),
        )
    }
}

pub use appearance::CheckboxStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 Checkbox 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct CheckboxStyle {
        /// 勾选填充色（error 态为 error 色）。
        pub accent: Hsla,
        /// 勾选图标准色。
        pub on_accent: Hsla,
        /// 未选中边框色。
        pub outline: Hsla,
        /// 方框边长。
        pub box_size: Pixels,
        /// 方框圆角。
        pub corner_radius: Pixels,
        /// 边框宽度。
        pub border_width: Pixels,
        /// 勾图标尺寸。
        pub mark_size: Pixels,
        /// 触摸目标边长。
        pub touch_target: Pixels,
        /// 状态层基色。
        pub state_layer_color: Hsla,
        /// 按压档状态层不透明度。
        pub state_layer_opacity: f32,
        /// 禁用态内容不透明度对应的颜色。
        pub disabled_content: Hsla,
    }
    impl CheckboxStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet, error: bool, disabled: bool) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let (accent, on_accent, outline) = if error {
                (colors.error, colors.on_error, colors.error)
            } else {
                (colors.primary, colors.on_primary, colors.on_surface_variant)
            };
            Self {
                accent,
                on_accent,
                outline,
                box_size: px(18.),
                corner_radius: px(2.),
                border_width: px(2.),
                mark_size: px(16.),
                touch_target: px(40.),
                state_layer_color: accent,
                state_layer_opacity: state.pressed,
                disabled_content: if disabled {
                    colors.disabled_content(state)
                } else {
                    colors.on_surface
                },
            }
        }
    }
}
