// Windows 下隐藏随 GUI 程序弹出的控制台窗口
#![windows_subsystem = "windows"]

//! material3-gpui 组件演示（入口）。
//!
//! 结构：
//! - `Catalog`：根视图——主题状态、页面导航、侧栏与头部开关、对话框与弹层宿主；
//! - `pages`：每个组件页面一个独立 Entity 视图（页面只在自身状态变化时
//!   重渲染自己，Slider 拖动 / Progress 动画不再触发整树重绘）。
//!
//! 运行：`cargo run -p catalog`

// 模块名跟随文件名的驼峰式约定,非 snake_case
#[allow(non_snake_case)]
mod Home;
#[allow(non_snake_case)]
mod Titlebar;
mod pages;

use gpui::{
    AnyView, App, Bounds, Context, Entity, IntoElement, Render, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, point, prelude::*, px, size,
};

use material3_gpui::overlay::host;
use material3_gpui::prelude::*;
use pages::Pages;

const DEFAULT_SEED: u32 = 0x6750A4;

/// 页面标识。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageId {
    Buttons,
    IconButtonsFab,
    Selection,
    Chips,
    SliderProgress,
    Tabs,
    TextFields,
    Overlays,
    Navigation,
    Cards,
    Lists,
    Dialogs,
}

/// 页面元信息：(标题, 副标题, 图标)。
pub(crate) struct PageMeta {
    pub(crate) id: PageId,
    pub(crate) title: &'static str,
    pub(crate) subtitle: &'static str,
    pub(crate) icon: IconName,
}

pub(crate) const PAGES: [PageMeta; 12] = [
    PageMeta {
        id: PageId::Buttons,
        title: "Buttons",
        subtitle: "Common button variants with ripple and state layer springs (m3fx motion)",
        icon: IconName::Add,
    },
    PageMeta {
        id: PageId::IconButtonsFab,
        title: "Icon buttons & FAB",
        subtitle: "Icon button variants and floating action button sizes",
        icon: IconName::Favorite,
    },
    PageMeta {
        id: PageId::Selection,
        title: "Selection controls",
        subtitle: "Checkbox, radio button and switch with spring-driven animations",
        icon: IconName::Check,
    },
    PageMeta {
        id: PageId::Chips,
        title: "Chips",
        subtitle: "Assist, filter, input and suggestion chips",
        icon: IconName::Info,
    },
    PageMeta {
        id: PageId::SliderProgress,
        title: "Slider & Progress",
        subtitle: "Slider with linear and circular progress indicators",
        icon: IconName::Settings,
    },
    PageMeta {
        id: PageId::Tabs,
        title: "Tabs",
        subtitle: "Primary tabs; the selection indicator slides with a fastSpatial spring",
        icon: IconName::Menu,
    },
    PageMeta {
        id: PageId::TextFields,
        title: "Text fields",
        subtitle: "Outlined text field with floating label and focus morph",
        icon: IconName::Edit,
    },
    PageMeta {
        id: PageId::Overlays,
        title: "Overlays",
        subtitle: "Snackbar, menu and tooltip rendered through the window overlay host",
        icon: IconName::MoreVert,
    },
    PageMeta {
        id: PageId::Navigation,
        title: "Navigation",
        subtitle: "Top app bar, navigation bar, rail and drawer with spring indicator",
        icon: IconName::Menu,
    },
    PageMeta {
        id: PageId::Cards,
        title: "Cards",
        subtitle: "Elevated, filled and outlined cards",
        icon: IconName::Star,
    },
    PageMeta {
        id: PageId::Lists,
        title: "Lists",
        subtitle: "List items with icons, supporting text and dividers",
        icon: IconName::Person,
    },
    PageMeta {
        id: PageId::Dialogs,
        title: "Dialogs",
        subtitle: "Modal dialog with scrim, hero icon and actions",
        icon: IconName::Delete,
    },
];

/// catalog 根视图。
struct Catalog {
    dark: bool,
    seed: u32,
    page: PageId,
    /// 竖屏下是否显示主页列表(false = 详情页)。
    home_visible: bool,
    dialog_open: bool,
    wired: bool,
    dark_switch: Entity<SwitchState>,
    pages: Pages,
}

impl Catalog {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            dark: false,
            seed: DEFAULT_SEED,
            page: PageId::Buttons,
            home_visible: true,
            dialog_open: false,
            wired: false,
            dark_switch: Switch::new("theme-switch").build(cx),
            pages: Pages::new(cx),
        }
    }

    /// 以当前模式/Profile/种子色重建主题并刷新窗口。
    fn apply_theme(&self, cx: &mut App) {
        let mode = if self.dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Theme::set(cx, Theme::from_seed(self.seed, mode, Profile::Baseline2021));
        cx.refresh_windows();
    }

    /// 首帧接线：需要根句柄的回调。
    fn wire(&mut self, cx: &mut Context<Self>) {
        if self.wired {
            return;
        }
        self.wired = true;

        let this = cx.entity();
        self.dark_switch.update(cx, |switch, _| {
            switch.set_on_change(move |checked, _window, cx| {
                this.update(cx, |d, cx| {
                    d.dark = checked;
                    d.apply_theme(cx);
                })
            });
        });

        // 种子色实时应用动态色
        let this = cx.entity();
        self.pages.text_fields.update(cx, |page, _| {
            page.set_on_seed_changed(std::rc::Rc::new(move |seed, cx| {
                this.update(cx, |d, cx| {
                    if seed != d.seed {
                        d.seed = seed;
                        d.apply_theme(cx);
                    }
                });
            }));
        });

        // Dialogs 页：打开对话框 + 对话框按钮
        let this = cx.entity();
        let dialogs_page = self.pages.dialogs.clone();
        let handle = this.clone();
        dialogs_page.update(cx, |page, _| {
            page.set_on_open_dialog(std::rc::Rc::new(move |(), cx: &mut App| {
                handle.update(cx, |d, cx| {
                    d.dialog_open = true;
                    cx.notify();
                });
            }));
        });
        let page_handle = dialogs_page.clone();
        let _handle = this.clone();
        dialogs_page.update(cx, |page, cx| {
            page.b_dialog.update(cx, |button, _| {
                button.set_on_click(move |_, _window, cx| {
                    page_handle.update(cx, |page, cx| {
                        if let Some(handler) = page.on_open_dialog.clone() {
                            handler((), cx);
                        }
                    });
                });
            });
        });
        let handle = this.clone();
        dialogs_page.update(cx, |page, cx| {
            page.dlg_cancel.update(cx, |button, _| {
                button.set_on_click(move |_, _window, cx| {
                    handle.update(cx, |d, cx| {
                        d.dialog_open = false;
                        cx.notify();
                    });
                });
            });
        });
        let handle = this;
        dialogs_page.update(cx, |page, cx| {
            page.dlg_ok.update(cx, |button, _| {
                button.set_on_click(move |_, _window, cx| {
                    handle.update(cx, |d, cx| {
                        d.dialog_open = false;
                        cx.notify();
                    });
                });
            });
        });
    }
}

impl Render for Catalog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.wire(cx);
        #[cfg(target_os = "macos")]
        hide_maximize_buttons();

        let this = cx.entity();
        let colors = *cx.theme().colors();
        let typography = *cx.theme().typography();
        let font_family = cx.theme().font_family().clone();
        let page = self.page;
        let dialog_open = self.dialog_open;
        let wired = self.wired;
        let dialogs = self.pages.dialogs.read(cx);
        let dlg_cancel = dialogs.dlg_cancel.clone();
        let dlg_ok = dialogs.dlg_ok.clone();
        let _ = wired;

        // 当前页面视图（页面只在自身状态变化时重渲染）
        let page_view: AnyView = match self.page {
            PageId::Buttons => self.pages.buttons.clone().into(),
            PageId::IconButtonsFab => self.pages.icon_buttons_fab.clone().into(),
            PageId::Selection => self.pages.selection.clone().into(),
            PageId::Chips => self.pages.chips.clone().into(),
            PageId::SliderProgress => self.pages.slider_progress.clone().into(),
            PageId::Tabs => self.pages.tabs.clone().into(),
            PageId::TextFields => self.pages.text_fields.clone().into(),
            PageId::Overlays => self.pages.overlays.clone().into(),
            PageId::Navigation => self.pages.navigation.clone().into(),
            PageId::Cards => self.pages.cards.clone().into(),
            PageId::Lists => self.pages.lists.clone().into(),
            PageId::Dialogs => self.pages.dialogs.clone().into(),
        };
        let meta = &PAGES[PAGES.iter().position(|p| p.id == page).unwrap_or(0)];
        let selected_ix = PAGES.iter().position(|p| p.id == page);
        // 横屏阈值:窗口拖宽到 840dp 及以上切换为左列表右详情双栏
        let landscape = window.viewport_size().width >= px(840.);

        // 主页面板(BakaXL 设置页风格;横屏为左栏,竖屏铺满)
        let home_pane = Home::pane(cx, selected_ix, {
            let this = this.clone();
            std::rc::Rc::new(move |ix: usize, cx: &mut App| {
                this.update(cx, |d, cx| {
                    d.page = PAGES[ix].id;
                    d.home_visible = false;
                    cx.notify();
                })
            })
        });

        // 详情页头(竖屏详情加返回按钮)
        let back_button = div()
            .id("detail-back")
            .size(px(40.))
            .flex_none()
            .rounded_full()
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .hover(|s| s.bg(colors.on_surface.opacity(0.08)))
            .child(
                Icon::new(IconName::ArrowBack)
                    .size(px(24.))
                    .color(colors.on_surface),
            )
            .on_click({
                let this = this.clone();
                move |_, _w, cx| {
                    this.update(cx, |d, cx| {
                        d.home_visible = true;
                        cx.notify();
                    })
                }
            });

        // 页头
        let content_header = div()
            .flex()
            .flex_wrap()
            .items_start()
            .justify_between()
            .gap(px(16.))
            .when(!landscape, |el| el.child(back_button))
            .child(
                div()
                    .flex_1()
                    .min_w(px(180.))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        typography
                            .headline_large
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child(meta.title),
                    )
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(meta.subtitle),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        typography
                            .label_large
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(if self.dark { "Dark" } else { "Light" }),
                    )
                    .child(self.dark_switch.clone()),
            );

        let content = div()
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.surface)
            .child(
                div()
                    .px(px(12.))
                    .pt(px(20.))
                    .pb(px(12.))
                    .child(content_header),
            )
            .child(
                div()
                    .id("catalog-content")
                    .flex_1()
                    .overflow_y_scroll()
                    .px(px(12.))
                    .pb(px(24.))
                    .child(page_view),
            );

        div()
            .id("catalog-root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.surface)
            .font_family(font_family)
            .text_color(colors.on_surface)
            // 自定义标题栏(隐藏系统标题栏后的窗体框架)
            .child(Titlebar::CustomTitleBar)
            .child(if landscape {
                // 横屏双栏:左主页面板 + 右详情
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .overflow_hidden()
                    .child(
                        div()
                            .w(px(440.))
                            .h_full()
                            .flex_none()
                            .flex()
                            .flex_col()
                            .bg(colors.surface)
                            .border_r_1()
                            .border_color(colors.outline_variant)
                            .child(home_pane),
                    )
                    .child(content)
            } else if self.home_visible {
                // 竖屏:主页列表铺满
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .bg(colors.surface)
                    .child(home_pane)
            } else {
                // 竖屏:详情页铺满
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(content)
            })
            // 窗口级弹层宿主：Snackbar / Menu / Tooltip
            .child(host(window, cx))
            .when(dialog_open, |el| {
                let close = {
                    let this = this.clone();
                    move |cx: &mut App| {
                        this.update(cx, |d, cx| {
                            d.dialog_open = false;
                            cx.notify();
                        })
                    }
                };
                el.child(
                    Dialog::new("catalog-dialog")
                        .icon(IconName::Delete)
                        .title("Permanently delete?")
                        .child(
                            "Deleting the selected items will also remove them from all synced \
                             devices. This action cannot be undone.",
                        )
                        .action(dlg_cancel.clone())
                        .action(dlg_ok.clone())
                        .on_dismiss(move |_w, cx| close(cx)),
                )
            })
    }
}

fn main() {
    gpui::Application::new()
        .with_assets(Md3Assets)
        .run(|cx: &mut App| {
            material3_gpui::init(cx);
            // 竖屏窗体:初始与最小尺寸一致(456×700,宽度对齐 BakaXL)
            let initial = size(px(456.), px(700.));
            let min_size = size(px(456.), px(700.));
            let bounds = Bounds::centered(None, initial, cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("material3-gpui".into()),
                        // 隐藏系统标题栏,由 catalog 自绘(Windows;macOS 红绿灯仍在)
                        appears_transparent: true,
                        // macOS 红绿灯显式定位:系统默认按 28dp 标题栏摆放,
                        // 在自绘 40dp 栏里会偏上;按钮高 16,12 使其在 40dp 内垂直居中
                        traffic_light_position: Some(point(px(9.), px(24.))),
                    }),
                    window_min_size: Some(min_size),
                    ..Default::default()
                },
                |_, cx| cx.new(Catalog::new),
            )
            .unwrap();
            cx.activate(true);
        });
}

/// 隐藏 macOS 红绿灯中的绿色最大化按钮(仅保留关闭与最小化)。
///
/// gpui 公开 API 只能整体定位红绿灯(`traffic_light_position`),不提供
/// 单个灯的显隐;此处经 AppKit 遍历本应用全部窗口,把 zoom 标准按钮隐藏。
/// AppKit 在窗口样式变化时可能重建标准按钮(如进出全屏),故随每帧重设。
/// objc2 绑定均为安全方法,主线程约束由 `MainThreadMarker` 保证。
#[cfg(target_os = "macos")]
fn hide_maximize_buttons() {
    use objc2::MainThreadMarker;
    use objc2_app_kit::{NSApplication, NSWindowButton};

    if let Some(mtm) = MainThreadMarker::new() {
        let app = NSApplication::sharedApplication(mtm);
        for window in app.windows().iter() {
            if let Some(zoom) = window.standardWindowButton(NSWindowButton::ZoomButton) {
                zoom.setHidden(true);
            }
        }
    }
}
