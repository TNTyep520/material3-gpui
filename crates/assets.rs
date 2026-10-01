//! 内嵌资源源：Material Symbols Rounded SVG、进度弧及应用图标。
//! 使用 Md3Assets 注册资源，或通过 with_fallback 与应用自己的资源源组合。

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// `CircularProgress` 旋转弧 SVG 的资源路径。
pub const PROGRESS_ARC_SVG_PATH: &str = "md3-icons/progress_arc.svg";

/// Material 站点图标(蓝色圆角方块 + 白圆盘 + M 徽标)的资源路径,
/// 供自定义标题栏/关于页等以 `img()` 渲染。
pub const MATERIAL3_FAVICON_SVG_PATH: &str = "md3-icons/material3-favicon.svg";

/// 内部进度图形及应用标识，与 Rounded 图标使用不同资源路径。
static RESOURCES: &[(&str, &[u8])] = &[
    (
        "md3-icons/progress_arc.svg",
        include_bytes!("assets/progress_arc.svg").as_slice(),
    ),
    (
        "md3-icons/material3-favicon.svg",
        include_bytes!("assets/material3-favicon.svg").as_slice(),
    ),
];

/// material3-gpui 的内嵌资源源
pub struct Md3Assets;

impl Md3Assets {
    /// 与另一个 AssetSource 组合：md3 内部资源优先，其余路径回退到 `fallback`。
    pub fn with_fallback(fallback: impl AssetSource) -> CombinedAssets {
        CombinedAssets {
            fallback: Box::new(fallback),
        }
    }

    fn find(path: &str) -> Option<&'static [u8]> {
        RESOURCES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| *bytes)
            .or_else(|| {
                let name = path
                    .strip_prefix("md3-icons/materialsymbolsrounded/")?
                    .strip_suffix(".svg")?;
                rounded_resources().get(name).map(|svg| svg.as_bytes())
            })
    }

    fn paths(path: &str) -> Vec<SharedString> {
        RESOURCES
            .iter()
            .map(|(name, _)| SharedString::from(*name))
            .chain(rounded_resources().keys().map(|name| {
                SharedString::from(format!("md3-icons/materialsymbolsrounded/{name}.svg"))
            }))
            .filter(|name| name.starts_with(path))
            .collect()
    }
}

fn rounded_resources() -> &'static BTreeMap<&'static str, &'static str> {
    static SYMBOLS: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();
    SYMBOLS.get_or_init(|| {
        include_str!("assets/material-symbols-rounded.data")
            .lines()
            .filter_map(|line| line.split_once('\t'))
            .collect()
    })
}
impl AssetSource for Md3Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Md3Assets::find(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Md3Assets::paths(path))
    }
}

/// [`Md3Assets`] 与用户资源源的组合体
pub struct CombinedAssets {
    fallback: Box<dyn AssetSource>,
}

impl AssetSource for CombinedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = Md3Assets::find(path) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        self.fallback.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut out = Md3Assets::paths(path);
        out.extend(self.fallback.list(path)?);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::str::from_utf8;

    use anyhow::Result;
    use gpui::{AssetSource, SharedString};
    use usvg::{Options, Tree};

    use super::{Md3Assets, PROGRESS_ARC_SVG_PATH, rounded_resources};
    use crate::icon::{ALL_ICONS, ICON_COUNT, IconName};

    #[test]
    fn every_icon_has_a_matching_embedded_svg() -> Result<()> {
        assert_eq!(rounded_resources().len(), ICON_COUNT);
        assert_eq!(
            include_str!("assets/material-symbols-rounded.data")
                .lines()
                .count(),
            ICON_COUNT
        );
        let assets = Md3Assets;
        let listed = assets.list("md3-icons/materialsymbolsrounded/")?;
        assert_eq!(listed.len(), ICON_COUNT);
        for &icon in ALL_ICONS {
            let path = icon.path();
            let data = assets.load(&path)?.expect("embedded SVG");
            let svg = from_utf8(&data)?;
            let tree = Tree::from_data(&data, &Options::default())?;
            assert!(tree.size().width() > 0.);
            assert!(svg.starts_with("<svg "));
            assert!(svg.ends_with("</svg>"));
            assert!(listed.contains(&path));
        }
        assert!(assets.load(PROGRESS_ARC_SVG_PATH)?.is_some());
        assert!(
            assets
                .load("md3-icons/materialsymbolsrounded/not_an_icon.svg")?
                .is_none()
        );
        assert!(
            assets
                .load("md3-icons/materialsymbolsrounded/../home.svg")?
                .is_none()
        );
        assert!(assets.list("application/")?.is_empty());
        Ok(())
    }

    struct Fallback;

    impl AssetSource for Fallback {
        fn load(&self, _path: &str) -> Result<Option<Cow<'static, [u8]>>> {
            Ok(Some(Cow::Borrowed(b"fallback")))
        }

        fn list(&self, path: &str) -> Result<Vec<SharedString>> {
            Ok(["application/icon.svg"]
                .into_iter()
                .filter(|name| name.starts_with(path))
                .map(SharedString::from)
                .collect())
        }
    }

    #[test]
    fn combined_assets_preserve_priority_and_fallback() -> Result<()> {
        let assets = Md3Assets::with_fallback(Fallback);
        let home = assets.load(&IconName::Home.path())?.expect("home SVG");
        assert!(home.starts_with(b"<svg "));
        assert_eq!(
            assets.load("application/icon.svg")?.as_deref(),
            Some(b"fallback".as_slice())
        );
        assert_eq!(
            assets.list("md3-icons/materialsymbolsrounded/")?.len(),
            ICON_COUNT
        );
        assert_eq!(
            assets.list("application/")?,
            vec![SharedString::from("application/icon.svg")]
        );
        Ok(())
    }
}
