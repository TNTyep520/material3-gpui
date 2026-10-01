//! catalog 组件展厅：每页使用独立 Entity，组件状态变化只刷新对应页面。
//! 共用分组表面和带标签的预览单元统一变体与状态展示。
// 页面文件按 `Page_<Name>` 命名(多词用驼峰式),非 snake_case
#![allow(non_snake_case)]

pub mod Page_Additional;
pub mod Page_AppBars;
pub mod Page_Buttons;
pub mod Page_ButtonsExtended;
pub mod Page_Cards;
pub mod Page_Chips;
pub mod Page_Dialogs;
pub mod Page_IconButtonsFab;
pub mod Page_Lists;
pub mod Page_Navigation;
pub mod Page_Overlays;
pub mod Page_Selection;
pub mod Page_Sheets;
pub mod Page_SliderProgress;
pub mod Page_Tabs;
pub mod Page_TextFields;

use gpui::{AnyElement, App, Entity, IntoElement, Styled, div, prelude::*, px};

// 页面间回调类型（页 → 根）。
pub type PageCallback<A> = std::rc::Rc<dyn Fn(A, &mut App)>;

/// 实体更新失败的兜底(窗口/页面已释放等场景):输出到 stderr 后放行。
pub(crate) trait LogErr<T, E: std::fmt::Display> {
    fn log_err(self) -> Option<T>;
}

impl<T, E: std::fmt::Display> LogErr<T, E> for Result<T, E> {
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                eprintln!("catalog: {err}");
                None
            }
        }
    }
}

use material3_gpui::prelude::*;

/// 页面集合：根视图持有并按导航切换。
#[derive(Clone)]
pub struct Pages {
    pub additional: Entity<Page_Additional::AdditionalPage>,
    pub buttons: Entity<Page_Buttons::ButtonsPage>,
    pub buttons_extended: Entity<Page_ButtonsExtended::ButtonsExtendedPage>,
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
    pub app_bars: Entity<Page_AppBars::AppBarsPage>,
    pub sheets: Entity<Page_Sheets::SheetsPage>,
}

impl Pages {
    /// 创建全部页面视图（组件实体在各页面构造函数中只创建一次）。
    pub fn new(cx: &mut App) -> Self {
        Self {
            additional: Page_Additional::AdditionalPage::new(cx),
            buttons: Page_Buttons::ButtonsPage::new(cx),
            buttons_extended: Page_ButtonsExtended::ButtonsExtendedPage::new(cx),
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
            app_bars: Page_AppBars::AppBarsPage::new(cx),
            sheets: Page_Sheets::SheetsPage::new(cx),
        }
    }
}

/// 垂直排列组件分组，宽度由页面容器约束。
pub(crate) fn gallery(groups: impl IntoIterator<Item = AnyElement>) -> impl IntoElement {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(24.))
        .children(groups)
}

/// 核心主题色的紧凑预览，颜色随主题更新。
pub(crate) fn palette_strip(cx: &App) -> impl IntoElement {
    let colors = *cx.theme().colors();
    div()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(6.))
        .children(
            [
                colors.primary,
                colors.secondary,
                colors.tertiary,
                colors.error,
                colors.surface_variant,
            ]
            .map(|color| div().size(px(14.)).rounded_full().bg(color)),
        )
}

/// 带标题的组件展示区域；内容按行换行，保留组件自己的表面样式。
pub(crate) fn showcase_group(
    cx: &App,
    title: &'static str,
    items: impl IntoIterator<Item = AnyElement>,
) -> AnyElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            theme
                .typography()
                .title_medium
                .apply(div())
                .text_color(theme.colors().on_surface)
                .child(title),
        )
        .child(
            div()
                .w_full()
                .min_w_0()
                .p(px(20.))
                .rounded(px(16.))
                .border_1()
                .border_color(theme.colors().outline_variant)
                .bg(theme.colors().surface_container_low)
                .flex()
                .flex_wrap()
                .items_start()
                .gap(px(20.))
                .children(items),
        )
        .into_any_element()
}

/// 固定最小展示高度并附加标签，便于比较同组组件的变体和状态。
pub(crate) fn specimen(cx: &App, label: &'static str, element: impl IntoElement) -> AnyElement {
    let theme = cx.theme();
    div()
        .w(px(176.))
        .max_w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(
            div()
                .w_full()
                .min_h(px(64.))
                .flex()
                .items_center()
                .child(element),
        )
        .child(
            theme
                .typography()
                .label_medium
                .apply(div())
                .text_color(theme.colors().on_surface_variant)
                .child(label),
        )
        .into_any_element()
}
/// 为不同 Card 变体提供相同的内容和尺寸，以便直接比较表面样式。
pub(crate) fn catalog_card(cx: &App, card: Card, title: &'static str) -> impl IntoElement {
    let theme = cx.theme();
    card.w(px(220.))
        .max_w_full()
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
