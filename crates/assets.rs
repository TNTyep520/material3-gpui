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

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;
use std::path::PathBuf;

pub const PROGRESS_ARC_SVG_PATH: &str = "md3-icons/progress_arc.svg";

static RESOURCES: &[(&str, &[u8])] = &[(
    "md3-icons/progress_arc.svg",
    include_bytes!("assets/progress_arc.svg").as_slice(),
)];

pub struct Md3Assets {
    icon_dirs: Vec<PathBuf>,
}

impl Default for Md3Assets {
    fn default() -> Self {
        Self::new()
    }
}

impl Md3Assets {
    pub fn new() -> Self {
        Self {
            icon_dirs: Vec::new(),
        }
    }

    pub fn with_icon_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.icon_dirs.push(dir.into());
        self
    }

    pub fn with_fallback(self, fallback: impl AssetSource + 'static) -> CombinedAssets {
        CombinedAssets {
            inner: self,
            fallback: Box::new(fallback),
        }
    }

    fn icon_file_name(path: &str) -> Option<&str> {
        let name = path.strip_prefix("md3-icons/")?.strip_suffix(".svg")?;
        if name.is_empty() || name.contains("..") || name.contains('/') || name.contains('\\') {
            return None;
        }
        Some(name)
    }

    fn find(&self, path: &str) -> Option<Cow<'static, [u8]>> {
        if let Some((_, bytes)) = RESOURCES.iter().find(|(name, _)| *name == path) {
            return Some(Cow::Borrowed(bytes));
        }
        let name = Self::icon_file_name(path)?;
        for dir in &self.icon_dirs {
            let candidate = dir.join(format!("{name}.svg"));
            match std::fs::read(&candidate) {
                Ok(bytes) => return Some(Cow::Owned(bytes)),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => {
                    eprintln!("material3-gpui: failed to read icon {candidate:?}: {err}");
                }
            }
        }
        None
    }

    fn paths(&self, prefix: &str) -> Vec<SharedString> {
        let mut out: Vec<SharedString> = RESOURCES
            .iter()
            .map(|(name, _)| SharedString::from(*name))
            .filter(|name| name.starts_with(prefix))
            .collect();
        for dir in &self.icon_dirs {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let Some(name) = file_name.to_str() else {
                    continue;
                };
                if !name.ends_with(".svg") {
                    continue;
                }
                let path = SharedString::from(format!("md3-icons/{name}"));
                if path.starts_with(prefix) {
                    out.push(path);
                }
            }
        }
        out
    }
}

impl AssetSource for Md3Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(self.find(path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(self.paths(path))
    }
}

pub struct CombinedAssets {
    inner: Md3Assets,
    fallback: Box<dyn AssetSource>,
}

impl AssetSource for CombinedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = self.inner.find(path) {
            return Ok(Some(bytes));
        }
        self.fallback.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut out = self.inner.paths(path);
        out.extend(self.fallback.list(path)?);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use anyhow::Result;
    use gpui::{AssetSource, SharedString};

    use super::{Md3Assets, PROGRESS_ARC_SVG_PATH};

    #[test]
    fn embedded_resources_load() -> Result<()> {
        let assets = Md3Assets::new();
        assert!(assets.load(PROGRESS_ARC_SVG_PATH)?.is_some());
        assert!(assets.load("md3-icons/not_embedded.svg")?.is_none());
        Ok(())
    }

    #[test]
    fn icon_dirs_load_on_demand_and_reject_traversal() -> Result<()> {
        let dir = std::env::temp_dir().join(format!("md3-gpui-icons-{}", std::process::id()));
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("bolt.svg"), "<svg></svg>")?;

        let assets = Md3Assets::new().with_icon_dir(&dir);
        let loaded = assets.load("md3-icons/bolt.svg")?;
        assert_eq!(loaded.map(Cow::into_owned), Some(b"<svg></svg>".to_vec()));
        assert!(assets.load("md3-icons/missing.svg")?.is_none());
        assert!(assets.load("md3-icons/../bolt.svg")?.is_none());
        assert!(
            assets
                .list("md3-icons/")?
                .contains(&SharedString::from("md3-icons/bolt.svg"))
        );
        std::fs::remove_dir_all(&dir)?;
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
        let assets = Md3Assets::new().with_fallback(Fallback);
        assert!(assets.load(PROGRESS_ARC_SVG_PATH)?.is_some());
        assert_eq!(
            assets.load("application/icon.svg")?.as_deref(),
            Some(b"fallback".as_slice())
        );
        assert_eq!(
            assets.list("application/")?,
            vec![SharedString::from("application/icon.svg")]
        );
        Ok(())
    }
}
