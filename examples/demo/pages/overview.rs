//! Components overview 页：全部页面的导航列表。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, prelude::*, px};
use md3_gpui::prelude::*;

use super::{gallery, showcase_group};
use crate::PAGES;

/// Overview 页视图。
pub struct OverviewPage {
    /// 导航回调（参数为目标页在 `PAGES` 中的下标）。
    pub(crate) on_navigate: Option<super::PageCallback<usize>>,
}

impl OverviewPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self { on_navigate: None })
    }

    /// 设置导航回调（根视图首帧接线）。
    pub fn set_on_navigate(&mut self, handler: super::PageCallback<usize>) {
        self.on_navigate = Some(handler);
    }
}

impl Render for OverviewPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let on_navigate = self.on_navigate.clone();

        gallery([showcase_group(
            cx,
            "Material Components",
            [md3_gpui::Card::new()
                .outlined()
                .overflow_hidden()
                .w_full()
                .child(
                    md3_gpui::List::new().children(PAGES.iter().enumerate().skip(1).map(
                        |(ix, meta)| {
                            let on_navigate = on_navigate.clone();
                            md3_gpui::ListItem::new(("overview", ix), meta.title)
                                .supporting_text(meta.subtitle)
                                .leading_icon(meta.icon)
                                .trailing(
                                    Icon::new(IconName::ChevronRight)
                                        .size(px(24.))
                                        .color(colors.on_surface_variant),
                                )
                                .on_click(move |_, _w, cx| {
                                    if let Some(on_navigate) = on_navigate.clone() {
                                        on_navigate(ix, cx);
                                    }
                                })
                        },
                    )),
                )
                .into_any_element()],
        )])
    }
}
