//! MD3 导航组件族（对应 material-web 的 navigation-bar / navigation-rail /
//! navigation-drawer 与 m3fx 的 `M3NavigationBar` / `M3NavigationRail` /
//! `M3NavigationDrawer` / `M3TopAppBar`）。
//!
//! 指示条滑动动画对齐 m3fx（Apache-2.0，© 2026 Glavo）：active indicator
//! 以 fastSpatial 弹簧在项间滑动。
//!
//! ```ignore
//! NavigationBar::new("nav")
//!     .item(NavigationItemSpec::new("Home", IconName::Home))
//!     .item(NavigationItemSpec::new("Search", IconName::Search))
//!     .selected(0)
//!     .on_change(|ix, _, _| {})
//!     .build(cx)   // -> Entity<NavigationBarState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 可选中导航族(Bar / Rail / Drawer)的内部状态接口:
/// 点击项时由组件内部完成选中,状态真正变化才触发 on_change。
trait NavSelection: 'static {
    /// 当前选中下标。
    fn selected_index(&self) -> usize;
    /// 切换选中项(指示条动画由各实现自带)。
    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>)
    where
        Self: Sized;
}

impl NavSelection for NavigationBarState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationRailState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationDrawerState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

/// 导航项描述。
#[derive(Clone)]
pub struct NavigationItemSpec {
    /// 标签。
    pub label: SharedString,
    /// 图标。
    pub icon: Option<IconName>,
    /// 可选徽标文字（右上天角）。
    pub badge: Option<SharedString>,
    /// 是否允许点击与选中。
    pub enabled: bool,
    on_click: Option<ItemClickHandler>,
}

type ItemClickHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// AndroidX NavigationBarItem 对应的条目配置。
pub type NavigationBarItem = NavigationItemSpec;
/// AndroidX NavigationRailItem 对应的条目配置。
pub type NavigationRailItem = NavigationItemSpec;
/// AndroidX NavigationDrawerItem 对应的条目配置。
pub type NavigationDrawerItem = NavigationItemSpec;

impl NavigationItemSpec {
    /// 创建导航项。
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            label: label.into(),
            icon: Some(icon),
            badge: None,
            enabled: true,
            on_click: None,
        }
    }

    /// 设置徽标文字。
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// 设置条目是否可以响应点击。
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// 设置条目级点击回调；它在容器的选中回调之后执行。
    pub fn on_click(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

/// 导航项内容渲染（Bar / Rail / Drawer 共用）。
#[allow(clippy::too_many_arguments)]
fn navigation_item<T: NavSelection>(
    id: impl Into<ElementId>,
    spec: &NavigationItemSpec,
    selected: bool,
    horizontal: bool,
    indicator_offset: Option<f32>,
    item: &NavigationItemStyle,
    entity: &Entity<T>,
    on_change: Option<&ChangeHandler>,
    ix: usize,
) -> gpui::Stateful<gpui::Div> {
    let icon_color = if selected {
        item.selected_icon_color
    } else {
        item.unselected_icon_color
    };
    let label_color = if selected {
        item.selected_label_color
    } else {
        item.unselected_label_color
    };
    let (indicator_w, indicator_h) = item.indicator_size;
    let hover = item.hover_opacity;
    let pressed = item.pressed_opacity;
    let indicator_color = item.indicator_color;
    let click_entity = entity.clone();
    let on_change = on_change.cloned();
    let item_on_click = spec.on_click.clone();
    let enabled = spec.enabled;
    let interaction_group = SharedString::from(format!("navigation-{:?}-{ix}", entity.entity_id()));

    // 指示条胶囊：仅选中项渲染，锚定整项宽度的中心（relative 0.5），
    // 按 (弹簧位置 − 项下标) 的偏移跨项滑动；越出部分由容器裁剪
    let pill = indicator_offset.map(|offset| {
        div()
            .absolute()
            .left(relative(0.5 + offset))
            .ml(-indicator_w / 2.0)
            .top(px(0.))
            .w(indicator_w)
            .h(indicator_h)
            .flex_none()
            .rounded(item.indicator_radius)
            .bg(indicator_color)
    });

    let base = div()
        .id(id)
        .group(interaction_group.clone())
        .relative()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(item_gap_from(horizontal)))
        .when(enabled, |el| el.cursor_pointer())
        .text_color(label_color)
        // 指示条胶囊在下、图标在上;图标行铺满整项宽度,
        // 使胶囊的 relative 定位以整项为基准
        .child(
            div()
                .relative()
                .w_full()
                .h(indicator_h)
                .flex()
                .items_center()
                .justify_center()
                .when_some(pill, |el, pill| el.child(pill))
                .child(
                    div()
                        .id("navigation-state-layer")
                        .absolute()
                        .left(relative(0.5))
                        .ml(-indicator_w / 2.0)
                        .top(px(0.))
                        .w(indicator_w)
                        .h(indicator_h)
                        .rounded(item.indicator_radius)
                        .group_hover(interaction_group.clone(), move |style| {
                            style.bg(icon_color.opacity(hover))
                        })
                        .group_active(interaction_group, move |style| {
                            style.bg(icon_color.opacity(pressed))
                        }),
                )
                .when_some(spec.icon, |el, icon| {
                    el.child(Icon::new(icon).size(item.icon_size).color(icon_color))
                }),
        )
        // 点击:组件内部先完成选中(指示条滑动),
        // 状态真正变化才触发一次 on_change(未设置回调时仅内部切换)
        .when(enabled, |el| {
            el.on_click(move |_, window, cx| {
                click_entity.update(cx, |state, cx| {
                    let changed = ix != state.selected_index();
                    state.select_index(ix, window, cx);
                    if changed && let Some(handler) = on_change.clone() {
                        handler(ix, window, cx);
                    }
                });
                if let Some(handler) = &item_on_click {
                    handler(window, cx);
                }
            })
        });

    let base = item.label.apply(base);
    base.child(spec.label.clone())
}

fn item_gap_from(horizontal: bool) -> f32 {
    if horizontal {
        16. // rail: 图标区与标签间距
    } else {
        4. // bar: 紧凑
    }
}

/// MD3 底部导航栏。
pub struct NavigationBar {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
}

/// 底部导航栏的有状态部分。
pub struct NavigationBarState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationBar {
    /// 创建导航栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
        }
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    /// 批量追加。
    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationBarState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationBarState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationBarState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中项（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for NavigationBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let bar = NavigationBarStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let indicator_pos = self.indicator.value() as f32;
        let selected = self.selected;
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w_full()
            .h(bar.height)
            .flex()
            .flex_none()
            .overflow_hidden()
            .bg(bar.container_color)
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;
                // 指示条画在选中项内部，弹簧进度做相对偏移
                let offset = if is_selected {
                    Some(indicator_pos - ix as f32)
                } else {
                    None
                };
                let item_el = navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    false,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                );
                item_el
                    .flex_1()
                    .h_full()
                    .justify_center()
                    .px(bar.item_padding)
            }))
    }
}

/// MD3 导航侧栏（Rail）。
pub struct NavigationRail {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
}

use crate::components::fab::FabState;

/// 导航侧栏的有状态部分。
pub struct NavigationRailState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationRail {
    /// 创建导航侧栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
            header: None,
        }
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    /// 批量追加。
    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 顶部槽位内容（通常为 FAB）。
    pub fn header(mut self, header: Entity<FabState>) -> Self {
        self.header = Some(header);
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationRailState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationRailState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            header: self.header,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationRailState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中项（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for NavigationRailState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationRailState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let rail = NavigationRailStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let selected = self.selected;
        let header = self.header.clone();
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w(rail.width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .items_center()
            .pt(rail.top_padding)
            .gap(rail.item_gap)
            .bg(rail.container_color)
            .overflow_y_scroll()
            .children(header.map(|h| div().pb(px(16.)).child(h)))
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;
                // Rail 为纵向列表：指示条在选中项内居中（无跨项滑动）
                let offset = if is_selected { Some(0.0) } else { None };
                navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    true,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                )
                .w_full()
                .py(rail.item_gap)
            }))
    }
}

/// 导航抽屉内容项（分组内条目或分组标题）。
pub enum DrawerEntry {
    /// 分组标题。
    Section(SharedString),
    /// 导航项。
    Item(NavigationItemSpec),
}

/// MD3 导航抽屉。
pub struct NavigationDrawer {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
}

/// 导航抽屉的有状态部分。
pub struct NavigationDrawerState {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
    driver: AnimationDriver,
}

impl NavigationDrawer {
    /// 创建导航抽屉构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
            selected: 0,
            modal: false,
            on_change: None,
        }
    }

    /// 追加条目。
    pub fn entry(mut self, entry: DrawerEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.entries.push(DrawerEntry::Item(item));
        self
    }

    /// 追加分组标题。
    pub fn section(mut self, title: impl Into<SharedString>) -> Self {
        self.entries.push(DrawerEntry::Section(title.into()));
        self
    }

    /// 初始选中下标（按条目中的 Item 序号）。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// modal 变体（带阴影，配 scrim 使用）。
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationDrawerState> {
        cx.new(|_| NavigationDrawerState {
            id: self.id,
            entries: self.entries,
            selected: self.selected,
            modal: self.modal,
            on_change: self.on_change,
            driver: AnimationDriver::default(),
        })
    }
}

macro_rules! navigation_drawer_variant {
    ($name:ident, $modal:expr) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的导航抽屉。")]
        pub struct $name(NavigationDrawer);

        impl $name {
            /// 创建指定呈现方式的导航抽屉。
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(NavigationDrawer::new(id).modal($modal))
            }

            /// 添加导航项。
            pub fn item(mut self, item: NavigationDrawerItem) -> Self {
                self.0 = self.0.item(item);
                self
            }

            /// 添加分组标题。
            pub fn section(mut self, title: impl Into<SharedString>) -> Self {
                self.0 = self.0.section(title);
                self
            }

            /// 设置选中项下标。
            pub fn selected(mut self, index: usize) -> Self {
                self.0 = self.0.selected(index);
                self
            }

            /// 设置选中项变化回调。
            pub fn on_change(
                mut self,
                handler: impl Fn(usize, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_change(handler);
                self
            }

            /// 构建导航抽屉实体。
            pub fn build(self, cx: &mut App) -> Entity<NavigationDrawerState> {
                self.0.build(cx)
            }
        }
    };
}

navigation_drawer_variant!(ModalNavigationDrawer, true);
navigation_drawer_variant!(PermanentNavigationDrawer, false);
navigation_drawer_variant!(DismissibleNavigationDrawer, false);

/// AndroidX ModalDrawerSheet 对应的模态抽屉面板。
pub type ModalDrawerSheet = ModalNavigationDrawer;
/// AndroidX PermanentDrawerSheet 对应的常驻抽屉面板。
pub type PermanentDrawerSheet = PermanentNavigationDrawer;
/// AndroidX DismissibleDrawerSheet 对应的可收起抽屉面板。
pub type DismissibleDrawerSheet = DismissibleNavigationDrawer;

impl AnimatedComponent for NavigationDrawerState {
    fn step(&mut self, _now: Instant) -> bool {
        false
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl NavigationDrawerState {
    /// 当前选中下标（按条目中的 Item 序号）。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中条目（按 Item 序号；选中高亮即时更新）。
    pub fn select(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self
            .entries
            .iter()
            .filter(|entry| matches!(entry, DrawerEntry::Item(_)))
            .count();
        if index >= item_count || index == self.selected {
            return;
        }
        self.selected = index;
        cx.notify();
    }
}

impl Render for NavigationDrawerState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let drawer = NavigationDrawerStyle::resolve(theme.token_set(), self.modal);
        let item = NavigationItemStyle::resolve(theme.token_set());
        let width = drawer.drawer_width(self.modal);
        let entity = cx.entity();
        let mut item_ix = 0usize;

        let mut column = div()
            .id(self.id.clone())
            .w(width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .gap(px(4.))
            .p(drawer.padding)
            .bg(drawer.container_color)
            .overflow_y_scroll()
            .when(self.modal, |el| {
                el.shadow(drawer.modal_elevation.shadows(drawer.shadow_color))
            });

        for entry in self.entries.iter() {
            match entry {
                DrawerEntry::Section(title) => {
                    let el = drawer
                        .section_header
                        .apply(div())
                        .px(drawer.item_horizontal_padding)
                        .pt(px(16.))
                        .text_color(drawer.section_header_color)
                        .child(title.clone());
                    column = column.child(el);
                }
                DrawerEntry::Item(spec) => {
                    let ix = item_ix;
                    let is_selected = ix == self.selected;
                    let on_change = self.on_change.clone();
                    let click_entity = entity.clone();
                    let icon_color = if is_selected {
                        item.selected_icon_color
                    } else {
                        item.unselected_icon_color
                    };
                    let label_color = if is_selected {
                        item.selected_label_color
                    } else {
                        item.unselected_label_color
                    };
                    let hover = item.hover_opacity;
                    let pressed = item.pressed_opacity;
                    // 抽屉条目为横向行式布局:[图标 24][标签][可选徽标]
                    let el = div()
                        .id((SharedString::from(format!("{}-item", self.id)), ix))
                        .w_full()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.))
                        .h(px(56.))
                        .rounded(item.indicator_radius)
                        .px(drawer.item_horizontal_padding)
                        .cursor_pointer()
                        .text_color(label_color)
                        .when(is_selected, |el| el.bg(item.indicator_color))
                        .when(!is_selected, |el| {
                            el.hover(move |s| s.bg(icon_color.opacity(hover)))
                        })
                        .active(move |s| s.bg(icon_color.opacity(pressed)))
                        .when_some(spec.icon, |el, icon| {
                            el.child(
                                div()
                                    .flex_none()
                                    .child(Icon::new(icon).size(item.icon_size).color(icon_color)),
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(item.label.apply(div()).child(spec.label.clone())),
                        )
                        .when_some(spec.badge.clone(), |el, badge| {
                            el.child(
                                crate::components::Badge::new((
                                    SharedString::from(format!("{}-badge", self.id)),
                                    ix,
                                ))
                                .label(badge),
                            )
                        })
                        .on_click(move |_, window, cx| {
                            click_entity.update(cx, |state, cx| {
                                let changed = ix != state.selected;
                                state.select(ix, window, cx);
                                if changed && let Some(handler) = on_change.clone() {
                                    handler(ix, window, cx);
                                }
                            })
                        });
                    column = column.child(el);
                    item_ix += 1;
                }
            }
        }
        column
    }
}

pub use appearance::{
    NavigationBarStyle, NavigationDrawerStyle, NavigationItemStyle, NavigationRailStyle,
};

mod appearance {
    use crate::theme::TokenSet;
    use gpui::{Hsla, Pixels, px};
    /// 导航项（NavigationBar / Rail / Drawer 共用）样式。
    #[derive(Clone, Copy, Debug)]
    pub struct NavigationItemStyle {
        /// 指示条胶囊宽/高。
        pub indicator_size: (Pixels, Pixels),
        /// 指示条圆角。
        pub indicator_radius: Pixels,
        /// 选中指示条色。
        pub indicator_color: Hsla,
        /// 选中项图标色。
        pub selected_icon_color: Hsla,
        /// 未选中项图标色。
        pub unselected_icon_color: Hsla,
        /// 选中项标签色。
        pub selected_label_color: Hsla,
        /// 未选中项标签色。
        pub unselected_label_color: Hsla,
        /// 图标尺寸。
        pub icon_size: Pixels,
        /// 标签字型。
        pub label: crate::theme::TypeStyle,
        /// hover 状态层不透明度。
        pub hover_opacity: f32,
        /// 按压状态层不透明度。
        pub pressed_opacity: f32,
    }
    impl NavigationItemStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            let colors = &tokens.colors;
            Self {
                indicator_size: (px(64.), px(32.)),
                indicator_radius: tokens.shapes.full,
                indicator_color: colors.secondary_container,
                selected_icon_color: colors.on_secondary_container,
                unselected_icon_color: colors.on_surface_variant,
                selected_label_color: colors.on_surface,
                unselected_label_color: colors.on_surface_variant,
                icon_size: px(24.),
                label: tokens.typography.label_medium,
                hover_opacity: crate::theme::HOVER_OPACITY,
                pressed_opacity: crate::theme::PRESSED_OPACITY,
            }
        }
    }
    /// NavigationBar 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct NavigationBarStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 高度。
        pub height: Pixels,
        /// 项间距（项内部 图标区/标签 间距）。
        pub item_gap: Pixels,
        /// 项内边距。
        pub item_padding: Pixels,
    }
    impl NavigationBarStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            Self {
                container_color: tokens.colors.surface_container,
                height: px(80.),
                item_gap: px(4.),
                item_padding: px(8.),
            }
        }
    }
    /// NavigationRail 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct NavigationRailStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 宽度。
        pub width: Pixels,
        /// 项间距。
        pub item_gap: Pixels,
        /// 顶部内边距。
        pub top_padding: Pixels,
    }
    impl NavigationRailStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet) -> Self {
            Self {
                container_color: tokens.colors.surface_container,
                width: px(80.),
                item_gap: px(4.),
                top_padding: px(8.),
            }
        }
    }
    /// NavigationDrawer 样式。
    #[derive(Clone, Copy, Debug)]
    pub struct NavigationDrawerStyle {
        /// 容器色。
        pub container_color: Hsla,
        /// 标准宽度。
        pub width: Pixels,
        /// modal 宽度。
        pub modal_width: Pixels,
        /// 分组标题色。
        pub section_header_color: Hsla,
        /// 内边距。
        pub padding: Pixels,
        /// 项水平内边距。
        pub item_horizontal_padding: Pixels,
        /// 阴影颜色。
        pub shadow_color: Hsla,
        /// modal 阴影等级。
        pub modal_elevation: crate::theme::Elevation,
        /// 分组标题字型。
        pub section_header: crate::theme::TypeStyle,
    }
    impl NavigationDrawerStyle {
        /// 由令牌推导默认样式。
        pub fn resolve(tokens: &TokenSet, _modal: bool) -> Self {
            Self {
                container_color: tokens.colors.surface_container_low,
                width: px(360.),
                modal_width: px(360.),
                section_header_color: tokens.colors.on_surface_variant,
                padding: px(12.),
                item_horizontal_padding: px(12.),
                shadow_color: tokens.colors.shadow,
                modal_elevation: crate::theme::Elevation::Level1,
                section_header: tokens.typography.title_small,
            }
        }

        /// 当前宽度。
        pub fn drawer_width(&self, modal: bool) -> Pixels {
            if modal { self.modal_width } else { self.width }
        }
    }
}
