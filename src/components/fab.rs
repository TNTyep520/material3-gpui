//! MD3 FAB（对应 material-web 的 `md-fab`）。
//!
//! 尺寸：Small 40dp / Standard 56dp / Large 96dp；
//! 颜色：Surface / Primary / Secondary / Tertiary；
//! 支持 Extended FAB（带文字标签）。
//!
//! 交互行为移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3FloatingActionButtonSkin`（Apache-2.0，© 2026 Glavo）。
//!
//! ```ignore
//! Fab::new("add", IconName::Add)
//!     .label("New item")
//!     .on_click(|_, _, _| {})
//!     .build(cx)   // -> Entity<FabState>
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
use crate::theme::{ActiveTheme, Elevation};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// FAB 尺寸。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabSize {
    /// 40dp。
    Small,
    /// 56dp（默认）。
    #[default]
    Standard,
    /// 96dp。
    Large,
}

/// FAB 配色。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FabColor {
    /// 表面配色。
    Surface,
    /// 主色配色（默认）。
    #[default]
    Primary,
    /// 次级配色。
    Secondary,
    /// 第三配色。
    Tertiary,
}

/// MD3 悬浮操作按钮构建器（`.build(cx)` 产出 [`FabState`]）。
pub struct Fab {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
}

/// FAB 的有状态部分。
pub struct FabState {
    id: ElementId,
    icon: IconName,
    size: FabSize,
    color: FabColor,
    label: Option<SharedString>,
    lowered: bool,
    on_click: Option<ClickHandler>,
    surface: InteractiveSurface,
}

impl Fab {
    /// 创建 FAB 构建器。
    pub fn new(id: impl Into<ElementId>, icon: IconName) -> Self {
        Self {
            id: id.into(),
            icon,
            size: FabSize::default(),
            color: FabColor::default(),
            label: None,
            lowered: false,
            on_click: None,
        }
    }

    /// 设置尺寸。
    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// 设置配色。
    pub fn color(mut self, color: FabColor) -> Self {
        self.color = color;
        self
    }

    /// Extended FAB：附带文字标签（固定高 56dp）。
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// 降低的 elevation（Level1 而非 Level3）。
    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
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
    pub fn build(self, cx: &mut App) -> Entity<FabState> {
        cx.new(|_| FabState {
            id: self.id,
            icon: self.icon,
            size: self.size,
            color: self.color,
            label: self.label,
            lowered: self.lowered,
            on_click: self.on_click,
            surface: InteractiveSurface::new(),
        })
    }
}

impl AnimatedComponent for FabState {
    fn step(&mut self, now: Instant) -> bool {
        self.surface.step(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        self.surface.driver_mut()
    }
}

impl Render for FabState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surface.is_animating() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let shapes = *theme.shapes();
        let state_layer = *theme.state_layer();

        let (bg, fg) = match self.color {
            FabColor::Surface => (colors.surface_container_high, colors.primary),
            FabColor::Primary => (colors.primary_container, colors.on_primary_container),
            FabColor::Secondary => (colors.secondary_container, colors.on_secondary_container),
            FabColor::Tertiary => (colors.tertiary_container, colors.on_tertiary_container),
        };

        let extended = self.label.is_some();
        let (container, radius, icon_size) = if extended {
            (px(56.), shapes.large, px(24.))
        } else {
            match self.size {
                FabSize::Small => (px(40.), shapes.medium, px(24.)),
                FabSize::Standard => (px(56.), shapes.large, px(24.)),
                FabSize::Large => (px(96.), shapes.extra_large, px(36.)),
            }
        };

        let elevation = if self.lowered {
            Elevation::Level1
        } else {
            Elevation::Level3
        };

        let label_style = theme.typography().label_large;
        let shadow_color = colors.shadow;

        let base = div()
            .id(self.id.clone())
            .h(container)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .rounded(radius)
            .bg(bg)
            .text_color(fg)
            .shadow(elevation.shadows(shadow_color))
            .cursor_pointer()
            .overflow_hidden();

        let base = if extended {
            label_style.apply(base.pl(px(16.)).pr(px(20.)))
        } else {
            base.w(container)
        };

        let entity = cx.entity();
        let base = {
            crate::interaction::wire(
                &self.surface,
                base,
                &entity,
                theme.motion(),
                |s: &mut Self| &mut s.surface,
                fg,
                state_layer.pressed,
                radius,
            )
        };

        let base = if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        base.child(Icon::new(self.icon).size(icon_size))
            .when_some(self.label.clone(), |el, label| el.child(label))
    }
}
