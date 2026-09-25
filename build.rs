//! 构建脚本:Windows 下把 material3-favicon.svg 转成 ICO 并嵌入 exe,
//! 作为窗口类图标(gpui 以资源 ID=1 加载,任务栏/Alt-Tab 亦使用)。
//! 其他平台无操作。
//!
//! 注意:rc.exe 不支持 PNG 压缩帧(RC2176),必须写传统 32bpp DIB 帧。

use std::env;
use std::path::{Path, PathBuf};

const ICON_SIZE: u32 = 256;

fn main() {
    println!("cargo:rerun-if-changed=src/assets/material3-favicon.svg");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let ico_path = render_ico(&out_dir);
    write_rc(&out_dir, &ico_path);
    embed_resource::compile(out_dir.join("material3.rc"), embed_resource::NONE)
        .manifest_optional()
        .expect("failed to compile Windows resources");
}

/// 渲染 SVG 为 256×256,并包装成传统 32bpp DIB 单帧 ICO。
fn render_ico(out_dir: &Path) -> PathBuf {
    let svg = include_str!("src/assets/material3-favicon.svg");
    let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default())
        .expect("failed to parse material3-favicon.svg");
    let size = tree.size();
    // SVG 自身 70×70 正方形,等比放大铺满
    let scale = ICON_SIZE as f32 / size.width();

    let mut pixmap = resvg::tiny_skia::Pixmap::new(ICON_SIZE, ICON_SIZE).unwrap();
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // DIB:BITMAPINFOHEADER(40B)+ 自下而上 BGRA 像素 + 1bpp AND 掩码
    // biHeight = 2×实际高(像素 + 掩码)
    let s = ICON_SIZE as usize;
    let mut dib = Vec::with_capacity(40 + s * s * 4 + s * (s / 8));
    dib.extend_from_slice(&40u32.to_le_bytes()); // biSize
    dib.extend_from_slice(&(ICON_SIZE as i32).to_le_bytes()); // biWidth
    dib.extend_from_slice(&((ICON_SIZE * 2) as i32).to_le_bytes()); // biHeight
    dib.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
    dib.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
    dib.extend_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
    dib.extend_from_slice(&((s * s * 4) as u32).to_le_bytes()); // biSizeImage(像素)
    dib.extend_from_slice(&0u32.to_le_bytes()); // biXPelsPerMeter
    dib.extend_from_slice(&0u32.to_le_bytes()); // biYPelsPerMeter
    dib.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
    dib.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

    let px = pixmap.data();
    // 自下而上、RGBA(premultiplied)→ BGRA(还原直通 alpha,避免边缘发黑)
    for y in (0..s).rev() {
        for x in 0..s {
            let i = (y * s + x) * 4;
            let (r, g, b_, a) = (px[i], px[i + 1], px[i + 2], px[i + 3]);
            let (r, g, b_) = if a == 0 {
                (0u8, 0u8, 0u8)
            } else {
                let a = a as u32;
                (
                    ((r as u32 * 255 + a / 2) / a) as u8,
                    ((g as u32 * 255 + a / 2) / a) as u8,
                    ((b_ as u32 * 255 + a / 2) / a) as u8,
                )
            };
            dib.push(b_);
            dib.push(g);
            dib.push(r);
            dib.push(a);
        }
    }
    // AND 掩码:32bpp 含 alpha 时置 0(透明度由 alpha 通道决定)
    dib.resize(40 + s * s * 4 + s * (s / 8), 0);

    // ICO 容器:6 字节头 + 16 字节目录项 + DIB
    let mut ico = Vec::with_capacity(22 + dib.len());
    ico.extend_from_slice(&[0, 0, 1, 0, 1, 0]); // ICONDIR: 保留字 + 类型 1(图标) + 1 帧
    ico.push(0); // 宽(0 表示 256)
    ico.push(0); // 高
    ico.push(0); // 调色板色数
    ico.push(0); // 保留
    ico.extend_from_slice(&1u16.to_le_bytes()); // 色面
    ico.extend_from_slice(&32u16.to_le_bytes()); // 位深
    ico.extend_from_slice(&(dib.len() as u32).to_le_bytes());
    ico.extend_from_slice(&22u32.to_le_bytes());
    ico.extend_from_slice(&dib);

    let ico_path = out_dir.join("material3-favicon.ico");
    std::fs::write(&ico_path, &ico).expect("failed to write icon ICO");
    ico_path
}

/// 生成 rc:`1 ICON "..."`(资源 ID 必须为 1,gpui 以 MAKEINTRESOURCE(1) 加载)。
fn write_rc(out_dir: &Path, ico_path: &Path) {
    let ico = ico_path.to_string_lossy().replace('\\', "\\\\");
    let rc = out_dir.join("material3.rc");
    std::fs::write(&rc, format!("1 ICON \"{ico}\"\n")).expect("failed to write rc");
}
