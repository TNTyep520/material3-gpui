//! MD3 TopAppBar（对应 compose material3 的 `TopAppBar` 三档变体）
//!
//! 规格：Small 64dp（标题居中）、Medium 112dp 与 Large 152dp（标题位于
//! 左下），背景 surface,前后槽位放图标按钮:
//!
//! ```ignore
//! TopAppBar::small("app-bar").title("Title")
//!     .leading(IconButton::new("back", IconName::ArrowBack))
//!     .action(IconButton::new("more", IconName::Menu))
//! ```

use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled,
    Window, div, prelude::*, px,
};

use crate::prelude::ActiveTheme;

/// 顶栏高度档位。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAppBarVariant {
    /// 64dp，标题靠左。
    Small,
    /// 64dp，标题居中。
    CenterAligned,
    /// 112dp,标题左下。
    Medium,
    /// 152dp,标题左下。
    Large,
    /// 可在 112dp 与 64dp 之间折叠。
    MediumFlexible,
    /// 可在 152dp 与 64dp 之间折叠。
    LargeFlexible,
}

/// MD3 顶栏。
#[derive(IntoElement)]
pub struct TopAppBar {
    id: ElementId,
    variant: TopAppBarVariant,
    title: SharedString,
    leading: Vec<AnyElement>,
    actions: Vec<AnyElement>,
    collapsed_fraction: f32,
}

impl TopAppBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: TopAppBarVariant::Small,
            title: SharedString::default(),
            leading: Vec::new(),
            actions: Vec::new(),
            collapsed_fraction: 0.,
        }
    }

    pub fn small(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Small)
    }

    pub fn medium(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Medium)
    }

    /// 创建居中标题的顶部应用栏。
    pub fn center_aligned(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::CenterAligned)
    }

    pub fn large(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Large)
    }

    pub fn variant(mut self, variant: TopAppBarVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置可折叠应用栏进度，0 为展开，1 为折叠。
    pub fn collapsed_fraction(mut self, fraction: f32) -> Self {
        self.collapsed_fraction = fraction.clamp(0., 1.);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// 前置槽位（一般为返回/菜单图标按钮）。
    pub fn leading(mut self, leading: impl IntoElement) -> Self {
        self.leading.push(leading.into_any_element());
        self
    }

    /// 设置导航图标内容，对应 AndroidX navigationIcon 槽位。
    pub fn navigation_icon(self, content: impl IntoElement) -> Self {
        self.leading(content)
    }

    /// 后置动作槽位（图标按钮,从左到右追加）。
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

macro_rules! top_app_bar_variant {
    ($name:ident, $variant:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的顶部应用栏。")]
        #[derive(IntoElement)]
        pub struct $name(TopAppBar);

        impl $name {
            /// 创建指定标题布局的应用栏。
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(TopAppBar::new(id).variant(TopAppBarVariant::$variant))
            }

            /// 设置标题。
            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.title(title);
                self
            }

            /// 设置可折叠应用栏进度。
            pub fn collapsed_fraction(mut self, fraction: f32) -> Self {
                self.0 = self.0.collapsed_fraction(fraction);
                self
            }

            /// 添加导航图标或其他前置内容。
            pub fn navigation_icon(mut self, content: impl IntoElement) -> Self {
                self.0 = self.0.leading(content);
                self
            }

            /// 添加动作内容。
            pub fn action(mut self, content: impl IntoElement) -> Self {
                self.0 = self.0.action(content);
                self
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                self.0.render(window, cx)
            }
        }
    };
}

top_app_bar_variant!(CenterAlignedTopAppBar, CenterAligned);
top_app_bar_variant!(MediumTopAppBar, Medium);
top_app_bar_variant!(LargeTopAppBar, Large);
top_app_bar_variant!(MediumFlexibleTopAppBar, MediumFlexible);
top_app_bar_variant!(LargeFlexibleTopAppBar, LargeFlexible);

/// AndroidX TwoRowsTopAppBar 对应的双行标题应用栏。
pub type TwoRowsTopAppBar = LargeTopAppBar;

/// AndroidX BottomAppBar 对应的底部应用栏。
#[derive(IntoElement)]
pub struct BottomAppBar {
    id: ElementId,
    actions: Vec<AnyElement>,
    floating_action_button: Option<AnyElement>,
    collapsed_fraction: f32,
}

/// AndroidX BottomAppBarState 对应的折叠进度。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BottomAppBarState {
    /// 0 为展开，1 为折叠。
    pub collapsed_fraction: f32,
}

impl BottomAppBarState {
    /// 创建并限制折叠进度到 0..=1。
    pub fn new(collapsed_fraction: f32) -> Self {
        Self {
            collapsed_fraction: collapsed_fraction.clamp(0., 1.),
        }
    }
}

impl BottomAppBar {
    /// 创建底部应用栏。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            actions: Vec::new(),
            floating_action_button: None,
            collapsed_fraction: 0.,
        }
    }

    /// 添加一个操作控件。
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// 设置栏内的悬浮操作按钮。
    pub fn floating_action_button(mut self, button: impl IntoElement) -> Self {
        self.floating_action_button = Some(button.into_any_element());
        self
    }

    /// 设置底栏折叠状态。
    pub fn state(mut self, state: BottomAppBarState) -> Self {
        self.collapsed_fraction = state.collapsed_fraction;
        self
    }
}

/// AndroidX FlexibleBottomAppBar 对应的可折叠底部应用栏。
#[derive(IntoElement)]
pub struct FlexibleBottomAppBar(BottomAppBar);

impl FlexibleBottomAppBar {
    /// 创建展开的灵活底部应用栏。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self(BottomAppBar::new(id))
    }

    /// 添加操作控件。
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.0 = self.0.action(action);
        self
    }

    /// 设置栏内 FAB。
    pub fn floating_action_button(mut self, button: impl IntoElement) -> Self {
        self.0 = self.0.floating_action_button(button);
        self
    }

    /// 设置折叠状态。
    pub fn state(mut self, state: BottomAppBarState) -> Self {
        self.0 = self.0.state(state);
        self
    }
}

impl ParentElement for FlexibleBottomAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.0.extend(elements);
    }
}

impl RenderOnce for FlexibleBottomAppBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.0.render(window, cx)
    }
}

impl ParentElement for BottomAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements);
    }
}

impl RenderOnce for BottomAppBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        div()
            .id(self.id)
            .w_full()
            .h(px(80. - 16. * self.collapsed_fraction))
            .flex()
            .flex_none()
            .items_center()
            .gap(px(8.))
            .px(px(16.))
            .bg(colors.surface_container)
            .children(self.actions)
            .when_some(self.floating_action_button, |el, button| {
                el.child(div().ml_auto().child(button))
            })
    }
}

impl ParentElement for TopAppBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.actions.extend(elements)
    }
}

impl RenderOnce for TopAppBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let mut style = TopAppBarStyle::resolve(theme.token_set(), self.variant);
        if matches!(
            self.variant,
            TopAppBarVariant::MediumFlexible | TopAppBarVariant::LargeFlexible
        ) {
            style.height -= (style.height - px(64.)) * self.collapsed_fraction;
        }

        let small = matches!(
            self.variant,
            TopAppBarVariant::Small | TopAppBarVariant::CenterAligned
        ) || matches!(
            self.variant,
            TopAppBarVariant::MediumFlexible | TopAppBarVariant::LargeFlexible
        ) && self.collapsed_fraction >= 0.5;
        let centered = self.variant == TopAppBarVariant::CenterAligned;

        let leading_row = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(4.))
            .children(self.leading);

        let actions_row = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(4.))
            .children(self.actions);

        let title_element = style
            .title
            .apply(div())
            .text_color(style.title_color)
            .truncate()
            .child(self.title);

        let bar = div()
            .id(self.id)
            .w_full()
            .h(style.height)
            .flex_none()
            .flex()
            .flex_col()
            .bg(style.container_color);

        if small {
            // Small:单行,前置 | 居中标题 | 动作
            bar.child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_center()
                    .px(px(4.))
                    .gap(px(4.))
                    .child(leading_row)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .when(centered, |el| el.justify_center())
                            .when(!centered, |el| el.pl(px(12.)))
                            .child(title_element),
                    )
                    .child(actions_row),
            )
        } else {
            // Medium/Large:顶行前置+动作,标题沉底靠左(Compose 规范)
            bar.child(
                div()
                    .h(px(64.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .px(px(4.))
                    .gap(px(4.))
                    .child(leading_row)
                    .child(div().flex_1())
                    .child(actions_row),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .items_end()
                    .px(px(16.))
                    .pb(px(12.))
                    .child(title_element),
            )
        }
    }
}

pub use appearance::TopAppBarStyle;

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// TopAppBar 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct TopAppBarStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 标题色。
        pub title_color: Hsla,
        /// 图标色。
        pub icon_color: Hsla,
        /// 高度。
        pub height: Pixels,
        /// 水平内边距。
        pub horizontal_padding: Pixels,
        /// 元素间距。
        pub gap: Pixels,
        /// 标题字型。
        pub title: crate::theme::TypeStyle,
    }
    impl TopAppBarStyle {
        /// 由令牌推导默认样式(AppBar Small/Medium/Large 令牌)。
        pub fn resolve(tokens: &TokenSet, variant: super::TopAppBarVariant) -> Self {
            use crate::tokens::{AppBarLargeTokens, AppBarMediumTokens, AppBarSmallTokens};
            let (height, title) = match variant {
                super::TopAppBarVariant::Small | super::TopAppBarVariant::CenterAligned => (
                    AppBarSmallTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarSmallTokens::TITLE_FONT.resolve(tokens),
                ),
                super::TopAppBarVariant::Medium | super::TopAppBarVariant::MediumFlexible => (
                    AppBarMediumTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarMediumTokens::TITLE_FONT.resolve(tokens),
                ),
                super::TopAppBarVariant::Large | super::TopAppBarVariant::LargeFlexible => (
                    AppBarLargeTokens::CONTAINER_HEIGHT.pixels(),
                    AppBarLargeTokens::TITLE_FONT.resolve(tokens),
                ),
            };
            Self {
                container_color: tokens.colors.surface,
                title_color: tokens.colors.on_surface,
                icon_color: tokens.colors.on_surface_variant,
                height,
                horizontal_padding: px(16.),
                gap: px(8.),
                title,
            }
        }
    }
}
