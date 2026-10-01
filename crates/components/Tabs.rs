//! MD3 Tabs（对应 material-web 的 `md-tabs` / `md-primary-tab`）。
//!
//! Primary tabs：高 48dp（带图标 64dp），底部 3dp 圆角指示条。
//!
//! 指示条滑动动画移植自 [m3fx](https://github.com/Glavo/m3fx) 的
//! `M3TabBarSkin`（Apache-2.0，© 2026 Glavo）：选中指示条以
//! fastSpatial 弹簧在标签间滑动。
//!
//! ```ignore
//! TabBar::new("tabs")
//!     .tabs([Tab::new("One"), Tab::new("Two")])
//!     .selected(0)
//!     .on_change(|ix, _, _| {})
//!     .build(cx)   // -> Entity<TabBarState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::{ActiveTheme, HOVER_OPACITY, PRESSED_OPACITY};

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 单个标签页描述
pub struct Tab {
    /// 标签文本。
    pub label: SharedString,
    /// 可选图标。
    pub icon: Option<IconName>,
    /// 是否允许点击此标签。
    pub enabled: bool,
    leading_icon: bool,
}

impl Tab {
    /// 创建标签描述。
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            enabled: true,
            leading_icon: false,
        }
    }

    /// 设置图标。
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// 设置标签是否可交互。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 将图标放在标题左侧。
    pub fn leading_icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self.leading_icon = true;
        self
    }
}

/// AndroidX LeadingIconTab 对应的图标与文字并排标签。
pub struct LeadingIconTab(Tab);

impl LeadingIconTab {
    /// 创建带前导图标的标签。
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self(Tab::new(label).leading_icon(icon))
    }

    /// 设置标签是否可交互。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.0 = self.0.enabled(enabled);
        self
    }
}

impl From<LeadingIconTab> for Tab {
    fn from(tab: LeadingIconTab) -> Self {
        tab.0
    }
}

/// AndroidX 标签行的视觉层级。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabRowVariant {
    /// 主标签行，显示强调色指示条。
    #[default]
    Primary,
    /// 次级标签行，显示较细的指示条。
    Secondary,
}

/// MD3 标签栏构建器（`.build(cx)` 产出 [`TabBarState`]）。
pub struct TabBar {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    variant: TabRowVariant,
    scrollable: bool,
    on_change: Option<ChangeHandler>,
}

/// 标签栏的有状态部分：指示条滑动动画。
pub struct TabBarState {
    id: ElementId,
    tabs: Vec<Tab>,
    selected: usize,
    variant: TabRowVariant,
    scrollable: bool,
    on_change: Option<ChangeHandler>,
    /// 指示条位置（以标签下标为单位，弹簧驱动）。
    indicator: Animatable,
    driver: AnimationDriver,
}

impl TabBar {
    /// 创建标签栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected: 0,
            variant: TabRowVariant::default(),
            scrollable: false,
            on_change: None,
        }
    }

    /// 追加一个标签。
    pub fn tab(mut self, tab: impl Into<Tab>) -> Self {
        self.tabs.push(tab.into());
        self
    }

    /// 批量追加标签。
    pub fn tabs(mut self, tabs: impl IntoIterator<Item = impl Into<Tab>>) -> Self {
        self.tabs.extend(tabs.into_iter().map(Into::into));
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 设置主或次级标签行外观。
    pub fn variant(mut self, variant: TabRowVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置为可横向滚动的标签行。
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// 选中标签变化回调，参数为新选中的下标。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建有状态组件实体。
    pub fn build(self, cx: &mut App) -> Entity<TabBarState> {
        let selected = self.selected.min(self.tabs.len().saturating_sub(1));
        cx.new(|_| TabBarState {
            id: self.id,
            tabs: self.tabs,
            selected,
            variant: self.variant,
            scrollable: self.scrollable,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

/// AndroidX TabRow 对应的固定宽度标签行。
pub type TabRow = TabBar;

macro_rules! tab_row_variant {
    ($name:ident, $variant:ident, $scrollable:expr) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的标签行。")]
        pub struct $name(TabBar);

        impl $name {
            /// 创建指定视觉层级和滚动方式的标签行。
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(
                    TabBar::new(id)
                        .variant(TabRowVariant::$variant)
                        .scrollable($scrollable),
                )
            }

            /// 添加单个标签。
            pub fn tab(mut self, tab: impl Into<Tab>) -> Self {
                self.0 = self.0.tab(tab);
                self
            }

            /// 批量添加标签。
            pub fn tabs(mut self, tabs: impl IntoIterator<Item = impl Into<Tab>>) -> Self {
                self.0 = self.0.tabs(tabs);
                self
            }

            /// 设置初始选中下标。
            pub fn selected(mut self, index: usize) -> Self {
                self.0 = self.0.selected(index);
                self
            }

            /// 设置选中项变化回调。
            pub fn on_change(
                mut self,
                handler: impl Fn(usize, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_change(handler);
                self
            }

            /// 构建可渲染的标签行实体。
            pub fn build(self, cx: &mut App) -> Entity<TabBarState> {
                self.0.build(cx)
            }
        }
    };
}

tab_row_variant!(PrimaryTabRow, Primary, false);
tab_row_variant!(SecondaryTabRow, Secondary, false);
tab_row_variant!(PrimaryScrollableTabRow, Primary, true);
tab_row_variant!(SecondaryScrollableTabRow, Secondary, true);

impl TabBarState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换到指定标签（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.tabs.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for TabBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for TabBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }

        let theme = cx.theme();
        let colors = theme.colors();
        let selected = self.selected;
        let on_change = self.on_change.clone();
        let entity = cx.entity();
        let style = TabBarStyle::resolve(
            theme.token_set(),
            self.tabs.iter().any(|tab| tab.icon.is_some()),
        );
        let label_style = style.label;
        let has_icons = self.tabs.iter().any(|t| t.icon.is_some());
        let height = style.bar_height(has_icons);
        let indicator_pos = self.indicator.value() as f32;

        let primary = style.selected_item_color;
        let on_surface_variant = style.unselected_item_color;
        let surface = style.container_color;
        let outline_variant = style.divider_color;

        div()
            .id(self.id.clone())
            .w_full()
            .flex()
            .overflow_hidden()
            .when(self.scrollable, |el| el.overflow_x_scroll())
            .bg(surface)
            .border_b_1()
            .border_color(outline_variant)
            .children(self.tabs.iter().enumerate().map(|(ix, tab)| {
                let is_selected = ix == selected;
                let fg = if is_selected {
                    primary
                } else {
                    on_surface_variant
                };
                let layer = if is_selected {
                    primary
                } else {
                    colors.on_surface
                };
                let on_change = on_change.clone();
                let click_entity = entity.clone();
                let tab_el = div()
                    .id((SharedString::from(format!("{}-tab", self.id)), ix))
                    .relative()
                    .flex_1()
                    .when(self.scrollable, |el| el.flex_none().w(px(120.)))
                    .h(height)
                    .flex()
                    .flex_col()
                    .when(tab.leading_icon, |el| el.flex_row())
                    .items_center()
                    .justify_center()
                    .gap(style.gap)
                    .when(tab.enabled, |el| el.cursor_pointer())
                    .text_color(fg)
                    .hover(move |s| s.bg(layer.opacity(HOVER_OPACITY)))
                    .active(move |s| s.bg(layer.opacity(PRESSED_OPACITY)))
                    // 点击:组件内部先完成选中(弹簧滑动),
                    // 状态真正变化才触发一次 on_change
                    .when(tab.enabled, |el| {
                        el.on_click(move |_, window, cx| {
                            click_entity.update(cx, |state, cx| {
                                let changed = ix != state.selected;
                                state.select(ix, window, cx);
                                if changed && let Some(handler) = on_change.clone() {
                                    handler(ix, window, cx);
                                }
                            });
                        })
                    })
                    .when_some(tab.icon, |el, icon| {
                        el.child(Icon::new(icon).size(style.icon_size))
                    });
                let tab_el = label_style.apply(tab_el).child(tab.label.clone());
                // 选中指示条：3dp 高、圆角上边、宽度收窄。
                // 画在选中标签内部并以弹簧位置做相对偏移
                //（一个标签宽度 = 1.0 个 relative 单位，0.5 为标签中心，
                // 滑动时随偏移跨标签平移，由容器 overflow_hidden 裁剪）。
                tab_el.when(is_selected, |el| {
                    el.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left(relative(0.5 + indicator_pos - ix as f32))
                            .w(px(48.))
                            .ml(px(-24.))
                            .flex()
                            .justify_center()
                            .child(
                                div()
                                    .h(if self.variant == TabRowVariant::Secondary {
                                        px(2.)
                                    } else {
                                        px(3.)
                                    })
                                    .w(px(48.))
                                    .rounded_tl(px(3.))
                                    .rounded_tr(px(3.))
                                    .bg(primary),
                            ),
                    )
                })
            }))
    }
}

pub use appearance::TabBarStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// MD3 标签栏样式。
    #[derive(Clone, Copy, Debug)]
    pub struct TabBarStyle {
        /// 栏背景色。
        pub container_color: Hsla,
        /// 底部分隔线色。
        pub divider_color: Hsla,
        /// 选中项内容色。
        pub selected_item_color: Hsla,
        /// 未选中项内容色。
        pub unselected_item_color: Hsla,
        /// 指示条颜色。
        pub indicator_color: Hsla,
        /// 指示条高/宽。
        pub indicator_size: (Pixels, Pixels),
        /// 无图标时栏高。
        pub height: Pixels,
        /// 带图标时栏高。
        pub height_with_icon: Pixels,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 图标与文字间距。
        pub gap: Pixels,
        /// hover 状态层不透明度。
        pub hover_opacity: f32,
        /// 按压状态层不透明度。
        pub pressed_opacity: f32,
        /// 标签字型。
        pub label: crate::theme::TypeStyle,
    }
    impl TabBarStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet, _has_icons: bool) -> Self {
            let colors = &tokens.colors;
            Self {
                container_color: colors.surface,
                divider_color: colors.outline_variant,
                selected_item_color: colors.primary,
                unselected_item_color: colors.on_surface_variant,
                indicator_color: colors.primary,
                indicator_size: (px(3.), px(48.)),
                height: px(48.),
                height_with_icon: px(64.),
                icon_size: px(24.),
                gap: px(4.),
                hover_opacity: crate::theme::HOVER_OPACITY,
                pressed_opacity: crate::theme::PRESSED_OPACITY,
                label: tokens.typography.label_large,
            }
        }
    }
    impl TabBarStyle {
        /// 当前使用的栏高。
        pub fn bar_height(&self, has_icons: bool) -> Pixels {
            if has_icons {
                self.height_with_icon
            } else {
                self.height
            }
        }
    }
}
