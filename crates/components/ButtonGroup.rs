use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, Window, div,
    prelude::*,
};

use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::ButtonGroupSmallTokens;

/// 按钮组样式(由令牌推导,可用 [`ButtonGroup::spacing`] 覆盖间距)。
#[derive(Clone, Copy, Debug)]
pub struct ButtonGroupStyle {
    /// 按钮之间的间距。
    pub spacing: Pixels,
}

impl ButtonGroupStyle {
    /// 由令牌推导默认样式(小尺寸按钮组,组间距 12dp)。
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            spacing: ButtonGroupSmallTokens::BETWEEN_SPACE.pixels(),
        }
    }
}

/// A horizontal Material button group with a shared touch target.
#[derive(IntoElement)]
pub struct ButtonGroup {
    id: ElementId,
    children: Vec<AnyElement>,
    vertical: bool,
    spacing: Option<Pixels>,
    expanded_ratio: f32,
    overflow_indicator: Option<AnyElement>,
}

/// AndroidX ButtonGroupMenuState 对应的溢出菜单可见状态。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonGroupMenuState {
    is_showing: bool,
}

impl ButtonGroupMenuState {
    /// 创建初始显示或隐藏的菜单状态。
    pub fn new(initial_is_showing: bool) -> Self {
        Self {
            is_showing: initial_is_showing,
        }
    }

    /// 返回菜单是否显示。
    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    /// 显示溢出菜单。
    pub fn show(&mut self) {
        self.is_showing = true;
    }

    /// 收起溢出菜单。
    pub fn dismiss(&mut self) {
        self.is_showing = false;
    }
}

impl ButtonGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: Vec::new(),
            vertical: false,
            spacing: None,
            expanded_ratio: 1.,
            overflow_indicator: None,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    /// 覆盖组间距(默认取 [`ButtonGroupSmallTokens::BETWEEN_SPACE`])。
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing = Some(spacing);
        self
    }

    /// 设置按钮间距相对默认值的展开比例。
    pub fn expanded_ratio(mut self, ratio: f32) -> Self {
        self.expanded_ratio = ratio.max(0.);
        self
    }

    /// 添加溢出菜单触发控件；仅在调用者提供时显示。
    pub fn overflow_indicator(mut self, indicator: impl IntoElement) -> Self {
        self.overflow_indicator = Some(indicator.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = ButtonGroupStyle::resolve(cx.theme().token_set());
        div()
            .id(self.id)
            .flex()
            .when(self.vertical, |el| el.flex_col())
            .gap(self.spacing.unwrap_or(style.spacing) * self.expanded_ratio)
            .children(self.children)
            .when_some(self.overflow_indicator, |el, indicator| el.child(indicator))
    }
}
