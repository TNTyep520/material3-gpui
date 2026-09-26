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
    /// 64dp,标题居中。
    Small,
    /// 112dp,标题左下。
    Medium,
    /// 152dp,标题左下。
    Large,
}

/// MD3 顶栏。
#[derive(IntoElement)]
pub struct TopAppBar {
    id: ElementId,
    variant: TopAppBarVariant,
    title: SharedString,
    leading: Vec<AnyElement>,
    actions: Vec<AnyElement>,
}

impl TopAppBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: TopAppBarVariant::Small,
            title: SharedString::default(),
            leading: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn small(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Small)
    }

    pub fn medium(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Medium)
    }

    pub fn large(id: impl Into<ElementId>) -> Self {
        Self::new(id).variant(TopAppBarVariant::Large)
    }

    pub fn variant(mut self, variant: TopAppBarVariant) -> Self {
        self.variant = variant;
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

    /// 后置动作槽位（图标按钮,从左到右追加）。
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
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
        let colors = theme.colors();
        let typography = theme.typography();

        let (height, centered) = match self.variant {
            TopAppBarVariant::Small => (px(64.), true),
            TopAppBarVariant::Medium => (px(112.), false),
            TopAppBarVariant::Large => (px(152.), false),
        };

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

        // Small:标题居中(label_large);Medium/Large:标题沉底靠左(headline)
        let title_element = match self.variant {
            TopAppBarVariant::Small => typography
                .label_large
                .apply(div())
                .text_color(colors.on_surface)
                .truncate()
                .child(self.title),
            TopAppBarVariant::Medium => typography
                .headline_small
                .apply(div())
                .text_color(colors.on_surface)
                .truncate()
                .child(self.title),
            TopAppBarVariant::Large => typography
                .headline_large
                .apply(div())
                .text_color(colors.on_surface)
                .truncate()
                .child(self.title),
        };

        let bar = div()
            .id(self.id)
            .w_full()
            .h(height)
            .flex_none()
            .flex()
            .flex_col()
            .bg(colors.surface);

        if centered {
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
                            .justify_center()
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
