use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Window, div,
    prelude::*, px,
};
use std::rc::Rc;

type ExpandedChangeHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::OutlinedTextFieldTokens;

/// 暴露下拉框样式(由令牌推导)。
#[derive(Clone, Copy, Debug)]
pub struct ExposedDropdownMenuStyle {
    /// 下拉菜单相对字段顶部向下的偏移(字段容器高度)。
    pub menu_top_offset: Pixels,
    /// 展开时为菜单预留的底部空间。
    pub menu_reserved_height: Pixels,
}

impl ExposedDropdownMenuStyle {
    /// 由令牌推导默认样式(字段高度取 outlined 文本框令牌)。
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            menu_top_offset: OutlinedTextFieldTokens::CONTAINER_HEIGHT.pixels(),
            // 展开预留空间:菜单最大高度约束(gpui 下由外层滚动容器承接溢出)
            menu_reserved_height: px(220.),
        }
    }
}

/// MD3 暴露下拉框:字段 + 展开时悬浮其下的菜单容器。
#[derive(IntoElement)]
pub struct ExposedDropdownMenu {
    id: ElementId,
    field: AnyElement,
    menu: Option<AnyElement>,
    expanded: bool,
    on_expanded_change: Option<ExpandedChangeHandler>,
}

/// AndroidX ExposedDropdownMenuBox 对应的锚点与菜单容器。
pub type ExposedDropdownMenuBox = ExposedDropdownMenu;

impl ExposedDropdownMenu {
    pub fn new(id: impl Into<ElementId>, field: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            field: field.into_any_element(),
            menu: None,
            expanded: false,
            on_expanded_change: None,
        }
    }
    /// 展开时悬浮在字段下方的菜单内容。
    pub fn menu(mut self, menu: impl IntoElement) -> Self {
        self.menu = Some(menu.into_any_element());
        self
    }
    /// 展开状态。
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// 点击锚点时请求切换展开状态。
    pub fn on_expanded_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_expanded_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ExposedDropdownMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = ExposedDropdownMenuStyle::resolve(cx.theme().token_set());
        let expanded = self.expanded;
        let anchor = div()
            .id((self.id.clone(), "anchor"))
            .child(self.field)
            .when_some(self.on_expanded_change, |el, handler| {
                el.cursor_pointer()
                    .on_click(move |_, window, cx| handler(!expanded, window, cx))
            });
        div()
            .id(self.id)
            .relative()
            .w_full()
            .when(expanded, |el| el.pb(style.menu_reserved_height))
            .child(anchor)
            .when(expanded, |el| {
                el.when_some(self.menu, |el, menu| {
                    el.child(
                        div()
                            .absolute()
                            .top(style.menu_top_offset)
                            .left_0()
                            .right_0()
                            .child(menu),
                    )
                })
            })
    }
}
