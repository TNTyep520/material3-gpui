//! MD3 Chips（对应 material-web 的 `md-*-chip`）。
//!
//! 变体：Assist / Filter / Input / Suggestion。
//! 规格：高 32dp、圆角 8dp、label-large 字体。
//! Filter chip 选中时显示勾图标 + secondary-container 底色；
//! Input chip 可带尾部移除按钮。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3ChipSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! Chip::new("tag", "Rust")
//!     .variant(ChipVariant::Filter)
//!     .selected(active)
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<ChipState>
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
use crate::theme::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 纸片变体。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChipVariant {
    /// 辅助纸片。
    #[default]
    Assist,
    /// 过滤纸片（可选中）。
    Filter,
    /// 输入纸片（可移除）。
    Input,
    /// 建议纸片。
    Suggestion,
}

/// MD3 纸片构建器（`.build(cx)` 产出 [`ChipState`]）。
pub struct Chip {
    id: ElementId,
    label: SharedString,
    variant: ChipVariant,
    selected: bool,
    disabled: bool,
    elevated: bool,
    leading_icon: Option<IconName>,
    on_click: Option<ClickHandler>,
    on_remove: Option<ClickHandler>,
}

/// 纸片的有状态部分。
pub struct ChipState {
    id: ElementId,
    label: SharedString,
    variant: ChipVariant,
    selected: bool,
    disabled: bool,
    elevated: bool,
    leading_icon: Option<IconName>,
    on_click: Option<ClickHandler>,
    on_remove: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl Chip {
    /// 创建纸片构建器。
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ChipVariant::default(),
            selected: false,
            disabled: false,
            elevated: false,
            leading_icon: None,
            on_click: None,
            on_remove: None,
        }
    }

    /// 设置变体。
    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Assist 变体。
    pub fn assist(self) -> Self {
        self.variant(ChipVariant::Assist)
    }

    /// Filter 变体。
    pub fn filter(self) -> Self {
        self.variant(ChipVariant::Filter)
    }

    /// Input 变体。
    pub fn input(self) -> Self {
        self.variant(ChipVariant::Input)
    }

    /// Suggestion 变体。
    pub fn suggestion(self) -> Self {
        self.variant(ChipVariant::Suggestion)
    }

    /// 选中态（主要用于 Filter chip）。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// 设置禁用态。
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 悬浮样式（elevated，无描边 + 阴影）。
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    /// 设置前导图标。
    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.leading_icon = Some(icon);
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

    /// Input chip 的尾部移除按钮回调（设置后显示 × 按钮）。
    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<ChipState> {
        cx.new(|_| ChipState {
            id: self.id,
            label: self.label,
            variant: self.variant,
            selected: self.selected,
            disabled: self.disabled,
            elevated: self.elevated,
            leading_icon: self.leading_icon,
            on_click: self.on_click,
            on_remove: self.on_remove,
            surface: InteractiveSurface::new(),
        })
    }
}

impl AnimatedComponent for ChipState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for ChipState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let shapes = *theme.shapes();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let selected = self.selected;

        let fg = if disabled {
            colors.disabled_content(&state_layer)
        } else if selected {
            colors.on_secondary_container
        } else {
            colors.on_surface
        };
        let icon_color = if disabled {
            fg
        } else if selected {
            colors.on_secondary_container
        } else {
            colors.primary
        };

        let bg = if selected {
            Some(colors.secondary_container)
        } else if self.elevated {
            Some(colors.surface_container_low)
        } else {
            None
        };

        // Filter chip 选中时自动带勾图标
        let leading = if self.variant == ChipVariant::Filter && selected {
            Some(IconName::Check)
        } else {
            self.leading_icon
        };

        let has_leading = leading.is_some();
        let has_trailing = self.on_remove.is_some();
        let label_style = theme.typography().label_large;
        let outline_color = if disabled {
            colors.disabled_content(&state_layer)
        } else {
            colors.outline_variant
        };
        let show_outline = bg.is_none() && !self.elevated;

        let base = div()
            .id(self.id.clone())
            .h(px(32.))
            .flex()
            .flex_none()
            .items_center()
            .gap(px(8.))
            .rounded(shapes.small)
            .pl(if has_leading { px(8.) } else { px(16.) })
            .pr(if has_trailing { px(8.) } else { px(16.) })
            .text_color(fg);
        let base = label_style.apply(base);

        let base = base
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when(show_outline, |el| el.border_1().border_color(outline_color))
            .when(!disabled, |el| el.cursor_pointer().overflow_hidden());

        let entity = cx.entity();
        let base = if disabled {
            base
        } else {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                fg,
                state_layer.pressed,
                shapes.small,
            )
        };

        let base = if disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        let base = base
            .when_some(leading, |el, icon| {
                el.child(Icon::new(icon).size(px(18.)).color(icon_color))
            })
            .child(self.label.clone());

        base.when_some(
            self.on_remove.clone().filter(|_| !disabled),
            |el, handler| {
                el.child(
                    div()
                        // NamedChild:由父 id 组合子 id,不分配字符串
                        .id((self.id.clone(), "remove"))
                        .size(px(18.))
                        .flex()
                        .flex_none()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .cursor_pointer()
                        .hover(move |s| s.bg(fg.opacity(state_layer.hover)))
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            handler(event, window, cx)
                        })
                        .child(Icon::new(IconName::Close).size(px(16.)).color(fg)),
                )
            },
        )
    }
}
