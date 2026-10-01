//! Bottom sheet 页「Sharing」:ModalBottomSheet + DragHandle,动作行直达列表。

use gpui::{
    App, AppContext as _, Entity, IntoElement, ParentElement as _, Render, Styled, Window, div,
    prelude::*, px,
};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

type DismissHandler = std::rc::Rc<dyn Fn(&mut App)>;

/// Bottom sheet 页视图。
pub struct SheetsPage {
    pub(crate) open_button: Entity<ButtonState>,
    pub(crate) sheet_open: bool,
    on_dismiss: Option<DismissHandler>,
}

impl SheetsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        let open_button = Button::new("open-sheet-btn", "Share this page")
            .filled()
            .build(cx);
        cx.new(|_| Self {
            open_button,
            sheet_open: false,
            on_dismiss: None,
        })
    }

    /// 首帧接线:注入 scrim 点击回调(回调内关闭弹层)。
    pub fn set_on_dismiss(&mut self, on_dismiss: impl Fn(&mut App) + 'static) {
        self.on_dismiss = Some(std::rc::Rc::new(on_dismiss));
    }
}

/// 单条分享动作的数据(标题, 图标)。
const SHARE_ACTIONS: [(&str, IconName); 4] = [
    ("Send to proof readers", IconName::Menu),
    ("Copy poster link", IconName::Info),
    ("Edit title", IconName::Edit),
    ("Delete", IconName::Delete),
];

impl Render for SheetsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let sheet_open = self.sheet_open;
        let on_dismiss = self.on_dismiss.clone();

        gallery(
            [showcase_group(
                cx,
                "Modal bottom sheet",
                [div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .child(div().text_color(colors.on_surface_variant).child(
                        "Tap the button to open a modal bottom sheet with a drag \
                             handle. Tap the scrim to dismiss.",
                    ))
                    .child(self.open_button.clone())
                    .into_any_element()],
            )]
            .into_iter()
            .chain(std::iter::once(
                div()
                    .when(sheet_open, |el| {
                        el.child(
                            ModalBottomSheet::new("sheet-demo")
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(8.))
                                        .child(
                                            typography
                                                .title_large
                                                .apply(div())
                                                .text_color(colors.on_surface)
                                                .child("Share with the studio"),
                                        )
                                        .child(
                                            typography
                                                .body_medium
                                                .apply(div())
                                                .text_color(colors.on_surface_variant)
                                                .child(
                                                    "Everyone here can also open \
                                                     & comment",
                                                ),
                                        ),
                                )
                                .child(material3_gpui::List::new().children(
                                    SHARE_ACTIONS.into_iter().enumerate().map(
                                        |(ix, (title, icon))| {
                                            material3_gpui::ListItem::new(("sheet-item", ix), title)
                                                .leading_icon(icon)
                                        },
                                    ),
                                ))
                                .when_some(on_dismiss, |el, on_dismiss| {
                                    el.on_dismiss(move |_window, cx| on_dismiss(cx))
                                }),
                        )
                    })
                    .into_any_element(),
            ))
            .collect::<Vec<_>>(),
        )
    }
}
