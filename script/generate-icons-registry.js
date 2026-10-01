const fs = require("fs");
const path = require("path");

const iconsDir = path.join("crates", "assets", "symbols_icons");
const outPath = path.join("crates", "assets", "icons_registry.rs");

const names = fs.readdirSync(iconsDir);
names.sort();
const entries = names
  .filter((name) => name.endsWith(".svg"))
  .map((name) => {
    const stem = name.slice(0, -4);
    return `    ("${stem}", include_bytes!(concat!("symbols_icons/", "${stem}", ".svg"))),`;
  });

const content = [
  "// 按 icons_registry 生成脚本维护;每条目将对应 SVG 编译期内嵌进二进制。",
  "pub static ICON_SVGS: &[(&str, &[u8])] = &[",
  entries.join("\n"),
  "];",
  "",
];
fs.writeFileSync(outPath, content.join("\n"));
console.log("registry entries:", entries.length);
