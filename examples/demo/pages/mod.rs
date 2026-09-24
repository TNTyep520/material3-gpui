//! demo 页面模块：每个组件页面一个独立 Entity 视图。
//!
//! 页面视图只在自身状态变化时重渲染自己（Slider 拖动、Progress 动画
//! 等不再触发整棵应用树重绘——掉帧修复的核心）。

pub mod buttons;
pub mod cards;
pub mod chips;
pub mod dialogs;
pub mod icon_buttons_fab;
pub mod lists;
pub mod navigation;
pub mod overlays;
pub mod overview;
pub mod selection;
pub mod slider_progress;
pub mod tabs;
pub mod text_fields;

use gpui::{AnyElement, App, Entity, FontWeight, IntoElement, Styled, div, prelude::*, px};

// 页面间回调类型（页 → 根）。
pub type PageCallback<A> = std::rc::Rc<dyn Fn(A, &mut App)>;

use material3_gpui::prelude::*;

/// 页面集合：根视图持有并按导航切换。
#[derive(Clone)]
pub struct Pages {
    pub overview: Entity<overview::OverviewPage>,
    pub buttons: Entity<buttons::ButtonsPage>,
    pub icon_buttons_fab: Entity<icon_buttons_fab::IconButtonsFabPage>,
    pub selection: Entity<selection::SelectionPage>,
    pub chips: Entity<chips::ChipsPage>,
    pub slider_progress: Entity<slider_progress::SliderProgressPage>,
    pub tabs: Entity<tabs::TabsPage>,
    pub text_fields: Entity<text_fields::TextFieldsPage>,
    pub overlays: Entity<overlays::OverlaysPage>,
    pub navigation: Entity<navigation::NavigationPage>,
    pub cards: Entity<cards::CardsPage>,
    pub lists: Entity<lists::ListsPage>,
    pub dialogs: Entity<dialogs::DialogsPage>,
}

impl Pages {
    /// 创建全部页面视图（组件实体在各页面构造函数中只创建一次）。
    pub fn new(cx: &mut App) -> Self {
        Self {
            overview: overview::OverviewPage::new(cx),
            buttons: buttons::ButtonsPage::new(cx),
            icon_buttons_fab: icon_buttons_fab::IconButtonsFabPage::new(cx),
            selection: selection::SelectionPage::new(cx),
            chips: chips::ChipsPage::new(cx),
            slider_progress: slider_progress::SliderProgressPage::new(cx),
            tabs: tabs::TabsPage::new(cx),
            text_fields: text_fields::TextFieldsPage::new(cx),
            overlays: overlays::OverlaysPage::new(cx),
            navigation: navigation::NavigationPage::new(cx),
            cards: cards::CardsPage::new(cx),
            lists: lists::ListsPage::new(cx),
            dialogs: dialogs::DialogsPage::new(cx),
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
pub(crate) fn demo_card(cx: &App, card: Card, title: &'static str) -> impl IntoElement {
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
