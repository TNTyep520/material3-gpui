> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.

# material3-gpui

基于 [GPUI](https://docs.rs/gpui)（Zed 的 GPU 加速 UI 框架）的 **Material Design 3** 组件库，并已移植 **Material Design 3 Expressive** 的核心组件。纯 gpui 实现，无中间层渲染依赖。

- 令牌（tokens）、运动（motion）与主题架构移植自 [m3fx](https://github.com/Glavo/m3fx)（Apache-2.0）
- Expressive 组件（加载指示器、波浪进度条等）对齐 [androidx compose material3](https://github.com/androidx/androidx/tree/androidx-main/compose/material3/material3) 的同名组件；组件令牌值直接取自其 `Tokens.kt`
- 动态色使用 `mcu-*` 系列算法库（material-color-utilities 的 Rust 实现，HCT 色彩空间）

## 特性

- **令牌系统**（`Profile` × `TokenSet`）
  - `TokenSet`：颜色 / 字体排印 / 形状 / 阴影 / 运动 / 状态层 / 组件令牌组，支持 builder 级覆盖
  - `Profile::Baseline2021`：基线形状、字阶与运动方案
  - 动态色：`Theme::from_seed(seed, mode, profile)`，完整复现 MD3 基线调色板
- **运动系统**（`material3_gpui::motion`）
  - `MotionScheme`：六种语义角色（fast/default/slow × effects/spatial），内置 `standard()` 与 `expressive()` 两套预设
  - 解析闭式阻尼弹簧求解器，支持保速度重定向的 `Animatable` 值
  - 13 条 MD3 缓动曲线（含三段式 emphasized 曲线），支持 `reduce_motion`
- **交互行为**：弹簧驱动的状态层（hover/press）、指针涟漪、开关 / 复选框 / 单选 / 标签指示器动画
- **窗口级弹层系统**（`material3_gpui::overlay`）：`OverlayHost` + `show_snackbar` / `show_menu` / `show_tooltip`
- **按需图标**（Iconify 风格）：`Icon` 按 Material Symbols Rounded SVG 名称渲染；`Md3Assets::with_icon_dir(dir)` 注册图标目录，运行时按需从磁盘读取，库本身不内嵌图标资源
- **样式层**（`material3_gpui::styles`）：每个组件一个 `XxxStyle` 结构（几何、形状、颜色、字型），默认值由令牌推导
- **MD3 Expressive 组件**：
  - `LoadingIndicator`：7 形状连续变形动画（expressive default spatial 缓动 + sin² 呼吸缩放），支持 `CONTAINED` 变体
  - `LinearWavyProgressIndicator` / `CircularWavyProgressIndicator`：确定性（相位 1λ/s 推进）与不确定性（1750ms 双段扫掠 / 6000ms 旋转扫掠）波浪进度
  - `Slider`：五档尺寸（XS/S/M/L/XL，轨道 16–96dp）、按压 handle 变细弹簧（4→2dp）、非拖动改值 FastSpatial 弹簧、离散刻度点、`centered` 变体、拖动反色 value indicator、M/L/XL 轨道内嵌图标
  - `FabMenu`：展开 / 收起弹簧动画（defaultSpatial / fastSpatial）的 FAB 菜单，内置 Add/Close 切换钮
  - `Carousel`：multi-browse 排布 + defaultSpatial 弹簧选中有宽度/透明度插值
  - `FloatingToolbar`：standard / vibrant 双配色浮动工具栏（横向 / 纵向，48dp 槽位）
  - `MediumFlexibleTopAppBar` / `LargeFlexibleTopAppBar`：可折叠弹性应用栏
  - `TypeScale::expressive()`：Expressive 字阶（display/headline 升 Medium，title/label 升 SemiBold）
- **基准 MD3 组件**：

  | 分类 | 组件 |
  |---|---|
  | 按钮 | `Button`（filled / tonal / elevated / outlined / text）、`ToggleButton`、`SplitButton`、`ButtonGroup`、`IconButton`（4 变体 + 开关型）、`Fab`（3 尺寸 / 4 配色 / 扩展型）、`FabMenu` |
  | 选择 | `Checkbox`、`RadioButton`、`Switch`、`Slider`（`SliderSize` 五档 / 离散 / 居中 / 值指示器）、`RangeSlider`、`Chip`（assist / filter / input / suggestion）、`SegmentedButton` |
  | 容器 | `Card`（elevated / filled / outlined）、`Dialog`、`List` / `ListItem`、`Divider`、`Scaffold` |
  | 导航 | `TabBar` / `TabRow`（primary / secondary / scrollable）、`TopAppBar`（5 变体 + flexible）、`BottomAppBar`、`NavigationBar`、`NavigationRail`、`NavigationDrawer`、`WideNavigationRail` |
  | 输入 | `TextField`（outlined、浮动标签、helper/error）、`SecureTextField`、`SearchBar`、`ExposedDropdownMenu` |
  | 弹层 | `Snackbar`、`Menu`、`Tooltip`（经窗口 `overlay::host`）、`ModalBottomSheet`、`BottomSheetScaffold` |
  | 进度 | `LinearProgress`、`CircularProgress`、波浪进度（见上）、`LoadingIndicator` |
  | 其他 | `Badge` / `BadgedBox`、`Carousel`、`DatePicker`、`TimePicker`、`TimeInput`、`SwipeToDismissBox`、`Scrollbar` |

## 快速开始

`Cargo.toml`（需要 Rust 1.85+，edition 2024）：

```toml
[dependencies]
gpui = "0.2"
material3-gpui = "0.1"
```

```rust
use gpui::*;
use material3_gpui::prelude::*;

struct MyApp;

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .size_full()
            .bg(theme.colors().surface)
            .flex()
            .items_center()
            .justify_center()
            .child(
                Button::new("hello", "Hello MD3")
                    .filled()
                    .leading_icon(IconName::Favorite)
                    .on_click(|_, _, _| println!("clicked!"))
                    .build(cx), // 实体组件：构建一次，长期持有句柄
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Md3Assets::new().with_icon_dir("icons")) // 注册图标目录（按需读取）
        .run(|cx: &mut App| {
            material3_gpui::init(cx); // 注册内嵌字体并安装默认（亮色）主题
            cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MyApp))
                .unwrap();
            cx.activate(true);
        });
}
```

## 运行组件展厅（catalog）

```bash
cargo run -p catalog
```

catalog 按 m3fx demo 的方式组织：侧栏 17 个分组、23 个组件页，每页为大标题 + 副标题 + 分组流式布局；页面切换带 shared-axis 过渡动画。包含 MD3 Expressive 演示（变形加载指示器、波浪进度、五档 Slider、FabMenu、弹簧动画 Motion 页等），支持亮 / 暗主题切换与种子色实时动态色。图标来自 `catalog/symbols_icons/`（4150 个 Material Symbols Rounded SVG，按需读取）。

> 首次构建需要编译 gpui 及其依赖，耗时较长。

## 主题定制

```rust
use material3_gpui::prelude::*;

// 基线亮 / 暗色（种子色 #6750A4）
Theme::set(cx, Theme::dark());

// 由种子色生成动态色
Theme::set(cx, Theme::from_seed(0x006A6A, ThemeMode::Light, Profile::Baseline2021));

// 经 builder 覆盖令牌组；切换到 Expressive 运动方案
let tokens = TokenSet::builder(Profile::Baseline2021, cx.theme().colors().clone())
    .with_motion(MotionScheme::expressive())
    .build();
let mut theme = Theme::light();
theme.set_token_set(tokens);
Theme::set(cx, theme);
```

组件在 `render` 时经 `cx.theme()` 读取全局主题（`theme.colors()`、`theme.typography()`、`theme.shapes()`、`theme.motion()` 等）。替换主题后调用 `cx.refresh_windows()` 触发重绘。

## 有状态组件

交互组件自带动画状态，属于实体（Entity），构建一次并长期持有：

```rust
// 创建（在 new() 或首次 render，切勿每帧构建）
let checkbox = Checkbox::new("agree").build(cx);
let switch = Switch::new("wifi").on_change(|checked, _, _| {}).build(cx);
let fab_menu = FabMenu::new("menu").action(fab).build(cx); // -> Entity<FabMenuState>

// 渲染：实体句柄即元素
div().child(checkbox.clone()).child(switch.clone())
```

简单容器（`Card`、`Divider`、`List` / `ListItem`、`Dialog`、进度指示器、`Carousel` 等）保持无状态 `RenderOnce`。

## 字体与图标

`material3_gpui::init()` 自动注册内嵌字体：

- **Roboto** Regular / Medium（Apache-2.0）——字阶中 title/label 系列使用 Medium(500)；600/700 字重就近回退到 Medium

图标为**按需加载**的 Material Symbols Rounded SVG（Apache-2.0）：

```rust
use material3_gpui::assets::Md3Assets;

// 注册一个或多个图标目录；Icon::new("bolt") 请求 md3-icons/bolt.svg 时
// 依次在这些目录中查找 {name}.svg 并从磁盘读取
Md3Assets::new().with_icon_dir("path/to/symbols_icons")
```

图标名即文件名（snake_case，如 `Icon::new("arrow_forward")`），缺失的图标渲染为空，不 panic。catalog 的 `catalog/symbols_icons/` 收录了完整 4150 个图标可直接复用。非拉丁文字回退到系统字体（MD3 推荐 Noto 系列）。

## 项目结构

```
crates/                  库源码（[lib] path = crates/material3_gpui.rs）
  components/            组件实现（一组件一文件；MD3E 组件见
                         LoadingIndicator / FabMenu / FloatingToolbar / Carousel 等）
  tokens/                组件令牌（对齐 compose material3 Tokens.kt）
  motion/                运动系统（弹簧 / 缓动 / 运动方案 standard + expressive）
  theme/                 主题（颜色方案 / 令牌集 / 字阶 baseline + expressive / 动态色）
  overlay（components）   窗口级弹层宿主
catalog/                 组件展厅应用（17 节 23 页，m3fx demo 布局）
  symbols_icons/         Material Symbols Rounded SVG × 4150（按需加载）
```

## 已知限制 / 路线图

- [ ] 键盘焦点环与无障碍（focus-visible 状态层、键盘激活）
- [ ] `TextField` 的 IME 组合输入；`Menu` 键盘导航
- [ ] SemiBold(600) / Bold(700) 字体未内嵌，Expressive 字阶高字重就近回退 Medium
- [ ] `Carousel` 滚动吸附；`LoadingIndicator` 的角特征级 morph（当前为角度采样近似）
- [ ] `SplitButton` / `ButtonGroup` 的 Expressive 内角形变
- [ ] 其余组件的样式层（`XxxStyle`）迁移

## 许可证

Apache-2.0。内嵌图标来自 [Material Symbols](https://fonts.google.com/icons)（Apache-2.0）。

### 致谢

- [m3fx](https://github.com/Glavo/m3fx)（Apache-2.0）—— 令牌、运动与交互行为
- [androidx compose material3](https://github.com/androidx/androidx/tree/androidx-main/compose/material3/material3)（Apache-2.0）—— 组件令牌值与 Expressive 组件规格
- [material color utilities](https://github.com/material-foundation/material-color-utilities)（经 `mcu-*` crate）—— 动态色算法
