//! 自定义标题栏(客户端窗口装饰)。
//!
//! 布局:`[站点图标 36dp] .......... [最小化][关闭]`,图标尺寸对齐
//! BakaXL 标题栏实测值;按钮图标用 Material Symbols 字体字形。
//!
//! 平台行为:
//! - **Windows**:整条标题栏标 [`WindowControlArea::Drag`](拖动/双击最大化
//!   由系统接管),两个按钮标 `Min`/`Close` 由系统原生执行,自身**不挂
//!   `on_click`**(挂了会消费 NC 事件导致原生动作失效);
//! - **Linux**:按钮挂 `on_click` 调 `minimize_window()`/`remove_window()`,
//!   拖动用 `start_window_move()`(Wayland/X11 生效),双击标题栏切
//!   `zoom_window()`;
//! - **macOS**:不渲染自定义按钮(系统红绿灯仍在),仅留避让内边距。

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    App, ClickEvent, Hsla, IntoElement, MouseButton, ParentElement as _, RenderOnce, Rgba,
    StatefulInteractiveElement as _, Styled, Window, WindowControlArea, div, img, prelude::*, px,
};
use material3_gpui::assets::MATERIAL3_FAVICON_SVG_PATH;
use material3_gpui::fonts::TEXT_FONT_FAMILY;
use material3_gpui::icon::{Icon, IconName};
use material3_gpui::prelude::ActiveTheme;

/// 标题栏高度。
const HEIGHT: f32 = 40.0;
/// 标题栏图标尺寸。
const ICON_SIZE: f32 = 20.0;
/// 图标右侧的文字。
const TITLE_TEXT: &str = "Material3 Catalog";
/// 每个窗口按钮的宽度。
const BUTTON_WIDTH: f32 = 40.0;
/// 关闭按钮悬停红(Windows 11 惯例)。
const CLOSE_HOVER_RED: Rgba = Rgba {
    r: 232.0 / 255.0,
    g: 17.0 / 255.0,
    b: 32.0 / 255.0,
    a: 1.0,
};
/// macOS 红绿灯避让内边距(关闭+最小化两灯止于约 45dp,另留 12dp 间距)。
const MAC_TRAFFIC_LIGHT_INSET: f32 = 48.0;

/// 自定义标题栏。
#[derive(IntoElement)]
pub struct CustomTitleBar;

impl RenderOnce for CustomTitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = *cx.theme().colors();
        let is_windows = cfg!(target_os = "windows");
        let is_mac = cfg!(target_os = "macos");
        let is_linux = !is_windows && !is_mac;

        // Linux 拖动:按下置位、首次移动即交给 compositor(Zed 模式)
        let dragging = Rc::new(Cell::new(false));

        let title_bar = div()
            .id("custom-title-bar")
            .relative()
            .h(px(HEIGHT))
            .w_full()
            .flex_none()
            .flex()
            .items_center()
            .bg(colors.surface)
            .when(is_windows, |el| {
                el.window_control_area(WindowControlArea::Drag)
            })
            .when(is_linux, |el| {
                let down = dragging.clone();
                let up = dragging.clone();
                let mv = dragging.clone();
                el.on_mouse_down(MouseButton::Left, move |_, _, _| down.set(true))
                    .on_mouse_up(MouseButton::Left, move |_, _, _| up.set(false))
                    .on_mouse_move(move |_, window, _| {
                        if mv.get() {
                            mv.set(false);
                            window.start_window_move();
                        }
                    })
                    .on_click(move |event, window, _| {
                        if event.click_count() == 2 {
                            window.zoom_window();
                        }
                    })
            });

        // 左侧:站点图标(macOS 先避让红绿灯)
        let title_bar = title_bar
            .when(is_mac, |el| {
                el.child(div().flex_none().w(px(MAC_TRAFFIC_LIGHT_INSET)))
            })
            .child(
                img(MATERIAL3_FAVICON_SVG_PATH)
                    .size(px(ICON_SIZE))
                    .ml(px(12.))
                    .flex_none(),
            )
            .child(
                div()
                    .ml(px(10.))
                    .text_size(px(13.))
                    .font_family(TEXT_FONT_FAMILY)
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .text_color(colors.on_surface)
                    .flex_none()
                    .child(TITLE_TEXT),
            )
            // 弹性空白:按钮推到右侧
            .child(div().flex_1());

        // 右侧窗口按钮:仅 Windows/Linux;macOS 用系统红绿灯
        let controls = if is_mac {
            div()
        } else {
            div()
                .absolute()
                .right(px(0.))
                .top(px(0.))
                .h(px(HEIGHT))
                .flex()
                .items_center()
                .child(window_button(
                    "titlebar-minimize",
                    WindowControlArea::Min,
                    colors.on_surface.opacity(0.08),
                    MINIMIZE_CODEPOINT,
                    move |_event, window, _cx| window.minimize_window(),
                    is_windows,
                ))
                .child(window_button(
                    "titlebar-close",
                    WindowControlArea::Close,
                    CLOSE_HOVER_RED,
                    "close-button",
                    move |_event, window, _cx| window.remove_window(),
                    is_windows,
                ))
        };

        title_bar.child(controls)
    }
}

/// 窗口按钮:图标居中、hover 样式、窗口控制区标记与(Linux)点击动作。
///
/// - Windows:标 [`WindowControlArea`] 即可,动作由系统 NC 路径执行;
/// - Linux:挂 `on_click` 调用窗口 API(`window_control_area` 在 Linux 无效)。
fn window_button(
    id: &'static str,
    area: WindowControlArea,
    hover_bg: impl Into<Hsla>,
    icon_name: &'static str,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    windows_native: bool,
) -> impl IntoElement {
    let hover_bg: Hsla = hover_bg.into();
    let base = div()
        .id(id)
        .w(px(BUTTON_WIDTH))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .occlude()
        .hover(move |s| s.bg(hover_bg).text_color(gpui::white()))
        .active(move |s| s.bg(hover_bg).opacity(0.8));
    let base = if windows_native {
        base.window_control_area(area)
    } else {
        base.on_click(move |event, window, cx| {
            cx.stop_propagation();
            on_click(event, window, cx)
        })
    };
    // 关闭按钮用 ligature(已在 demo 各处验证);最小化用码点直取
    let icon = if icon_name == "close-button" {
        Icon::new(IconName::Close).size(px(16.))
    } else {
        Icon::ligature(icon_name).size(px(16.))
    };
    base.child(icon)
}

/// 最小化按钮的 Material Symbols 码点(0xE15B,cmap 已验证存在)。
const MINIMIZE_CODEPOINT: &str = "\u{e15b}";
