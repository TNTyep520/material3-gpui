//! 组件样式（Component Styles）。
//!
//! 架构对齐 [m3fx](https://github.com/Glavo/m3fx) 的
//! `styles/controls/*.css`（Apache-2.0，© 2026 Glavo）：每个控件的
//! 全部可样式化值（几何、圆角、内边距、各状态颜色、字型、elevation）
//! 集中在一个 `XxxStyle` 结构体中，默认值由 [`TokenSet`](crate::theme::TokenSet)
//! 推导（等价于 m3fx 的 UA stylesheet），组件 render 只消费 Style。
//!
//! 实例级覆盖（等价于 m3fx 的 styleable properties / 用户 CSS）：
//! 组件 builder 提供 `.style(|s| { s.container_color = …; })`。
//!
//! ```ignore
//! Button::new("hi", "Hi")
//!     .style(|s: &mut ButtonStyle| s.container_color = Some(colors.tertiary))
//!     .build(cx)
//! ```

#[path = "styles/Button.rs"]
pub mod button;
#[path = "styles/Chip.rs"]
pub mod chip;
#[path = "styles/Container.rs"]
pub mod container;
#[path = "styles/Fab.rs"]
pub mod fab;
#[path = "styles/Input.rs"]
pub mod input;
#[path = "styles/Navigation.rs"]
pub mod navigation;
#[path = "styles/Overlay.rs"]
pub mod overlay;
#[path = "styles/Progress.rs"]
pub mod progress;
#[path = "styles/SegmentedButton.rs"]
pub mod segmented_button;
#[path = "styles/Selection.rs"]
pub mod selection;
#[path = "styles/Tabs.rs"]
pub mod tabs;

pub use button::{ButtonStyle, IconButtonStyle};
pub use chip::ChipStyle;
pub use container::{CardStyle, DialogStyle, DividerStyle, ListItemStyle};
pub use fab::FabStyle;
pub use input::TextFieldStyle;
pub use navigation::{
    NavigationBarStyle, NavigationDrawerStyle, NavigationItemStyle, NavigationRailStyle,
    TopAppBarStyle,
};
pub use overlay::{MenuStyle, SnackbarStyle, TooltipStyle};
pub use progress::{CircularProgressStyle, LinearProgressStyle};
pub use segmented_button::SegmentedButtonStyle;
pub use selection::{CheckboxStyle, RadioStyle, SliderStyle, SwitchStyle};
pub use tabs::TabBarStyle;
