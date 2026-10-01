use crate::components::{Button, ButtonVariant, IconButton, IconButtonVariant};
use crate::icon::IconName;
use crate::theme::{ActiveTheme, TokenSet};
use crate::tokens::SplitButtonSmallTokens;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, Pixels, RenderOnce,
    SharedString, Window, div, prelude::*,
};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// 分裂按钮样式(由令牌推导)。
#[derive(Clone, Copy, Debug)]
pub struct SplitButtonStyle {
    /// 主按钮与尾部图标按钮之间的间距。
    pub between_space: Pixels,
}

impl SplitButtonStyle {
    /// 由令牌推导默认样式(小尺寸分裂按钮,间距 2dp)。
    pub fn resolve(_tokens: &TokenSet) -> Self {
        Self {
            between_space: SplitButtonSmallTokens::BETWEEN_SPACE.pixels(),
        }
    }
}

/// MD3 分裂按钮:主按钮 + 尾部下拉图标按钮。
#[derive(IntoElement)]
pub struct SplitButton {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    trailing: IconName,
    on_click: Option<ClickHandler>,
    on_trailing_click: Option<ClickHandler>,
    menu: Option<AnyElement>,
}

/// AndroidX SplitButtonLayout 对应的双操作按钮布局。
pub type SplitButtonLayout = SplitButton;

impl SplitButton {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Filled,
            trailing: IconName::ChevronRight,
            on_click: None,
            on_trailing_click: None,
            menu: None,
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn filled(self) -> Self {
        self.variant(ButtonVariant::Filled)
    }
    pub fn outlined(self) -> Self {
        self.variant(ButtonVariant::Outlined)
    }
    pub fn tonal(self) -> Self {
        self.variant(ButtonVariant::FilledTonal)
    }
    pub fn trailing_icon(mut self, icon: IconName) -> Self {
        self.trailing = icon;
        self
    }
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// 设置尾部按钮的独立点击回调。
    pub fn on_trailing_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_trailing_click = Some(Box::new(handler));
        self
    }
    pub fn menu(mut self, menu: impl IntoElement) -> Self {
        self.menu = Some(menu.into_any_element());
        self
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style = SplitButtonStyle::resolve(cx.theme().token_set());
        let mut primary =
            Button::new((self.id.clone(), "primary"), self.label).variant(self.variant);
        if let Some(handler) = self.on_click {
            primary = primary.on_click(handler);
        }
        let primary = primary.build(cx);
        let mut trailing =
            IconButton::new((self.id, "menu"), self.trailing).variant(IconButtonVariant::Standard);
        if let Some(handler) = self.on_trailing_click {
            trailing = trailing.on_click(handler);
        }
        let trailing = trailing.build(cx);
        div()
            .flex()
            .items_center()
            .gap(style.between_space)
            .child(primary)
            .child(trailing)
            .when_some(self.menu, |el, menu| el.child(menu))
    }
}
