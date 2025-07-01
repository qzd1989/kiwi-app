import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

// ESModule 环境下处理 __dirname
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// 1. 读取 package.json
const packageJson = JSON.parse(
  fs.readFileSync(path.join(__dirname, "package.json"), "utf-8")
);
const version = packageJson.version;

// 2. 更新 tauri.conf.json
const tauriConfPath = path.join(__dirname, "src-tauri", "tauri.conf.json");
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf-8"));
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
console.log(`✅ Updated tauri.conf.json to version ${version}`);

// 3. 更新 Cargo.toml
const cargoTomlPath = path.join(__dirname, "src-tauri", "Cargo.toml");
let cargoToml = fs.readFileSync(cargoTomlPath, "utf-8");
cargoToml = cargoToml.replace(
  /version\s*=\s*"[^"]+"/,
  `version = "${version}"`
);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log(`✅ Updated Cargo.toml to version ${version}`);
