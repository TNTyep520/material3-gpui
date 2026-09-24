//! MD3 Button（共享按钮，对应 material-web 的 `md-*-button`）。
//!
//! 五种变体：Filled / Outlined / Text / Elevated / FilledTonal。
//! 视格与几何全部来自 [`ButtonStyle`](crate::styles::ButtonStyle)
//! （默认值由主题令牌推导，对齐 m3fx `styles/controls/button.css`）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3ButtonSkin` / `M3LabeledButtonSkinBase`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! use md3_gpui::prelude::*;
//!
//! // 在视图的 render 中（cx 为 &mut Context<V>）：
//! Button::new("save", "Save")
//!     .variant(ButtonVariant::Filled)
//!     .style(|s: &mut ButtonStyle| s.height = px(48.).into()) // 实例级样式覆盖
//!     .leading_icon(IconName::Check)
//!     .on_click(|_, _, _| println!("clicked"))
//!     .build(cx)                      // -> Entity<ButtonState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::styles::button::ButtonStyle;
use crate::theme::{ActiveTheme, Elevation};

pub use crate::styles::button::ButtonVariant;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type StyleOverride = Box<dyn FnOnce(&mut ButtonStyle)>;

/// MD3 共享按钮构建器（`.build(cx)` 产出 [`ButtonState`]）。
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
    style_override: Option<StyleOverride>,
}

/// 按钮的有状态部分：交互表面（状态层/涟漪）与动画。
pub struct ButtonState {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    leading_icon: Option<IconName>,
    trailing_icon: Option<IconName>,
    disabled: bool,
    on_click: Option<ClickHandler>,
    style_override: Option<StyleOverride>,
    surface: InteractiveSurface,
}

impl Button {
    /// 创建按钮构建器。
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::default(),
            leading_icon: None,
            trailing_icon: None,
            disabled: false,
            on_click: None,
            style_override: None,
        }
    }

    /// 设置变体。
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Filled 变体。
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }

    /// Outlined 变体。
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }

    /// Text 变体。
    pub fn text(self) -> Self {
        self.variant(ButtonVariant::Text)
    }

    /// Elevated 变体。
    pub fn elevated(self) -> Self {
        self.variant(ButtonVariant::Elevated)
    }

    /// FilledTonal 变体。
    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }

    /// 设置前导图标。
    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// 设置尾随图标。
    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 设置点击回调。
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// 实例级样式覆盖（在令牌默认值之上应用）。
    pub fn style(mut self, override_fn: impl FnOnce(&mut ButtonStyle) + 'static) -> Self {
        self.style_override = Some(Box::new(override_fn));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<ButtonState> {
        cx.new(|_| ButtonState {
            id: self.id,
            label: self.label,
            variant: self.variant,
            leading_icon: self.leading_icon,
            trailing_icon: self.trailing_icon,
            disabled: self.disabled,
            on_click: self.on_click,
            style_override: self.style_override,
            surface: InteractiveSurface::new(),
        })
    }
}

impl ButtonState {
    /// 组件最近的边界（窗口坐标），可用于菜单等弹层锚定。
    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }

    /// 替换点击回调（用于构造后接线）。
    pub fn set_on_click(&mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) {
        self.on_click = Some(Rc::new(handler));
    }
}

impl AnimatedComponent for ButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for ButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 动画循环：仍有动画时调度下一帧
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let has_leading = self.leading_icon.is_some();
        let has_trailing = self.trailing_icon.is_some();

        // 样式：令牌默认 + 实例覆盖（render 不再内置样式决策）
        let mut style = if self.disabled {
            ButtonStyle::resolve_disabled(
                cx.theme().token_set(),
                self.variant,
                has_leading,
                has_trailing,
            )
        } else {
            ButtonStyle::resolve(
                cx.theme().token_set(),
                self.variant,
                has_leading,
                has_trailing,
            )
        };
        if let Some(override_fn) = self.style_override.take() {
            override_fn(&mut style);
        }

        let (pl, pr) = style.padding;
        let icon_size = style.icon_size;

        let base = div()
            .id(self.id.clone())
            .h(style.height)
            // Compose M3：MinWidth = 58dp
            .min_w(px(58.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(style.icon_gap)
            .pl(pl)
            .pr(pr)
            .rounded(style.corner_radius)
            .text_color(style.content_color)
            .when(!self.disabled, |el| el.cursor_pointer().overflow_hidden());
        let base = style.label.apply(base);

        let base = if let Some(bg_color) = style.container_color {
            base.bg(bg_color)
        } else {
            base
        };

        let base = if let Some(outline_color) = style.outline_color {
            base.border_1().border_color(outline_color)
        } else {
            base
        };

        // Compose M3：hover 时 Filled 升为 1 级、Elevated 升为 2 级阴影
        let hovered = self.surface.hovered && !self.disabled;
        let elevation = if hovered {
            match self.variant {
                ButtonVariant::Filled => Elevation::Level1,
                ButtonVariant::Elevated => Elevation::Level2,
                _ => style.elevation,
            }
        } else {
            style.elevation
        };
        let base = if elevation != Elevation::Level0 && !self.disabled {
            base.shadow(elevation.shadows(style.shadow_color))
        } else {
            base
        };

        let entity = cx.entity();
        let base = if self.disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                cx.theme().motion(),
                |s: &mut Self| &mut s.surface,
                style.state_layer_color,
                style.state_layer_opacity,
                style.corner_radius,
            )
        };

        let base = if self.disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.when_some(self.leading_icon, |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
        .child(self.label.clone())
        .when_some(self.trailing_icon, |el, icon| {
            el.child(Icon::new(icon).size(icon_size))
        })
    }
}
