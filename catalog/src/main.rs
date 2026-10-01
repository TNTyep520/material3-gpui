// Windows 下隐藏随 GUI 程序弹出的控制台窗口
#![windows_subsystem = "windows"]

//! material3-gpui 组件展厅入口。
mod pages;
mod titlebar;

use gpui::{
    AnyView, App, Bounds, Context, Entity, IntoElement, KeyDownEvent, Render, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, point, prelude::*, px, size,
};

use material3_gpui::overlay::host;
use material3_gpui::prelude::*;
use pages::{Pages, palette_strip};

const DEFAULT_SEED: u32 = 0x6750A4;

/// 实体更新失败的兜底(窗口/页面已释放等场景):输出到 stderr 后放行。
pub(crate) trait LogErr<T> {
    fn log_err(self) -> Option<T>;
}

impl<T, E: std::fmt::Display> LogErr<T> for Result<T, E> {
    fn log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                eprintln!("catalog: {err}");
                None
            }
        }
    }
}

/// 页面标识。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageId {
    Buttons,
    Additional,
    ButtonsExtended,
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
    AppBars,
    Sheets,
}

/// 页面元信息：导航和标题使用相同的组件名称。
pub(crate) struct PageMeta {
    pub(crate) id: PageId,
    /// 导航与顶栏使用的短标签。
    pub(crate) title: &'static str,
    pub(crate) icon: IconName,
}

pub(crate) const PAGES: [PageMeta; 16] = [
    PageMeta {
        id: PageId::Buttons,
        title: "Buttons",
        icon: IconName::Star,
    },
    PageMeta {
        id: PageId::Additional,
        title: "Additional",
        icon: IconName::Settings,
    },
    PageMeta {
        id: PageId::IconButtonsFab,
        title: "Icon buttons & FAB",
        icon: IconName::Favorite,
    },
    PageMeta {
        id: PageId::ButtonsExtended,
        title: "Toggle & split",
        icon: IconName::MoreVert,
    },
    PageMeta {
        id: PageId::Selection,
        title: "Selection",
        icon: IconName::Check,
    },
    PageMeta {
        id: PageId::Chips,
        title: "Chips",
        icon: IconName::Info,
    },
    PageMeta {
        id: PageId::SliderProgress,
        title: "Slider & progress",
        icon: IconName::ProgressActivity,
    },
    PageMeta {
        id: PageId::Tabs,
        title: "Tabs",
        icon: IconName::Menu,
    },
    PageMeta {
        id: PageId::TextFields,
        title: "Text fields",
        icon: IconName::Edit,
    },
    PageMeta {
        id: PageId::Overlays,
        title: "Overlays",
        icon: IconName::MoreVert,
    },
    PageMeta {
        id: PageId::Navigation,
        title: "Navigation",
        icon: IconName::Menu,
    },
    PageMeta {
        id: PageId::Cards,
        title: "Cards",
        icon: IconName::Star,
    },
    PageMeta {
        id: PageId::Lists,
        title: "Lists",
        icon: IconName::Person,
    },
    PageMeta {
        id: PageId::Dialogs,
        title: "Dialogs",
        icon: IconName::Delete,
    },
    PageMeta {
        id: PageId::AppBars,
        title: "App bars",
        icon: IconName::Home,
    },
    PageMeta {
        id: PageId::Sheets,
        title: "Bottom sheet",
        icon: IconName::Menu,
    },
];

/// catalog 根视图。
struct Catalog {
    dark: bool,
    seed: u32,
    page: PageId,
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

        // Bottom sheet 页:打开按钮与 scrim 关闭回调
        let open_button = self.pages.sheets.read(cx).open_button.clone();
        let sheets_weak = self.pages.sheets.downgrade();
        open_button.update(cx, |button, _| {
            button.set_on_click(move |_, _, cx| {
                sheets_weak
                    .update(cx, |page, cx| {
                        page.sheet_open = true;
                        cx.notify();
                    })
                    .log_err();
            });
        });

        let sheets_weak = self.pages.sheets.downgrade();
        self.pages.sheets.update(cx, |page, _| {
            page.set_on_dismiss(move |cx| {
                sheets_weak
                    .update(cx, |page, cx| {
                        page.sheet_open = false;
                        cx.notify();
                    })
                    .log_err();
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
                button.set_on_click(move |_, window, cx| {
                    handle.update(cx, |d, cx| {
                        d.dialog_open = false;
                        cx.notify();
                    });
                    material3_gpui::overlay::show_snackbar(
                        window,
                        cx,
                        Snackbar::new("3 recordings deleted"),
                        None,
                    );
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

        // 当前页面视图（页面只在自身状态变化时重渲染）
        let page_view: AnyView = match self.page {
            PageId::Buttons => self.pages.buttons.clone().into(),
            PageId::Additional => self.pages.additional.clone().into(),
            PageId::ButtonsExtended => self.pages.buttons_extended.clone().into(),
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
            PageId::AppBars => self.pages.app_bars.clone().into(),
            PageId::Sheets => self.pages.sheets.clone().into(),
        };
        let meta = &PAGES[PAGES.iter().position(|p| p.id == page).unwrap_or(0)];
        // 对话框按钮实体属于 Dialogs 页(由其构造期创建)
        let dialogs = self.pages.dialogs.read(cx);
        let dlg_cancel = dialogs.dlg_cancel.clone();
        let dlg_ok = dialogs.dlg_ok.clone();

        let rail_pane = div()
            .w(px(232.))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_r_1()
            .border_color(colors.outline_variant)
            .child(
                typography
                    .label_large
                    .apply(div())
                    .px(px(20.))
                    .py(px(20.))
                    .text_color(colors.on_surface_variant)
                    .child("Components"),
            )
            .child(
                div()
                    .id("catalog-navigation")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(12.))
                    .pb(px(12.))
                    .children(PAGES.iter().enumerate().map(|(index, meta)| {
                        let selected = meta.id == page;
                        let target = meta.id;
                        let color = if selected {
                            colors.on_secondary_container
                        } else {
                            colors.on_surface_variant
                        };
                        div()
                            .id(("catalog-page", index))
                            .focusable()
                            .tab_stop(true)
                            .w_full()
                            .h(px(44.))
                            .mb(px(4.))
                            .px(px(12.))
                            .rounded(px(12.))
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .cursor_pointer()
                            .focus(|style| style.bg(colors.secondary_container))
                            .text_color(color)
                            .when(selected, |element| element.bg(colors.secondary_container))
                            .when(!selected, |element| {
                                element.hover(move |style| style.bg(colors.surface_container_high))
                            })
                            .child(Icon::new(meta.icon).size(px(20.)).color(color))
                            .child(typography.label_large.apply(div()).child(meta.title))
                            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                                    this.page = target;
                                    cx.notify();
                                    cx.stop_propagation();
                                }
                            }))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.page = target;
                                cx.notify();
                            }))
                    })),
            );
        let theme_action = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(12.))
            .px(px(12.))
            .child(
                typography
                    .label_large
                    .apply(div())
                    .text_color(colors.on_surface_variant)
                    .child(if self.dark { "Dark" } else { "Light" }),
            )
            .child(self.dark_switch.clone());

        let detail_bar = div()
            .h(px(72.))
            .w_full()
            .flex_none()
            .px(px(28.))
            .border_b_1()
            .border_color(colors.outline_variant)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        typography
                            .title_large
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child(meta.title),
                    )
                    .child(
                        typography
                            .label_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child("Material 3 · variants and states"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(24.))
                    .child(palette_strip(cx))
                    .child(theme_action),
            );

        let content = div()
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .flex_col()
            .child(detail_bar)
            .child(
                div()
                    .id((
                        "catalog-content",
                        PAGES.iter().position(|meta| meta.id == page).unwrap_or(0),
                    ))
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .overflow_y_scroll()
                    .p(px(28.))
                    .child(div().w_full().max_w(px(1120.)).mx_auto().child(page_view)),
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
            .child(titlebar::CustomTitleBar)
            // 双栏:左导航抽屉 + 右组件详情(横屏固定布局)
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .overflow_hidden()
                    .child(rail_pane)
                    .child(content),
            )
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
                        .title("Delete 3 recordings?")
                        .child(
                            "The recordings and their transcripts will be removed from \
                             your library. This action cannot be undone.",
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
            // 横屏尺寸对齐资源管理器 tokens 窗口实测(996×621)
            let initial = size(px(996.), px(621.));
            let min_size = size(px(996.), px(621.));
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
