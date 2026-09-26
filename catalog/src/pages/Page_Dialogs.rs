//! Dialogs 页：打开按钮在本页，对话框本体由根视图渲染（浮层需在根层）。

use gpui::{App, Entity, IntoElement, Render, Window, prelude::*};
use material3_gpui::prelude::*;

use super::{gallery, showcase_group};

/// Dialogs 页视图。
pub struct DialogsPage {
    pub b_dialog: Entity<ButtonState>,
    pub dlg_cancel: Entity<ButtonState>,
    pub dlg_ok: Entity<ButtonState>,
    /// 打开对话框回调（由根视图接线）。
    pub(crate) on_open_dialog: Option<super::PageCallback<()>>,
}

impl DialogsPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let b_dialog = Button::new("b-dialog", "Open dialog").outlined().build(cx);
        let dlg_cancel = Button::new("dlg-cancel", "Cancel").text().build(cx);
        let dlg_ok = Button::new("dlg-ok", "Delete").text().build(cx);

        cx.new(|_| Self {
            b_dialog,
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
        gallery([showcase_group(
            cx,
            "Dialog Launchers",
            [self.b_dialog.clone().into_any_element()],
        )])
    }
}
