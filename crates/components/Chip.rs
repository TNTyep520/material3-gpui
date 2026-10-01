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
type SelectionHandler = Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>;

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
    on_selected_change: Option<SelectionHandler>,
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
    on_selected_change: Option<SelectionHandler>,
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
            on_selected_change: None,
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

    /// 设置启用状态；禁用时阻止点击与选择切换。
    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    /// 设置 Filter chip 的选中回调，传递新的选中状态。
    pub fn on_selected_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selected_change = Some(Rc::new(handler));
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
            on_selected_change: self.on_selected_change,
            on_remove: self.on_remove,
            surface: InteractiveSurface::new(),
        })
    }
}

impl ChipState {
    /// 返回当前选中状态。
    pub fn selected(&self) -> bool {
        self.selected
    }

    /// 设置选中状态，不触发选择回调。
    pub fn set_selected(&mut self, selected: bool, cx: &mut Context<Self>) {
        if self.selected != selected {
            self.selected = selected;
            cx.notify();
        }
    }
}

macro_rules! chip_variant {
    ($name:ident, $variant:ident, $elevated:expr) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的 GPUI chip。")]
        pub struct $name(Chip);

        impl $name {
            /// 创建带标签的 chip。
            pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
                Self(
                    Chip::new(id, label)
                        .variant(ChipVariant::$variant)
                        .elevated($elevated),
                )
            }

            /// 设置启用状态。
            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            /// 设置选中状态，供可选择的 chip 使用。
            pub fn selected(mut self, selected: bool) -> Self {
                self.0 = self.0.selected(selected);
                self
            }

            /// 设置前导图标。
            pub fn leading_icon(mut self, icon: IconName) -> Self {
                self.0 = self.0.leading_icon(icon);
                self
            }

            /// 设置点击回调。
            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_click(handler);
                self
            }

            /// 设置选中变化回调。
            pub fn on_selected_change(
                mut self,
                handler: impl Fn(bool, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_selected_change(handler);
                self
            }

            /// 设置 Input chip 的移除操作。
            pub fn on_remove(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_remove(handler);
                self
            }

            /// 创建可渲染的 chip 实体。
            pub fn build(self, cx: &mut App) -> Entity<ChipState> {
                self.0.build(cx)
            }
        }
    };
}

chip_variant!(AssistChip, Assist, false);
chip_variant!(ElevatedAssistChip, Assist, true);
chip_variant!(FilterChip, Filter, false);
chip_variant!(ElevatedFilterChip, Filter, true);
chip_variant!(InputChip, Input, false);
chip_variant!(SuggestionChip, Suggestion, false);
chip_variant!(ElevatedSuggestionChip, Suggestion, true);

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
        let state_layer = *theme.state_layer();
        let disabled = self.disabled;
        let selected =
            self.selected && matches!(self.variant, ChipVariant::Filter | ChipVariant::Input);
        let style = ChipStyle::resolve(
            theme.token_set(),
            self.variant,
            selected,
            self.elevated,
            disabled,
        );
        let fg = style.content_color;
        let icon_color = style.icon_color;
        let bg = style.container_color;

        // Filter chip 选中时自动带勾图标
        let leading = if self.variant == ChipVariant::Filter && selected {
            Some(IconName::Check)
        } else {
            self.leading_icon
        };

        let has_leading = leading.is_some();
        let has_trailing = self.on_remove.is_some();
        let label_style = style.label;

        let base = div()
            .id(self.id.clone())
            .h(style.height)
            .flex()
            .flex_none()
            .items_center()
            .gap(style.gap)
            .rounded(style.corner_radius)
            .pl(if has_leading {
                style.edge_padding
            } else {
                style.center_padding
            })
            .pr(if has_trailing {
                style.edge_padding
            } else {
                style.center_padding
            })
            .text_color(fg);
        let base = label_style.apply(base);

        let base = base
            .when_some(bg, |el, bg_color| el.bg(bg_color))
            .when_some(style.outline_color, |el, color| {
                el.border_1().border_color(color)
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
                style.corner_radius,
            )
        };

        let base = if disabled {
            base
        } else if self.variant == ChipVariant::Filter {
            let click_handler = self.on_click.clone();
            let selection_handler = self.on_selected_change.clone();
            base.on_click(cx.listener(move |this, event, window, cx| {
                let selected = !this.selected;
                this.set_selected(selected, cx);
                if let Some(handler) = &selection_handler {
                    handler(selected, window, cx);
                }
                if let Some(handler) = &click_handler {
                    handler(event, window, cx);
                }
            }))
        } else if let Some(handler) = self.on_click.clone() {
            base.on_click(move |event, window, cx| handler(event, window, cx))
        } else {
            base
        };

        let base = base
            .when_some(leading, |el, icon| {
                el.child(Icon::new(icon).size(style.icon_size).color(icon_color))
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
                        .child(Icon::new(IconName::Close).size(style.close_size).color(fg)),
                )
            },
        )
    }
}

pub use appearance::ChipStyle;

mod appearance {
    use super::ChipVariant;
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 纸片样式。
    #[derive(Clone, Copy, Debug)]
    pub struct ChipStyle {
        /// 容器色（`None` 为透明）。
        pub container_color: Option<Hsla>,
        /// 内容色。
        pub content_color: Hsla,
        /// 前导图标色。
        pub icon_color: Hsla,
        /// 描边色（`Some` 启用 1dp 描边）。
        pub outline_color: Option<Hsla>,
        /// 高度。
        pub height: Pixels,
        /// 圆角。
        pub corner_radius: Pixels,
        /// 带前导/尾随元素一侧的水平内边距。
        pub edge_padding: Pixels,
        /// 无前导/尾随元素一侧的水平内边距。
        pub center_padding: Pixels,
        /// 元素间距。
        pub gap: Pixels,
        /// 前导图标尺寸。
        pub icon_size: Pixels,
        /// 移除按钮尺寸。
        pub close_size: Pixels,
        /// 状态层基色。
        pub state_layer_color: Hsla,
        /// 按压档状态层不透明度。
        pub state_layer_opacity: f32,
        /// 文字字型。
        pub label: crate::theme::TypeStyle,
    }
    impl ChipStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(
            tokens: &TokenSet,
            variant: ChipVariant,
            selected: bool,
            elevated: bool,
            disabled: bool,
        ) -> Self {
            let colors = &tokens.colors;
            let state = &tokens.state_layer;
            let selected = selected && matches!(variant, ChipVariant::Filter | ChipVariant::Input);
            let (container, content, icon) = if disabled {
                (
                    (selected || elevated).then(|| colors.disabled_container(state)),
                    colors.disabled_content(state),
                    colors.disabled_content(state),
                )
            } else if selected {
                (
                    Some(colors.secondary_container),
                    colors.on_secondary_container,
                    colors.on_secondary_container,
                )
            } else if elevated {
                (
                    Some(colors.surface_container_low),
                    colors.on_surface,
                    colors.primary,
                )
            } else {
                (
                    None,
                    colors.on_surface_variant,
                    if variant == ChipVariant::Input {
                        colors.on_surface_variant
                    } else {
                        colors.primary
                    },
                )
            };

            Self {
                container_color: container,
                content_color: content,
                icon_color: icon,
                outline_color: if container.is_none() && !elevated {
                    Some(if disabled {
                        colors.on_surface.opacity(state.disabled_container)
                    } else {
                        colors.outline_variant
                    })
                } else {
                    None
                },
                height: px(32.),
                corner_radius: tokens.shapes.small,
                edge_padding: px(8.),
                center_padding: px(16.),
                gap: px(8.),
                icon_size: px(18.),
                close_size: px(16.),
                state_layer_color: content,
                state_layer_opacity: state.pressed,
                label: tokens.typography.label_large,
            }
        }
    }
}
