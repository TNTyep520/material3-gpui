//! Lists 页：带前后缀的列表项及分隔线展示。

use gpui::{AnyElement, App, Entity, IntoElement, Render, Window, div, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// 列表数据:(标题, 副文, 主图标, 尾部字符)。
const ENTRIES: [(&str, &str, IconName, Option<&str>); 5] = [
    ("Proofs batch", "Jan 9, 2026", IconName::Star, Some("3")),
    (
        "Field recordings",
        "Updated yesterday",
        IconName::Favorite,
        Some("12"),
    ),
    ("Riso calendar", "Drafting", IconName::Edit, None),
    (
        "Cutting mats",
        "Restock requested",
        IconName::Settings,
        None,
    ),
    ("Archive", "Last opened in May", IconName::Delete, None),
];

/// Lists 页视图。
pub struct ListsPage;

impl ListsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

/// 单个列表行(一条数据 → ListItem)。
fn inbox_row(
    ix: usize,
    title: &'static str,
    supporting: &'static str,
    icon: IconName,
    trailing: Option<&'static str>,
) -> AnyElement {
    let mut item = ListItem::new(("studio-inbox", ix), title)
        .supporting_text(supporting)
        .leading_icon(icon)
        .on_click(move |_, window, cx| {
            show_snackbar(
                window,
                cx,
                Snackbar::new(format!("Opened \"{title}\"")),
                None,
            );
        });
    if let Some(text) = trailing {
        item = item.trailing_text(text);
    } else {
        item = item.trailing_icon(IconName::ChevronRight);
    }
    div().child(item).into_any_element()
}

impl Render for ListsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let rows = ENTRIES
            .iter()
            .enumerate()
            .flat_map(|(ix, (title, supporting, icon, trailing))| {
                let row = inbox_row(ix, title, supporting, *icon, *trailing);
                if ix + 1 < ENTRIES.len() {
                    vec![row, Divider::horizontal().inset().into_any_element()]
                } else {
                    vec![row]
                }
            })
            .collect::<Vec<_>>();

        gallery([showcase_group(
            cx,
            "List items & dividers",
            [div()
                .w_full()
                .min_w_0()
                .child(List::new().children(rows))
                .into_any_element()],
        )])
    }
}
