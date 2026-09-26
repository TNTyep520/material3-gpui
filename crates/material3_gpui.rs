//! # material3-gpui
//!
//! 基于 [gpui](https://github.com/zed-industries/zed/tree/main/crates/gpui) 的
//! Material Design 3 组件库。令牌与运动系统移植自
//! [m3fx](https://github.com/Glavo/m3fx)（Apache-2.0），并对齐
//! [material-web](https://github.com/material-components/material-web) 的组件规格。
//!
//! ## 快速开始
//!
//! ```ignore
//! use gpui::*;
//! use gpui_platform::application;
//! use material3_gpui::prelude::*;
//!
//! fn main() {
//!     application()
//!         .with_assets(Md3Assets)          // 安装内嵌图标资源
//!         .run(|cx| {
//!             material3_gpui::init(cx);          // 安装默认（亮色）主题
//!             // 或者：自定义种子色动态色
//!             // Theme::set(cx, Theme::from_seed(0x006A6A, ThemeMode::Light, Profile::Baseline2021));
//!             cx.open_window(WindowOptions::default(), |_, cx| {
//!                 cx.new(|_| MyApp)
//!             }).unwrap();
//!         });
//! }
//!
//! impl Render for MyApp {
//!     fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let theme = cx.theme();
//!         div()
//!             .size_full()
//!             .bg(theme.colors().surface)
//!             .child(Button::new("hi", "Hello MD3").on_click(|_, _, _| {}).build(cx))
//!     }
//! }
//! ```

#[path = "Assets.rs"]
pub mod assets;
pub mod components;
#[path = "Fonts.rs"]
pub mod fonts;
#[path = "Icon.rs"]
pub mod icon;
#[path = "Interaction.rs"]
pub mod interaction;
pub mod motion;
#[path = "Overlay.rs"]
pub mod overlay;
pub mod styles;
pub mod theme;
pub mod tokens;

pub use assets::Md3Assets;
pub use components::*;
pub use icon::{ICON_FONT_FAMILY, Icon, IconName};
pub use theme::{ActiveTheme, Theme, ThemeMode};

use gpui::App;

/// 注册内嵌字体并安装默认（亮色）主题。若已有主题则不覆盖。
pub fn init(cx: &mut App) {
    if let Err(err) = fonts::install(cx) {
        eprintln!("material3-gpui: failed to register embedded fonts: {err}");
    }
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::light());
    }
}

/// 常用导出集合
pub mod prelude {
    pub use crate::assets::Md3Assets;
    pub use crate::components::*;
    pub use crate::fonts::TEXT_FONT_FAMILY;
    pub use crate::icon::ICON_FONT_FAMILY;
    pub use crate::icon::{Icon, IconName};
    pub use crate::motion::{
        Animatable, AnimatedComponent, AnimationDriver, Easing, MotionRole, MotionScheme,
        MotionSpec, SpringParameters,
    };
    pub use crate::overlay::{
        MenuItem, MenuState, OverlayHostState, OverlayRegistry, Snackbar, close_menu,
        close_tooltip, host, show_menu, show_snackbar, show_tooltip,
    };
    #[cfg(feature = "dynamic-color")]
    pub use crate::theme::color_scheme_from_seed;
    pub use crate::theme::{
        ActiveTheme, ColorScheme, ComponentTokens, Density, Elevation, ElevationTokens, Profile,
        Shapes, StateLayerTokens, Theme, ThemeMode, TokenSet, TokenSetBuilder, TypeScale,
        TypeStyle,
    };
}
