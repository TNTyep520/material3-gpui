// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 参考 https://github.com/Glavo/m3fx/blob/main/demo/src/main/java/org/glavo/m3fx/demo/DemoPageSupport.java

#![allow(non_snake_case)]

pub mod Page_AppBars;
pub mod Page_Buttons;
pub mod Page_ButtonsExtended;
pub mod Page_Cards;
pub mod Page_Carousel;
pub mod Page_Chips;
pub mod Page_Dialogs;
pub mod Page_FabMenu;
pub mod Page_IconButtonsFab;
pub mod Page_Lists;
pub mod Page_LoadingIndicators;
pub mod Page_Motion;
pub mod Page_Navigation;
pub mod Page_Overlays;
pub mod Page_Pickers;
pub mod Page_Progress;
pub mod Page_Search;
pub mod Page_Selection;
pub mod Page_Sheets;
pub mod Page_Sliders;
pub mod Page_Tabs;
pub mod Page_TextFields;
pub mod Page_Toolbars;

use gpui::{AnyElement, App, Entity, IntoElement, Styled, div, prelude::*, px};
use material3_gpui::prelude::*;

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

/// 页面集合：根视图持有并按导航切换。
#[derive(Clone)]
pub struct Pages {
    pub app_bars: Entity<Page_AppBars::AppBarsPage>,
    pub buttons: Entity<Page_Buttons::ButtonsPage>,
    pub buttons_extended: Entity<Page_ButtonsExtended::ButtonsExtendedPage>,
    pub cards: Entity<Page_Cards::CardsPage>,
    pub carousel: Entity<Page_Carousel::CarouselPage>,
    pub chips: Entity<Page_Chips::ChipsPage>,
    pub dialogs: Entity<Page_Dialogs::DialogsPage>,
    pub fab_menu: Entity<Page_FabMenu::FabMenuPage>,
    pub icon_buttons_fab: Entity<Page_IconButtonsFab::IconButtonsFabPage>,
    pub lists: Entity<Page_Lists::ListsPage>,
    pub loading_indicators: Entity<Page_LoadingIndicators::LoadingIndicatorsPage>,
    pub motion: Entity<Page_Motion::MotionPage>,
    pub navigation: Entity<Page_Navigation::NavigationPage>,
    pub overlays: Entity<Page_Overlays::OverlaysPage>,
    pub pickers: Entity<Page_Pickers::PickersPage>,
    pub progress: Entity<Page_Progress::ProgressPage>,
    pub search: Entity<Page_Search::SearchPage>,
    pub selection: Entity<Page_Selection::SelectionPage>,
    pub sheets: Entity<Page_Sheets::SheetsPage>,
    pub sliders: Entity<Page_Sliders::SlidersPage>,
    pub tabs: Entity<Page_Tabs::TabsPage>,
    pub text_fields: Entity<Page_TextFields::TextFieldsPage>,
    pub toolbars: Entity<Page_Toolbars::ToolbarsPage>,
}

impl Pages {
    /// 创建全部页面视图（组件实体在各页面构造函数中只创建一次）。
    pub fn new(cx: &mut App) -> Self {
        Self {
            app_bars: Page_AppBars::AppBarsPage::new(cx),
            buttons: Page_Buttons::ButtonsPage::new(cx),
            buttons_extended: Page_ButtonsExtended::ButtonsExtendedPage::new(cx),
            cards: Page_Cards::CardsPage::new(cx),
            carousel: Page_Carousel::CarouselPage::new(cx),
            chips: Page_Chips::ChipsPage::new(cx),
            dialogs: Page_Dialogs::DialogsPage::new(cx),
            fab_menu: Page_FabMenu::FabMenuPage::new(cx),
            icon_buttons_fab: Page_IconButtonsFab::IconButtonsFabPage::new(cx),
            lists: Page_Lists::ListsPage::new(cx),
            loading_indicators: Page_LoadingIndicators::LoadingIndicatorsPage::new(cx),
            motion: Page_Motion::MotionPage::new(cx),
            navigation: Page_Navigation::NavigationPage::new(cx),
            overlays: Page_Overlays::OverlaysPage::new(cx),
            pickers: Page_Pickers::PickersPage::new(cx),
            progress: Page_Progress::ProgressPage::new(cx),
            search: Page_Search::SearchPage::new(cx),
            selection: Page_Selection::SelectionPage::new(cx),
            sheets: Page_Sheets::SheetsPage::new(cx),
            sliders: Page_Sliders::SlidersPage::new(cx),
            tabs: Page_Tabs::TabsPage::new(cx),
            text_fields: Page_TextFields::TextFieldsPage::new(cx),
            toolbars: Page_Toolbars::ToolbarsPage::new(cx),
        }
    }
}

/// 页面骨架：m3fx demo 布局（大标题 + 副标题 + 分组流）。
pub(crate) fn page(
    cx: &App,
    title: &'static str,
    subtitle: &'static str,
    groups: impl IntoIterator<Item: IntoElement>,
) -> impl IntoElement {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(24.))
        .child(page_header(cx, title, subtitle))
        .child(gallery(groups))
}

/// m3fx DemoPageHeader：页标题 + on_surface_variant 副标题。
pub(crate) fn page_header(cx: &App, title: &'static str, subtitle: &'static str) -> AnyElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            theme
                .typography()
                .headline_small
                .apply(div())
                .text_color(theme.colors().on_surface)
                .child(title),
        )
        .child(
            theme
                .typography()
                .body_medium
                .apply(div())
                .text_color(theme.colors().on_surface_variant)
                .child(subtitle),
        )
        .into_any_element()
}

/// 垂直排列组件分组；组间距对齐 m3fx 的 18dp。
pub(crate) fn gallery(groups: impl IntoIterator<Item: IntoElement>) -> impl IntoElement {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(18.))
        .children(groups.into_iter().map(|group| group.into_any_element()))
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

/// m3fx createShowcaseGroup：组标题(label_large) + surface_container_low
/// 圆角容器(12dp)内 16dp 间距流式排布。
pub(crate) fn showcase_group(
    cx: &App,
    title: &'static str,
    items: impl IntoIterator<Item: IntoElement>,
) -> AnyElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(
            theme
                .typography()
                .label_large
                .apply(div())
                .text_color(theme.colors().on_surface)
                .child(title),
        )
        .child(
            div()
                .w_full()
                .min_w_0()
                .p(px(18.))
                .rounded(px(12.))
                .border_1()
                .border_color(theme.colors().outline_variant)
                .bg(theme.colors().surface_container_low)
                .flex()
                .flex_wrap()
                .items_start()
                .gap(px(16.))
                .children(items),
        )
        .into_any_element()
}

/// m3fx createFullWidthShowcaseGroup：组内容纵向堆叠（整宽控件行）。
pub(crate) fn full_width_group(
    cx: &App,
    title: &'static str,
    rows: impl IntoIterator<Item: IntoElement>,
) -> AnyElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(
            theme
                .typography()
                .label_large
                .apply(div())
                .text_color(theme.colors().on_surface)
                .child(title),
        )
        .child(
            div()
                .w_full()
                .min_w_0()
                .p(px(18.))
                .rounded(px(12.))
                .border_1()
                .border_color(theme.colors().outline_variant)
                .bg(theme.colors().surface_container_low)
                .flex()
                .flex_col()
                .gap(px(16.))
                .children(rows),
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
