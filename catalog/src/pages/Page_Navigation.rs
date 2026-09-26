//! Navigation 页：图标预览 + TopAppBar / NavigationBar / Rail / Drawer。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Navigation 页视图。
pub struct NavigationPage {
    nav_bar: Entity<NavigationBarState>,
    nav_rail: Entity<NavigationRailState>,
    nav_drawer: Entity<NavigationDrawerState>,
}

impl NavigationPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let rail_fab = Fab::new("rail-fab", IconName::Add)
            .size(FabSize::Small)
            .build(cx);
        let nav_bar = NavigationBar::new("nav-bar")
            .items([
                NavigationItemSpec::new("Home", IconName::Home),
                NavigationItemSpec::new("Search", IconName::Search),
                NavigationItemSpec::new("Profile", IconName::Person),
                NavigationItemSpec::new("Settings", IconName::Settings),
            ])
            .selected(0)
            .build(cx);
        let nav_rail = NavigationRail::new("nav-rail")
            .header(rail_fab)
            .items([
                NavigationItemSpec::new("Inbox", IconName::Info),
                NavigationItemSpec::new("Starred", IconName::Star),
                NavigationItemSpec::new("Sent", IconName::Edit),
            ])
            .selected(0)
            .build(cx);
        let nav_drawer = NavigationDrawer::new("nav-drawer")
            .item(NavigationItemSpec::new("Inbox", IconName::Info).badge("24"))
            .item(NavigationItemSpec::new("Starred", IconName::Star))
            .section("Labels")
            .item(NavigationItemSpec::new("Work", IconName::Edit))
            .item(NavigationItemSpec::new("Personal", IconName::Person))
            .selected(0)
            .build(cx);

        cx.new(|_| Self {
            nav_bar,
            nav_rail,
            nav_drawer,
        })
    }
}

/// 字体字形图标预览条。
fn icon_strip(cx: &App) -> impl IntoElement {
    let color = cx.theme().colors().on_surface_variant;
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(16.))
        .children(
            [
                IconName::Home,
                IconName::Search,
                IconName::Settings,
                IconName::Favorite,
                IconName::Star,
                IconName::Person,
                IconName::Edit,
                IconName::Delete,
                IconName::Info,
                IconName::Menu,
                IconName::MoreVert,
                IconName::Check,
                IconName::Close,
                IconName::Add,
                IconName::ArrowBack,
                IconName::ChevronRight,
            ]
            .into_iter()
            .map(|name| Icon::new(name).size(px(24.)).color(color)),
        )
}

impl Render for NavigationPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gallery([
            showcase_group(cx, "Icons", [icon_strip(cx).into_any_element()]),
            showcase_group(
                cx,
                "Top App Bar",
                [div()
                    .w_full()
                    .child(
                        material3_gpui::TopAppBar::new("Navigation").leading(
                            material3_gpui::Icon::new(IconName::Menu)
                                .size(px(24.))
                                .color(cx.theme().colors().on_surface),
                        ),
                    )
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Navigation Bar",
                [div()
                    .w_full()
                    .child(self.nav_bar.clone())
                    .into_any_element()],
            ),
            showcase_group(
                cx,
                "Navigation Rail & Drawer",
                [div()
                    .flex()
                    .flex_wrap()
                    .gap(px(16.))
                    .min_h(px(320.))
                    .child(self.nav_rail.clone())
                    .child(
                        material3_gpui::Card::new()
                            .outlined()
                            .overflow_hidden()
                            .w(px(360.))
                            .max_w_full()
                            .child(self.nav_drawer.clone()),
                    )
                    .into_any_element()],
            ),
        ])
    }
}
