//! 主页面:BakaXL 设置页风格的组件导航(分组标题 + 设置卡片)。
//!
//! 竖屏铺满整页,横屏作为左栏;点击卡片回调 `on_navigate` 打开对应
//! 组件页。全部使用 md3 组件(Card / ListItem / Icon)。

use gpui::{AnyElement, App, ParentElement as _, Styled, div, prelude::*, px};
use material3_gpui::icon::{Icon, IconName};
use material3_gpui::prelude::ActiveTheme;

use crate::PAGES;
use crate::pages::PageCallback;

/// 分组:(组标题, `PAGES` 下标列表)。
const GROUPS: &[(&str, &[usize])] = &[
    ("Actions", &[0, 1]),
    ("Selection", &[2, 3]),
    ("Input", &[6, 4]),
    ("Navigation", &[5, 8]),
    ("App shell", &[12, 13]),
    ("Containers", &[9, 10]),
    ("Feedback", &[7, 11]),
];

/// 主页面板:页头(应用标题 + 副标题) + 分组设置卡片列表(可滚动)。
///
/// `selected` 为当前详情页在 `PAGES` 中的下标(选中卡片显示指示条);
/// 点击卡片经 `on_navigate` 回调根视图。
pub(crate) fn pane(
    cx: &App,
    selected: Option<usize>,
    on_navigate: PageCallback<usize>,
) -> AnyElement {
    let theme = cx.theme();
    let colors = *theme.colors();
    let typography = *theme.typography();

    // 页头:应用名 + 副标题
    let header = div()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(2.))
        .child(
            typography
                .title_large
                .apply(div())
                .text_color(colors.on_surface)
                .child("Material3 Catalog"),
        )
        .child(
            typography
                .body_small
                .apply(div())
                .text_color(colors.on_surface_variant)
                .child("Material Design 3 component demo"),
        );

    // 分组卡片列表
    let mut sections: Vec<AnyElement> = Vec::new();
    for (group_title, indices) in GROUPS {
        let mut cards: Vec<AnyElement> = Vec::new();
        for &ix in *indices {
            let meta = &PAGES[ix];
            let is_selected = selected == Some(ix);
            let card = material3_gpui::Card::new()
                .outlined()
                .w_full()
                .relative()
                .overflow_hidden()
                .when(is_selected, |el| {
                    el.child(
                        div()
                            .absolute()
                            .left(px(0.))
                            .top(px(18.))
                            .w(px(4.))
                            .h(px(36.))
                            .rounded_full()
                            .bg(colors.primary),
                    )
                })
                .child(
                    material3_gpui::ListItem::new(("home", ix), meta.title)
                        .supporting_text(meta.subtitle)
                        .leading_icon(meta.icon)
                        .trailing(
                            Icon::new(IconName::ChevronRight)
                                .size(px(24.))
                                .color(colors.on_surface_variant),
                        )
                        .on_click({
                            let on_navigate = on_navigate.clone();
                            move |_, _w, cx| on_navigate(ix, cx)
                        }),
                );
            cards.push(card.into_any_element());
        }
        sections.push(
            div()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    div()
                        .text_size(px(16.))
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(colors.on_surface)
                        .child(*group_title),
                )
                .children(cards)
                .into_any_element(),
        );
    }

    div()
        .id("home-pane")
        .flex_1()
        .min_h_0()
        .flex()
        .flex_col()
        .gap(px(24.))
        .px(px(20.))
        .pt(px(16.))
        .pb(px(20.))
        .child(header)
        .child(
            div()
                .id("home-scroll")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .gap(px(24.))
                .children(sections),
        )
        .into_any_element()
}
