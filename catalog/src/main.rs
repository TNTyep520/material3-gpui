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

#![windows_subsystem = "windows"]

mod pages;
mod titlebar;

use std::time::Instant;

use gpui::{
    AnyView, App, Bounds, Context, Entity, IntoElement, KeyDownEvent, Render, TitlebarOptions,
    Window, WindowBounds, WindowOptions, div, point, prelude::*, px, size,
};

use material3_gpui::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use material3_gpui::overlay::host;
use material3_gpui::prelude::*;
use pages::{LogErr as _, Pages, palette_strip};

const DEFAULT_SEED: u32 = 0x6750A4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageId {
    Buttons,
    ButtonsExtended,
    IconButtonsFab,
    Selection,
    Chips,
    Sliders,
    LoadingIndicators,
    Progress,
    TextFields,
    Search,
    Pickers,
    Tabs,
    Navigation,
    AppBars,
    Cards,
    Lists,
    Dialogs,
    Sheets,
    Overlays,
    Toolbars,
    FabMenu,
    Carousel,
    Motion,
}

pub(crate) struct PageMeta {
    pub(crate) id: PageId,

    pub(crate) section: &'static str,

    pub(crate) title: &'static str,
    pub(crate) icon: &'static str,
}

const PAGES: [PageMeta; 23] = [
    PageMeta {
        id: PageId::Buttons,
        section: "Buttons",
        title: "Buttons",
        icon: "star",
    },
    PageMeta {
        id: PageId::ButtonsExtended,
        section: "Buttons",
        title: "Groups & split",
        icon: "more_vert",
    },
    PageMeta {
        id: PageId::IconButtonsFab,
        section: "Buttons",
        title: "Icon buttons & FAB",
        icon: "favorite",
    },
    PageMeta {
        id: PageId::Selection,
        section: "Selection",
        title: "Selection",
        icon: "check",
    },
    PageMeta {
        id: PageId::Chips,
        section: "Selection",
        title: "Chips",
        icon: "label",
    },
    PageMeta {
        id: PageId::Sliders,
        section: "Sliders",
        title: "Sliders",
        icon: "tune",
    },
    PageMeta {
        id: PageId::LoadingIndicators,
        section: "Loading & progress",
        title: "Loading indicators",
        icon: "progress_activity",
    },
    PageMeta {
        id: PageId::Progress,
        section: "Loading & progress",
        title: "Progress",
        icon: "autorenew",
    },
    PageMeta {
        id: PageId::TextFields,
        section: "Text fields",
        title: "Text fields",
        icon: "edit",
    },
    PageMeta {
        id: PageId::Search,
        section: "Search",
        title: "Search",
        icon: "search",
    },
    PageMeta {
        id: PageId::Pickers,
        section: "Date & time pickers",
        title: "Date & time",
        icon: "calendar_month",
    },
    PageMeta {
        id: PageId::Tabs,
        section: "Tabs",
        title: "Tabs",
        icon: "tab",
    },
    PageMeta {
        id: PageId::Navigation,
        section: "Navigation",
        title: "Navigation",
        icon: "menu",
    },
    PageMeta {
        id: PageId::AppBars,
        section: "App bars",
        title: "App bars",
        icon: "home",
    },
    PageMeta {
        id: PageId::Cards,
        section: "Cards",
        title: "Cards",
        icon: "dashboard",
    },
    PageMeta {
        id: PageId::Lists,
        section: "Lists",
        title: "Lists",
        icon: "list",
    },
    PageMeta {
        id: PageId::Dialogs,
        section: "Dialogs",
        title: "Dialogs",
        icon: "delete",
    },
    PageMeta {
        id: PageId::Sheets,
        section: "Sheets",
        title: "Bottom sheet",
        icon: "expand_less",
    },
    PageMeta {
        id: PageId::Overlays,
        section: "Overlays",
        title: "Overlays",
        icon: "notifications",
    },
    PageMeta {
        id: PageId::Toolbars,
        section: "Toolbars",
        title: "Toolbars",
        icon: "build",
    },
    PageMeta {
        id: PageId::FabMenu,
        section: "Toolbars",
        title: "FAB menu",
        icon: "add",
    },
    PageMeta {
        id: PageId::Carousel,
        section: "Additional",
        title: "Carousel",
        icon: "view_carousel",
    },
    PageMeta {
        id: PageId::Motion,
        section: "Additional",
        title: "Motion",
        icon: "animation",
    },
];

struct Catalog {
    dark: bool,
    seed: u32,
    page: PageId,
    dialog_open: bool,
    wired: bool,
    dark_switch: Entity<SwitchState>,
    pages: Pages,
    transition: Animatable,
    transition_direction: f32,
    driver: AnimationDriver,
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
            transition: Animatable::new(1.0, 1.0e-3),
            transition_direction: 1.,
            driver: AnimationDriver::default(),
        }
    }

    fn navigate(&mut self, target: PageId, window: &mut Window, cx: &mut Context<Self>) {
        if self.page == target {
            return;
        }
        let position = |id: PageId| PAGES.iter().position(|meta| meta.id == id).unwrap_or(0);
        self.transition_direction = if position(target) >= position(self.page) {
            1.
        } else {
            -1.
        };
        self.page = target;
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        self.transition.stop();
        self.transition.snap_to(0.0);
        self.transition.animate_to(1.0, &spec, Instant::now());
        if self.transition.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    fn apply_theme(&self, cx: &mut App) {
        let mode = if self.dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Theme::set(cx, Theme::from_seed(self.seed, mode, Profile::Baseline2021));
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

impl AnimatedComponent for Catalog {
    fn step(&mut self, now: Instant) -> bool {
        self.transition.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for Catalog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.wire(cx);
        #[cfg(target_os = "macos")]
        hide_maximize_buttons();

        if self.transition.is_running() {
            self.schedule_next(window, cx);
        }

        let this = cx.entity();
        let colors = *cx.theme().colors();
        let typography = *cx.theme().typography();
        let font_family = cx.theme().font_family().clone();
        let page = self.page;
        let dialog_open = self.dialog_open;
        let transition = self.transition.value() as f32;
        let transition_offset = px(self.transition_direction * (1. - transition) * 24.);

        let page_view: AnyView = match self.page {
            PageId::Buttons => self.pages.buttons.clone().into(),
            PageId::ButtonsExtended => self.pages.buttons_extended.clone().into(),
            PageId::IconButtonsFab => self.pages.icon_buttons_fab.clone().into(),
            PageId::Selection => self.pages.selection.clone().into(),
            PageId::Chips => self.pages.chips.clone().into(),
            PageId::Sliders => self.pages.sliders.clone().into(),
            PageId::LoadingIndicators => self.pages.loading_indicators.clone().into(),
            PageId::Progress => self.pages.progress.clone().into(),
            PageId::TextFields => self.pages.text_fields.clone().into(),
            PageId::Search => self.pages.search.clone().into(),
            PageId::Pickers => self.pages.pickers.clone().into(),
            PageId::Tabs => self.pages.tabs.clone().into(),
            PageId::Navigation => self.pages.navigation.clone().into(),
            PageId::AppBars => self.pages.app_bars.clone().into(),
            PageId::Cards => self.pages.cards.clone().into(),
            PageId::Lists => self.pages.lists.clone().into(),
            PageId::Dialogs => self.pages.dialogs.clone().into(),
            PageId::Sheets => self.pages.sheets.clone().into(),
            PageId::Overlays => self.pages.overlays.clone().into(),
            PageId::Toolbars => self.pages.toolbars.clone().into(),
            PageId::FabMenu => self.pages.fab_menu.clone().into(),
            PageId::Carousel => self.pages.carousel.clone().into(),
            PageId::Motion => self.pages.motion.clone().into(),
        };
        let meta = &PAGES[PAGES.iter().position(|p| p.id == page).unwrap_or(0)];

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
                    .children(
                        PAGES
                            .iter()
                            .enumerate()
                            .filter_map(|(index, meta)| {
                                let first_of_section =
                                    index == 0 || PAGES[index - 1].section != meta.section;
                                first_of_section.then_some((index, meta.section))
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                            .flat_map(|(_section_index, section)| {
                                let header = typography
                                    .label_medium
                                    .apply(div())
                                    .pt(px(16.))
                                    .pb(px(8.))
                                    .px(px(16.))
                                    .text_color(colors.on_surface_variant)
                                    .child(section)
                                    .into_any_element();
                                let items = PAGES
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, meta)| meta.section == section)
                                    .map(|(index, meta)| {
                                        let selected = meta.id == page;
                                        let target = meta.id;
                                        let color = if selected {
                                            colors.on_secondary_container
                                        } else {
                                            colors.on_surface
                                        };
                                        div()
                                            .id(("catalog-page", index))
                                            .focusable()
                                            .tab_stop(true)
                                            .w_full()
                                            .h(px(56.))
                                            .mb(px(2.))
                                            .px(px(16.))
                                            .rounded(px(28.))
                                            .flex()
                                            .items_center()
                                            .gap(px(12.))
                                            .cursor_pointer()
                                            .focus(|style| style.bg(colors.secondary_container))
                                            .text_color(color)
                                            .when(selected, |element| {
                                                element.bg(colors.secondary_container)
                                            })
                                            .when(!selected, |element| {
                                                element.hover(move |style| {
                                                    style.bg(colors.surface_container_high)
                                                })
                                            })
                                            .child(Icon::new(meta.icon).size(px(20.)).color(color))
                                            .child(
                                                typography
                                                    .label_large
                                                    .apply(div())
                                                    .child(meta.title),
                                            )
                                            .on_key_down(cx.listener(
                                                move |this, event: &KeyDownEvent, window, cx| {
                                                    if matches!(
                                                        event.keystroke.key.as_str(),
                                                        "enter" | "space"
                                                    ) {
                                                        this.navigate(target, window, cx);
                                                        cx.stop_propagation();
                                                    }
                                                },
                                            ))
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.navigate(target, window, cx);
                                            }))
                                            .into_any_element()
                                    })
                                    .collect::<Vec<_>>();
                                std::iter::once(header).chain(items)
                            }),
                    ),
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
                    .child(
                        div().w_full().max_w(px(1120.)).mx_auto().child(
                            div()
                                .relative()
                                .opacity(transition.clamp(0., 1.))
                                .left(transition_offset)
                                .child(page_view),
                        ),
                    ),
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
            .child(titlebar::CustomTitleBar)
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .overflow_hidden()
                    .child(rail_pane)
                    .child(content),
            )
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

fn main() {
    gpui::Application::new()
        .with_assets(
            Md3Assets::new().with_icon_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/symbols_icons")),
        )
        .run(|cx: &mut App| {
            material3_gpui::init(cx);

            let initial = size(px(996.), px(621.));
            let min_size = size(px(996.), px(621.));
            let bounds = Bounds::centered(None, initial, cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("material3-gpui".into()),

                        appears_transparent: true,

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
