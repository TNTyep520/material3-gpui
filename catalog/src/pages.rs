//! catalog 页面模块：每个组件页面一个独立 Entity 视图。
//!
//! 页面视图只在自身状态变化时重渲染自己（Slider 拖动、Progress 动画
//! 等不再触发整棵应用树重绘——掉帧修复的核心）。

// 页面文件按 `Page_<Name>` 命名(多词用驼峰式),非 snake_case
#![allow(non_snake_case)]

pub mod Page_Buttons;
pub mod Page_Cards;
pub mod Page_Chips;
pub mod Page_Dialogs;
pub mod Page_IconButtonsFab;
pub mod Page_Lists;
pub mod Page_Navigation;
pub mod Page_Overlays;
pub mod Page_Overview;
pub mod Page_Selection;
pub mod Page_SliderProgress;
pub mod Page_Tabs;
pub mod Page_TextFields;

use gpui::{AnyElement, App, Entity, FontWeight, IntoElement, Styled, div, prelude::*, px};

// 页面间回调类型（页 → 根）。
pub type PageCallback<A> = std::rc::Rc<dyn Fn(A, &mut App)>;

use material3_gpui::prelude::*;

/// 页面集合：根视图持有并按导航切换。
#[derive(Clone)]
pub struct Pages {
    pub overview: Entity<Page_Overview::OverviewPage>,
    pub buttons: Entity<Page_Buttons::ButtonsPage>,
    pub icon_buttons_fab: Entity<Page_IconButtonsFab::IconButtonsFabPage>,
    pub selection: Entity<Page_Selection::SelectionPage>,
    pub chips: Entity<Page_Chips::ChipsPage>,
    pub slider_progress: Entity<Page_SliderProgress::SliderProgressPage>,
    pub tabs: Entity<Page_Tabs::TabsPage>,
    pub text_fields: Entity<Page_TextFields::TextFieldsPage>,
    pub overlays: Entity<Page_Overlays::OverlaysPage>,
    pub navigation: Entity<Page_Navigation::NavigationPage>,
    pub cards: Entity<Page_Cards::CardsPage>,
    pub lists: Entity<Page_Lists::ListsPage>,
    pub dialogs: Entity<Page_Dialogs::DialogsPage>,
}

impl Pages {
    /// 创建全部页面视图（组件实体在各页面构造函数中只创建一次）。
    pub fn new(cx: &mut App) -> Self {
        Self {
            overview: Page_Overview::OverviewPage::new(cx),
            buttons: Page_Buttons::ButtonsPage::new(cx),
            icon_buttons_fab: Page_IconButtonsFab::IconButtonsFabPage::new(cx),
            selection: Page_Selection::SelectionPage::new(cx),
            chips: Page_Chips::ChipsPage::new(cx),
            slider_progress: Page_SliderProgress::SliderProgressPage::new(cx),
            tabs: Page_Tabs::TabsPage::new(cx),
            text_fields: Page_TextFields::TextFieldsPage::new(cx),
            overlays: Page_Overlays::OverlaysPage::new(cx),
            navigation: Page_Navigation::NavigationPage::new(cx),
            cards: Page_Cards::CardsPage::new(cx),
            lists: Page_Lists::ListsPage::new(cx),
            dialogs: Page_Dialogs::DialogsPage::new(cx),
        }
    }
}

/// m3fx 风格演示画廊:展示组垂直排列,组间距 18(对齐 m3fx `createGallery`)。
pub(crate) fn gallery(groups: impl IntoIterator<Item = AnyElement>) -> impl IntoElement {
    div().flex().flex_col().gap(px(18.)).children(groups)
}

/// m3fx 风格展示组:粗体 14px 标题 + 圆角卡片(surface-container-low
/// 背景、12px 圆角、18px 内边距),内容以 16px 间距流式排布
/// (对齐 m3fx `createShowcaseGroup` 与 m3fx-demo.css)。
pub(crate) fn showcase_group(
    cx: &App,
    title: &'static str,
    items: impl IntoIterator<Item = AnyElement>,
) -> AnyElement {
    let theme = cx.theme();
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.colors().on_surface)
                .child(title),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(16.))
                .p(px(18.))
                .rounded(px(12.))
                .bg(theme.colors().surface_container_low)
                .children(items),
        )
        .into_any_element()
}

/// 演示卡片。
pub(crate) fn catalog_card(cx: &App, card: Card, title: &'static str) -> impl IntoElement {
    let theme = cx.theme();
    card.w(px(220.))
        .p(px(16.))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(theme.typography().title_medium.apply(div()).child(title))
        .child(
            theme
                .typography()
                .body_medium
                .apply(div())
                .text_color(theme.colors().on_surface_variant)
                .child("Cards contain content and actions about a single subject."),
        )
}
