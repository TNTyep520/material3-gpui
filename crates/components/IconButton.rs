//! MD3 IconButton（对应 material-web 的 `md-icon-button` 系列）。
//!
//! 变体：Standard / Filled / FilledTonal / Outlined。
//! 规格：容器 40×40dp、图标 24dp、圆形。支持 toggle（选中态）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3IconButtonSkin` / `M3IconToggleButtonSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! IconButton::new("fav", IconName::Favorite)
//!     .selected(is_fav)
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<IconButtonState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, ClickEvent, Context, ElementId, Entity, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::InteractiveSurface;
use crate::motion::{AnimatedComponent, AnimationDriver};
use crate::theme::ActiveTheme;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 图标按钮变体。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconButtonVariant {
    /// 标准无容器图标按钮。
    #[default]
    Standard,
    /// 实心图标按钮。
    Filled,
    /// 次级色调实心图标按钮。
    FilledTonal,
    /// 描边图标按钮。
    Outlined,
}

/// MD3 图标按钮构建器（`.build(cx)` 产出 [`IconButtonState`]）。
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
}

/// 图标按钮的有状态部分。
pub struct IconButtonState {
    id: ElementId,
    icon: IconName,
    variant: IconButtonVariant,
    selected: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl IconButton {
    /// 创建图标按钮构建器。
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            variant: IconButtonVariant::default(),
            selected: false,
            disabled: false,
            on_click: None,
        }
    }

    /// 设置变体。
    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Filled 变体。
    pub fn filled(self) -> Self {
        self.variant(IconButtonVariant::Filled)
    }

    /// FilledTonal 变体。
    pub fn tonal(self) -> Self {
        self.variant(IconButtonVariant::FilledTonal)
    }

    /// Outlined 变体。
    pub fn outlined(self) -> Self {
        self.variant(IconButtonVariant::Outlined)
    }

    /// toggle 选中态。
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<IconButtonState> {
        cx.new(|_| IconButtonState {
            id: self.id,
            icon: self.icon,
            variant: self.variant,
            selected: self.selected,
            disabled: self.disabled,
            on_click: self.on_click,
            surface: InteractiveSurface::new(),
        })
    }
}

impl IconButtonState {
    /// 组件最近的边界（窗口坐标），可用于菜单等弹层锚定。
    pub fn bounds(&self) -> gpui::Bounds<gpui::Pixels> {
        self.surface.bounds.get()
    }
}

impl AnimatedComponent for IconButtonState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for IconButtonState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let selected = self.selected;

        let (bg, fg, outlined) = match (self.variant, selected) {
            (IconButtonVariant::Standard, false) => (None, colors.on_surface_variant, false),
            (IconButtonVariant::Standard, true) => (None, colors.primary, false),
            (IconButtonVariant::Filled, false) => (
                Some(colors.surface_container_highest),
                colors.primary,
                false,
            ),
            (IconButtonVariant::Filled, true) => (Some(colors.primary), colors.on_primary, false),
            (IconButtonVariant::FilledTonal, false) => (
                Some(colors.surface_container_highest),
                colors.on_surface_variant,
                false,
            ),
            (IconButtonVariant::FilledTonal, true) => (
                Some(colors.secondary_container),
                colors.on_secondary_container,
                false,
            ),
            (IconButtonVariant::Outlined, false) => (None, colors.on_surface_variant, true),
            (IconButtonVariant::Outlined, true) => (
                Some(colors.inverse_surface),
                colors.inverse_on_surface,
                false,
            ),
        };

        let fg = if disabled {
            colors.disabled_content(&state_layer)
        } else {
            fg
        };
        let bg = if disabled {
            bg.map(|_| colors.disabled_container(&state_layer))
        } else {
            bg
        };

        let base = div()
            .id(self.id.clone())
            .size(px(40.))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .text_color(fg)
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when(outlined, |el| {
                el.border_1().border_color(if disabled {
                    colors.disabled_content(&state_layer)
                } else {
                    colors.outline
                })
            })
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
                gpui::px(999.),
            )
        };

        let base = if disabled {
            base
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.child(Icon::new(self.icon).size(px(24.)))
    }
}
