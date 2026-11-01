// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 壳层布局对齐 Compose Material Catalog(androidx):
// home(卡片网格)→ component(描述 + 示例列表)→ example(居中示例)。

#![windows_subsystem = "windows"]

mod icons;
mod pages;

use gpui::{
    App, Bounds, Context, Entity, IntoElement, Render, TitlebarOptions, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

use material3_gpui::prelude::*;
use pages::{LogErr as _, PAGES, PageId, Pages};

const DEFAULT_SEED: u32 = 0x6750A4;

/// 顶栏标题与菜单文案,逐字取自 Compose Material Catalog 的字符串资源。
const APP_TITLE: &str = "Compose Material 3";

const GUIDELINES_URL: &str = "https://m3.material.io/components";

const DOCS_URL: &str =
    "https://developer.android.com/reference/kotlin/androidx/compose/material3/package-summary";

const SOURCE_URL: &str = "https://cs.android.com/androidx/platform/frameworks/support/+/androidx-main:compose/material3/";

const ISSUE_URL: &str = "https://issuetracker.google.com/issues?q=component:907884";

/// 导航栈条目,对应 APK 的 home / component/{id} / example/{id}/{index}。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Route {
    Home,
    Component(PageId),
    Example(PageId, usize),
}

struct Catalog {
    dark: bool,
    pin: bool,
    theme_sheet_open: bool,
    dialog_open: bool,
    wired: bool,
    dark_switch: Entity<SwitchState>,
    pages: Pages,
    route_stack: Vec<Route>,
}

impl Catalog {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            dark: false,
            pin: false,
            theme_sheet_open: false,
            dialog_open: false,
            wired: false,
            dark_switch: Switch::new("theme-switch").build(cx),
            pages: Pages::new(cx),
            route_stack: vec![Route::Home],
        }
    }

    fn route(&self) -> Route {
        *self.route_stack.last().expect("route stack never empty")
    }

    fn navigate(&mut self, route: Route, cx: &mut Context<Self>) {
        self.route_stack.push(route);
        cx.notify();
    }

    fn go_back(&mut self, cx: &mut Context<Self>) {
        if self.route_stack.len() > 1 {
            self.route_stack.pop();
            cx.notify();
        }
    }

    fn apply_theme(&self, cx: &mut App) {
        let mode = if self.dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Theme::set(
            cx,
            Theme::from_seed(DEFAULT_SEED, mode, Profile::Baseline2021),
        );
        cx.refresh_windows();
    }

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

        // Dialogs 示例的启动按钮 → 根对话框。
        let handle = cx.entity();
        let dialogs_page = self.pages.dialogs.clone();
        let launchers = dialogs_page.read(cx).launchers();
        dialogs_page.update(cx, |page, _| {
            page.set_on_open_dialog(std::rc::Rc::new(move |(), cx: &mut App| {
                handle.update(cx, |d, cx| {
                    d.dialog_open = true;
                    cx.notify();
                });
            }));
        });
        let page_handle = dialogs_page.clone();
        for launcher in launchers {
            let page_handle = page_handle.clone();
            launcher.update(cx, |button, _| {
                button.set_on_click(move |_, _window, cx| {
                    page_handle.update(cx, |page, cx| {
                        if let Some(handler) = page.on_open_dialog.clone() {
                            handler((), cx);
                        }
                    });
                });
            });
        }

        // Sheets 示例:打开按钮与关闭回调。
        let open_button = self.pages.bottom_sheet.read(cx).open_button.clone();
        let sheet_page = self.pages.bottom_sheet.downgrade();
        open_button.update(cx, |button, _| {
            button.set_on_click(move |_, _, cx| {
                sheet_page
                    .update(cx, |page, cx| {
                        page.sheet_open = true;
                        cx.notify();
                    })
                    .log_err();
            });
        });
        let sheet_page = self.pages.bottom_sheet.downgrade();
        self.pages.bottom_sheet.update(cx, |page, _| {
            page.set_on_dismiss(move |cx| {
                sheet_page
                    .update(cx, |page, cx| {
                        page.sheet_open = false;
                        cx.notify();
                    })
                    .log_err();
            });
        });
    }
}

impl Render for Catalog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.wire(cx);

        let colors = *cx.theme().colors();
        let typography = *cx.theme().typography();
        let font_family = cx.theme().font_family().clone();
        let route = self.route();
        let theme_sheet_open = self.theme_sheet_open;
        let dialog_open = self.dialog_open;
        let pin = self.pin;

        let title = match route {
            Route::Home => APP_TITLE.to_string(),
            Route::Component(id) => PAGES
                .iter()
                .find(|meta| meta.id == id)
                .map(|meta| meta.name.to_string())
                .unwrap_or_default(),
            Route::Example(id, index) => {
                let page = self.pages.page(id, cx);
                page.examples()
                    .get(index)
                    .map(|example| example.name.to_string())
                    .unwrap_or_default()
            }
        };

        // 返回按钮(非首页)。
        let back_button = div()
            .id("topbar-back")
            .size(px(40.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(move |style| style.bg(colors.surface_container_highest))
            .child(
                Icon::new(IconName::new("arrow_back"))
                    .size(px(24.))
                    .color(colors.on_surface),
            )
            .on_click(cx.listener(|this, _, _, cx| this.go_back(cx)));

        // Pin / Palette / More 三个顶栏操作。
        let pin_icon = if pin { "push_pin_filled" } else { "push_pin" };
        let pin_color = if pin {
            colors.primary
        } else {
            colors.on_surface_variant
        };
        let pin_button = div()
            .id("topbar-pin")
            .size(px(40.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(move |style| style.bg(colors.surface_container_highest))
            .child(
                Icon::new(IconName::new(pin_icon))
                    .size(px(24.))
                    .color(pin_color),
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.pin = !this.pin;
                cx.notify();
            }));

        let palette_button = div()
            .id("topbar-theme")
            .size(px(40.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(move |style| style.bg(colors.surface_container_highest))
            .child(
                Icon::new(IconName::new("palette"))
                    .size(px(24.))
                    .color(colors.on_surface_variant),
            )
            .on_click(cx.listener(|this, _, _, cx| {
                this.theme_sheet_open = true;
                cx.notify();
            }));

        let menu = MenuState::new()
            .item(
                MenuItem::new("View design guidelines").on_click(|_window, cx| {
                    cx.open_url(GUIDELINES_URL);
                }),
            )
            .item(
                MenuItem::new("View developer docs").on_click(|_window, cx| {
                    cx.open_url(DOCS_URL);
                }),
            )
            .item(MenuItem::new("View source code").on_click(|_window, cx| {
                cx.open_url(SOURCE_URL);
            }))
            .item(MenuItem::new("Report an issue").on_click(|_window, cx| {
                cx.open_url(ISSUE_URL);
            }))
            .build(cx);

        let more_button = div()
            .id("topbar-more")
            .size(px(40.))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(move |style| style.bg(colors.surface_container_highest))
            .child(
                Icon::new(IconName::new("more_vert"))
                    .size(px(24.))
                    .color(colors.on_surface_variant),
            )
            .on_click(move |event, window, cx| {
                let anchor = gpui::Bounds {
                    origin: event.position(),
                    size: size(px(0.), px(0.)),
                };
                show_menu(window, cx, menu.clone(), anchor);
            });

        let top_bar = div()
            .w_full()
            .h(px(64.))
            .flex_none()
            .flex()
            .items_center()
            .gap(px(12.))
            .px(px(4.))
            .bg(colors.surface)
            .child(
                div()
                    .w(px(48.))
                    .flex_none()
                    .flex()
                    .justify_center()
                    .when(!matches!(route, Route::Home), |el| el.child(back_button)),
            )
            .child(
                typography
                    .title_large
                    .apply(div())
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_color(colors.on_surface)
                    .child(title),
            )
            .child(
                div()
                    .flex_none()
                    .flex()
                    .items_center()
                    .child(pin_button)
                    .child(palette_button)
                    .child(more_button),
            );

        let content = match route {
            Route::Home => self.render_home(cx).into_any_element(),
            Route::Component(id) => self.render_component(id, cx).into_any_element(),
            Route::Example(id, index) => self.render_example(id, index, cx).into_any_element(),
        };

        let dialogs = self.pages.dialogs.read(cx);
        let dlg_cancel = dialogs.dlg_cancel.clone();
        let dlg_ok = dialogs.dlg_ok.clone();
        let dark_switch = self.dark_switch.clone();
        let sheet_open = self.pages.bottom_sheet.read(cx).sheet_open();
        let sheet = self.pages.bottom_sheet.read(cx).sheet(cx);

        div()
            .id("catalog-root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.surface)
            .font_family(font_family)
            .text_color(colors.on_surface)
            .child(top_bar)
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .min_w_0()
                    .flex()
                    .overflow_hidden()
                    .child(content),
            )
            .child(host(window, cx))
            .when(sheet_open, |el| el.child(sheet))
            .when(theme_sheet_open, |el| {
                el.child(
                    ModalBottomSheet::new("theme-sheet")
                        .on_dismiss({
                            let this = cx.entity();
                            move |_, cx| {
                                this.update(cx, |d, cx| {
                                    d.theme_sheet_open = false;
                                    cx.notify();
                                });
                            }
                        })
                        .child(
                            div()
                                .p(px(24.))
                                .flex()
                                .flex_col()
                                .gap(px(16.))
                                .child(
                                    typography
                                        .title_large
                                        .apply(div())
                                        .text_color(colors.on_surface)
                                        .child("Change theme"),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            typography
                                                .body_large
                                                .apply(div())
                                                .text_color(colors.on_surface)
                                                .child("Dark mode"),
                                        )
                                        .child(dark_switch),
                                ),
                        ),
                )
            })
            .when(dialog_open, |el| {
                let close = {
                    let this = cx.entity();
                    move |cx: &mut App| {
                        this.update(cx, |d, cx| {
                            d.dialog_open = false;
                            cx.notify();
                        })
                    }
                };
                el.child(
                    Dialog::new("catalog-dialog")
                        .icon(IconName::new("delete"))
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

impl Catalog {
    /// APK Home:自适应 180dp 列宽的组件卡片网格。
    fn render_home(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = *cx.theme().colors();
        let typography = *cx.theme().typography();

        let cards = PAGES
            .iter()
            .enumerate()
            .map(|(index, meta)| {
                let target = meta.id;
                let info = meta.additional_info;
                div()
                    .id(("home-card", index))
                    .w(px(180.))
                    .h(px(180.))
                    .m(px(4.))
                    .p(px(16.))
                    .relative()
                    .border_1()
                    .border_color(colors.outline_variant)
                    .rounded(px(12.))
                    .bg(colors.surface)
                    .cursor_pointer()
                    .hover(move |style| style.bg(colors.surface_container_low))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(IconName::new(meta.icon))
                                    .size(px(80.))
                                    .color(colors.on_surface_variant),
                            ),
                    )
                    .when(!info.is_empty(), |el| {
                        el.child(
                            div()
                                .absolute()
                                .top(px(8.))
                                .right(px(8.))
                                .px(px(4.))
                                .rounded(px(8.))
                                .bg(colors.error)
                                .child(
                                    typography
                                        .label_small
                                        .apply(div())
                                        .text_color(colors.surface)
                                        .child(info),
                                ),
                        )
                    })
                    .child(
                        typography
                            .body_small
                            .apply(div())
                            .absolute()
                            .bottom(px(16.))
                            .left(px(16.))
                            .text_color(colors.on_surface)
                            .child(meta.name),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate(Route::Component(target), cx);
                    }))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        div()
            .id("home-scroll")
            .flex_1()
            .min_h_0()
            .min_w_0()
            .overflow_y_scroll()
            .p(px(8.))
            .flex()
            .flex_wrap()
            .content_start()
            .children(cards)
    }

    /// APK Component 页:居中大图标 + Description + Examples 卡片列表。
    fn render_component(&self, id: PageId, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = *cx.theme().colors();
        let typography = *cx.theme().typography();
        let meta = PAGES
            .iter()
            .find(|meta| meta.id == id)
            .expect("valid PageId");
        let page = self.pages.page(id, cx);
        let description = page.description();
        let examples = page.examples();

        let example_items = examples
            .iter()
            .enumerate()
            .map(|(index, example)| {
                let target = (id, index);
                div()
                    .id(("example-item", index))
                    .w_full()
                    .p(px(16.))
                    .border_1()
                    .border_color(colors.outline_variant)
                    .rounded(px(12.))
                    .bg(colors.surface)
                    .cursor_pointer()
                    .hover(move |style| style.bg(colors.surface_container_low))
                    .flex()
                    .items_center()
                    .gap(px(16.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                typography
                                    .title_small
                                    .apply(div())
                                    .text_color(colors.on_surface)
                                    .child(example.name),
                            )
                            .child(
                                typography
                                    .body_small
                                    .apply(div())
                                    .text_color(colors.on_surface_variant)
                                    .child(example.description),
                            ),
                    )
                    .child(
                        Icon::new(IconName::new("chevron_right"))
                            .size(px(24.))
                            .color(colors.on_surface_variant),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate(Route::Example(target.0, target.1), cx);
                    }))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        div()
            .id(("component-scroll", meta.id as usize))
            .flex_1()
            .min_h_0()
            .min_w_0()
            .overflow_y_scroll()
            .p(px(16.))
            .flex()
            .flex_col()
            .child(
                div().w_full().py(px(42.)).flex().justify_center().child(
                    Icon::new(IconName::new(meta.icon))
                        .size(px(108.))
                        .color(colors.on_surface_variant),
                ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .pb(px(32.))
                    .child(
                        typography
                            .body_large
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child("Description"),
                    )
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface)
                            .child(description),
                    ),
            )
            .child(
                typography
                    .body_large
                    .apply(div())
                    .pb(px(16.))
                    .text_color(colors.on_surface)
                    .child("Examples"),
            )
            .children(example_items)
    }

    /// APK Example 页:顶栏下居中的示例内容。
    fn render_example(&self, id: PageId, index: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let page = self.pages.page(id, cx);
        let content = page.example(index, cx);
        div()
            .id(("example-scroll", index))
            .flex_1()
            .min_h_0()
            .min_w_0()
            .overflow_y_scroll()
            .p(px(16.))
            .flex()
            .flex_col()
            .child(
                div()
                    .min_h_0()
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().w_full().max_w(px(960.)).min_w_0().child(content)),
            )
    }
}

fn main() {
    gpui::Application::new()
        .with_assets(icons::CatalogAssets)
        .run(|cx: &mut App| {
            material3_gpui::init(cx);

            let initial = size(px(1080.), px(760.));
            let min_size = size(px(996.), px(621.));
            let bounds = Bounds::centered(None, initial, cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Compose Material 3 Catalog".into()),

                        appears_transparent: false,

                        ..Default::default()
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
