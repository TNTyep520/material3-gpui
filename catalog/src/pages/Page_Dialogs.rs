//! Dialogs 页「Manage files」：对话框启动按钮在本页，对话框本体由根视图渲染。

use gpui::{App, Entity, IntoElement, Render, Styled, Window, div, prelude::*, px};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Dialogs 页视图。
pub struct DialogsPage {
    pub b_dialog: Entity<ButtonState>,
    pub b_help: Entity<ButtonState>,
    pub dlg_cancel: Entity<ButtonState>,
    pub dlg_ok: Entity<ButtonState>,
    /// 打开对话框回调（由根视图接线）。
    pub(crate) on_open_dialog: Option<super::PageCallback<()>>,
}

impl DialogsPage {
    /// 创建页面及其初始组件状态。
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_dialog = Button::new("b-dialog", "Delete recordings")
            .filled()
            .build(cx);
        let b_help = Button::new("b-dialog-help", "How dialogs work")
            .tonal()
            .on_click(|_, window, cx| {
                show_snackbar(
                    window,
                    cx,
                    Snackbar::new("Dialogs mount at the window root"),
                    None,
                );
            })
            .build(cx);
        let dlg_cancel = Button::new("dlg-cancel", "Cancel").text().build(cx);
        let dlg_ok = Button::new("dlg-ok", "Delete").text().build(cx);

        cx.new(|_| Self {
            b_dialog,
            b_help,
            dlg_cancel,
            dlg_ok,
            on_open_dialog: None,
        })
    }

    /// 设置打开对话框回调（根视图首帧接线）。
    pub fn set_on_open_dialog(&mut self, handler: super::PageCallback<()>) {
        self.on_open_dialog = Some(handler);
    }
}

impl Render for DialogsPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();

        gallery([showcase_group(
            cx,
            "Dialog launchers",
            [div()
                .w_full()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(
                    typography
                        .body_medium
                        .apply(div())
                        .text_color(colors.on_surface_variant)
                        .child(
                            "Destructive actions pause on a modal dialog first. Tap \
                             scrim or Cancel to dismiss, Delete to confirm.",
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(self.b_dialog.clone())
                        .child(self.b_help.clone()),
                )
                .into_any_element()],
        )])
    }
}
