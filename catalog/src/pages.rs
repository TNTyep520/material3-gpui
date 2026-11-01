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
// 布局对齐 Compose Material Catalog(androidx):
// home(组件卡片网格)→ component(图标 + Description + Examples 列表)→
// example(顶栏 + 居中示例内容)。

#![allow(non_snake_case)]

pub mod Page_Badge;
pub mod Page_BottomAppBars;
pub mod Page_BottomSheet;
pub mod Page_ButtonGroups;
pub mod Page_Buttons;
pub mod Page_Card;
pub mod Page_Carousel;
pub mod Page_Checkboxes;
pub mod Page_Chips;
pub mod Page_DatePickers;
pub mod Page_Dialogs;
pub mod Page_ExtendedFab;
pub mod Page_Fab;
pub mod Page_FabMenu;
pub mod Page_FloatingToolbars;
pub mod Page_IconButtons;
pub mod Page_Lists;
pub mod Page_LoadingIndicators;
pub mod Page_Menus;
pub mod Page_NavigationBars;
pub mod Page_NavigationDrawers;
pub mod Page_NavigationRails;
pub mod Page_Progress;
pub mod Page_RadioButtons;
pub mod Page_Scrollbar;
pub mod Page_Search;
pub mod Page_SegmentedButtons;
pub mod Page_Sliders;
pub mod Page_Snackbars;
pub mod Page_SplitButtons;
pub mod Page_SwipeToDismiss;
pub mod Page_Switches;
pub mod Page_Tabs;
pub mod Page_TextFields;
pub mod Page_TimePickers;
pub mod Page_ToggleButtons;
pub mod Page_Tooltips;
pub mod Page_TopAppBars;
pub mod Page_Typography;

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

/// 示例元数据:名称与说明逐字取自 Compose Material Catalog 的 Examples.kt。
#[derive(Clone, Copy)]
pub(crate) struct ExampleMeta {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
}

/// 组件页契约:壳层负责顶栏/描述/示例列表,页面实体仅提供各示例的渲染。
pub(crate) trait ComponentPage {
    /// 组件官方描述(供壳层读取)。
    fn description(&self) -> &'static str;

    /// 本组件全部示例的元数据(顺序即组件页列表顺序,供壳层读取)。
    fn examples(&self) -> &'static [ExampleMeta];

    /// 渲染第 `index` 个示例的内容(Example 页的居中主体)。
    fn example(&self, index: usize, cx: &App) -> AnyElement;
}

/// 首页组件卡片与导航元数据,顺序对齐 Compose Material Catalog 的组件列表。
pub(crate) struct PageMeta {
    pub(crate) id: PageId,

    pub(crate) name: &'static str,

    /// 首页卡片图标;APK 中绝大多数组件使用默认 component 图标。
    pub(crate) icon: &'static str,

    /// 右上角附加徽标(如 "Unofficial");无徽标为空。
    pub(crate) additional_info: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageId {
    Badge,
    BottomAppBars,
    BottomSheet,
    Buttons,
    ButtonGroups,
    Card,
    Carousel,
    Checkboxes,
    Chips,
    DatePickers,
    Dialogs,
    ExtendedFab,
    Fab,
    FabMenu,
    FloatingToolbars,
    IconButtons,
    Lists,
    LoadingIndicators,
    Menus,
    NavigationBars,
    NavigationDrawers,
    NavigationRails,
    Progress,
    RadioButtons,
    Scrollbar,
    Search,
    SegmentedButtons,
    Sliders,
    Snackbars,
    SplitButtons,
    SwipeToDismiss,
    Switches,
    Tabs,
    TextFields,
    TimePickers,
    ToggleButtons,
    Tooltips,
    TopAppBars,
    Typography,
}

/// 库不支持的 APK 组件(Adaptive、Navigation Suite Scaffold、Pull-to-Refresh、
/// Material Shapes、Ripple、Surface)未列入。
pub(crate) const PAGES: [PageMeta; 39] = [
    PageMeta {
        id: PageId::Badge,
        name: "Badge",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::BottomAppBars,
        name: "Bottom App Bar",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::BottomSheet,
        name: "Bottom Sheet",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Buttons,
        name: "Buttons",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::ButtonGroups,
        name: "Button Groups",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Card,
        name: "Card",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Carousel,
        name: "Carousel",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Checkboxes,
        name: "Checkboxes",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Chips,
        name: "Chips",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::DatePickers,
        name: "Date pickers",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Dialogs,
        name: "Dialogs",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::ExtendedFab,
        name: "Extended FAB",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Fab,
        name: "Floating action buttons",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::FabMenu,
        name: "FAB Menu",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::FloatingToolbars,
        name: "Floating Toolbar",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::IconButtons,
        name: "Icon buttons",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Lists,
        name: "Lists",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::LoadingIndicators,
        name: "Loading indicators",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Menus,
        name: "Menus",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::NavigationBars,
        name: "Navigation bar",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::NavigationDrawers,
        name: "Navigation drawer",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::NavigationRails,
        name: "Navigation rail",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Progress,
        name: "Progress indicators",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::RadioButtons,
        name: "Radio buttons",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Search,
        name: "Search bars",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::SegmentedButtons,
        name: "Segmented Button",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Sliders,
        name: "Sliders",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Snackbars,
        name: "Snackbars",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::SplitButtons,
        name: "Split Button",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Switches,
        name: "Switches",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Tabs,
        name: "Tabs",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::TextFields,
        name: "Text fields",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::TimePickers,
        name: "Time Picker",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::ToggleButtons,
        name: "Toggle buttons",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Tooltips,
        name: "Tooltips",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::TopAppBars,
        name: "Top app bar",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Typography,
        name: "Typography",
        icon: "component",
        additional_info: "",
    },
    PageMeta {
        id: PageId::Scrollbar,
        name: "Scrollbar",
        icon: "component",
        additional_info: "Unofficial",
    },
    PageMeta {
        id: PageId::SwipeToDismiss,
        name: "Swipe to Dismiss",
        icon: "component",
        additional_info: "Unofficial",
    },
];

/// 页面实体集合:根视图持有,壳层按导航调用 `ComponentPage`。
#[derive(Clone)]
pub struct Pages {
    pub badge: Entity<Page_Badge::BadgePage>,
    pub bottom_app_bars: Entity<Page_BottomAppBars::BottomAppBarsPage>,
    pub bottom_sheet: Entity<Page_BottomSheet::BottomSheetPage>,
    pub buttons: Entity<Page_Buttons::ButtonsPage>,
    pub button_groups: Entity<Page_ButtonGroups::ButtonGroupsPage>,
    pub card: Entity<Page_Card::CardPage>,
    pub carousel: Entity<Page_Carousel::CarouselPage>,
    pub checkboxes: Entity<Page_Checkboxes::CheckboxesPage>,
    pub chips: Entity<Page_Chips::ChipsPage>,
    pub date_pickers: Entity<Page_DatePickers::DatePickersPage>,
    pub dialogs: Entity<Page_Dialogs::DialogsPage>,
    pub extended_fab: Entity<Page_ExtendedFab::ExtendedFabPage>,
    pub fab: Entity<Page_Fab::FabPage>,
    pub fab_menu: Entity<Page_FabMenu::FabMenuPage>,
    pub floating_toolbars: Entity<Page_FloatingToolbars::FloatingToolbarsPage>,
    pub icon_buttons: Entity<Page_IconButtons::IconButtonsPage>,
    pub lists: Entity<Page_Lists::ListsPage>,
    pub loading_indicators: Entity<Page_LoadingIndicators::LoadingIndicatorsPage>,
    pub menus: Entity<Page_Menus::MenusPage>,
    pub navigation_bars: Entity<Page_NavigationBars::NavigationBarsPage>,
    pub navigation_drawers: Entity<Page_NavigationDrawers::NavigationDrawersPage>,
    pub navigation_rails: Entity<Page_NavigationRails::NavigationRailsPage>,
    pub progress: Entity<Page_Progress::ProgressPage>,
    pub radio_buttons: Entity<Page_RadioButtons::RadioButtonsPage>,
    pub scrollbar: Entity<Page_Scrollbar::ScrollbarPage>,
    pub search: Entity<Page_Search::SearchPage>,
    pub segmented_buttons: Entity<Page_SegmentedButtons::SegmentedButtonsPage>,
    pub sliders: Entity<Page_Sliders::SlidersPage>,
    pub snackbars: Entity<Page_Snackbars::SnackbarsPage>,
    pub split_buttons: Entity<Page_SplitButtons::SplitButtonsPage>,
    pub swipe_to_dismiss: Entity<Page_SwipeToDismiss::SwipeToDismissPage>,
    pub switches: Entity<Page_Switches::SwitchesPage>,
    pub tabs: Entity<Page_Tabs::TabsPage>,
    pub text_fields: Entity<Page_TextFields::TextFieldsPage>,
    pub time_pickers: Entity<Page_TimePickers::TimePickersPage>,
    pub toggle_buttons: Entity<Page_ToggleButtons::ToggleButtonsPage>,
    pub tooltips: Entity<Page_Tooltips::TooltipsPage>,
    pub top_app_bars: Entity<Page_TopAppBars::TopAppBarsPage>,
    pub typography: Entity<Page_Typography::TypographyPage>,
}

impl Pages {
    /// 创建全部页面实体(组件实体在各页面构造函数中只创建一次)。
    #[allow(clippy::too_many_lines)]
    pub fn new(cx: &mut App) -> Self {
        Self {
            badge: Page_Badge::BadgePage::new(cx),
            bottom_app_bars: Page_BottomAppBars::BottomAppBarsPage::new(cx),
            bottom_sheet: Page_BottomSheet::BottomSheetPage::new(cx),
            buttons: Page_Buttons::ButtonsPage::new(cx),
            button_groups: Page_ButtonGroups::ButtonGroupsPage::new(cx),
            card: Page_Card::CardPage::new(cx),
            carousel: Page_Carousel::CarouselPage::new(cx),
            checkboxes: Page_Checkboxes::CheckboxesPage::new(cx),
            chips: Page_Chips::ChipsPage::new(cx),
            date_pickers: Page_DatePickers::DatePickersPage::new(cx),
            dialogs: Page_Dialogs::DialogsPage::new(cx),
            extended_fab: Page_ExtendedFab::ExtendedFabPage::new(cx),
            fab: Page_Fab::FabPage::new(cx),
            fab_menu: Page_FabMenu::FabMenuPage::new(cx),
            floating_toolbars: Page_FloatingToolbars::FloatingToolbarsPage::new(cx),
            icon_buttons: Page_IconButtons::IconButtonsPage::new(cx),
            lists: Page_Lists::ListsPage::new(cx),
            loading_indicators: Page_LoadingIndicators::LoadingIndicatorsPage::new(cx),
            menus: Page_Menus::MenusPage::new(cx),
            navigation_bars: Page_NavigationBars::NavigationBarsPage::new(cx),
            navigation_drawers: Page_NavigationDrawers::NavigationDrawersPage::new(cx),
            navigation_rails: Page_NavigationRails::NavigationRailsPage::new(cx),
            progress: Page_Progress::ProgressPage::new(cx),
            radio_buttons: Page_RadioButtons::RadioButtonsPage::new(cx),
            scrollbar: Page_Scrollbar::ScrollbarPage::new(cx),
            search: Page_Search::SearchPage::new(cx),
            segmented_buttons: Page_SegmentedButtons::SegmentedButtonsPage::new(cx),
            sliders: Page_Sliders::SlidersPage::new(cx),
            snackbars: Page_Snackbars::SnackbarsPage::new(cx),
            split_buttons: Page_SplitButtons::SplitButtonsPage::new(cx),
            swipe_to_dismiss: Page_SwipeToDismiss::SwipeToDismissPage::new(cx),
            switches: Page_Switches::SwitchesPage::new(cx),
            tabs: Page_Tabs::TabsPage::new(cx),
            text_fields: Page_TextFields::TextFieldsPage::new(cx),
            time_pickers: Page_TimePickers::TimePickersPage::new(cx),
            toggle_buttons: Page_ToggleButtons::ToggleButtonsPage::new(cx),
            tooltips: Page_Tooltips::TooltipsPage::new(cx),
            top_app_bars: Page_TopAppBars::TopAppBarsPage::new(cx),
            typography: Page_Typography::TypographyPage::new(cx),
        }
    }

    /// 按 `PageId` 取组件页的 trait 对象。
    pub fn page<'a>(&'a self, id: PageId, cx: &'a App) -> &'a dyn ComponentPage {
        match id {
            PageId::Badge => self.badge.read(cx),
            PageId::BottomAppBars => self.bottom_app_bars.read(cx),
            PageId::BottomSheet => self.bottom_sheet.read(cx),
            PageId::Buttons => self.buttons.read(cx),
            PageId::ButtonGroups => self.button_groups.read(cx),
            PageId::Card => self.card.read(cx),
            PageId::Carousel => self.carousel.read(cx),
            PageId::Checkboxes => self.checkboxes.read(cx),
            PageId::Chips => self.chips.read(cx),
            PageId::DatePickers => self.date_pickers.read(cx),
            PageId::Dialogs => self.dialogs.read(cx),
            PageId::ExtendedFab => self.extended_fab.read(cx),
            PageId::Fab => self.fab.read(cx),
            PageId::FabMenu => self.fab_menu.read(cx),
            PageId::FloatingToolbars => self.floating_toolbars.read(cx),
            PageId::IconButtons => self.icon_buttons.read(cx),
            PageId::Lists => self.lists.read(cx),
            PageId::LoadingIndicators => self.loading_indicators.read(cx),
            PageId::Menus => self.menus.read(cx),
            PageId::NavigationBars => self.navigation_bars.read(cx),
            PageId::NavigationDrawers => self.navigation_drawers.read(cx),
            PageId::NavigationRails => self.navigation_rails.read(cx),
            PageId::Progress => self.progress.read(cx),
            PageId::RadioButtons => self.radio_buttons.read(cx),
            PageId::Scrollbar => self.scrollbar.read(cx),
            PageId::Search => self.search.read(cx),
            PageId::SegmentedButtons => self.segmented_buttons.read(cx),
            PageId::Sliders => self.sliders.read(cx),
            PageId::Snackbars => self.snackbars.read(cx),
            PageId::SplitButtons => self.split_buttons.read(cx),
            PageId::SwipeToDismiss => self.swipe_to_dismiss.read(cx),
            PageId::Switches => self.switches.read(cx),
            PageId::Tabs => self.tabs.read(cx),
            PageId::TextFields => self.text_fields.read(cx),
            PageId::TimePickers => self.time_pickers.read(cx),
            PageId::ToggleButtons => self.toggle_buttons.read(cx),
            PageId::Tooltips => self.tooltips.read(cx),
            PageId::TopAppBars => self.top_app_bars.read(cx),
            PageId::Typography => self.typography.read(cx),
        }
    }
}

/// 示例内容包装:Example 页的居中主体(纵向堆叠,行间距 12dp)。
pub(crate) fn example_content(rows: impl IntoIterator<Item: IntoElement>) -> AnyElement {
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(px(12.))
        .children(rows)
        .into_any_element()
}

/// 示例内容中的一行:左侧说明(label_medium)+ 右侧控件,便于对照各变体。
pub(crate) fn example_row(
    cx: &App,
    label: &'static str,
    content: impl IntoIterator<Item: IntoElement>,
) -> AnyElement {
    let theme = cx.theme();
    div()
        .w_full()
        .min_w_0()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(16.))
        .child(
            theme
                .typography()
                .label_medium
                .apply(div())
                .w(px(200.))
                .flex_none()
                .min_w_0()
                .text_color(theme.colors().on_surface_variant)
                .child(label),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(12.))
                .children(content),
        )
        .into_any_element()
}

/// 为不同 Card 变体提供相同的内容和尺寸,以便直接比较表面样式。
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
