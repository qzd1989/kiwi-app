const fs = require("fs");
const path = require("path");

const packageJson = require("./package.json");
const version = packageJson.version;

// 1. 更新 tauri.conf.json
const tauriConfPath = path.join(__dirname, "src-tauri", "tauri.conf.json");
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf-8"));
tauriConf.package.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
console.log(`Updated tauri.conf.json to version ${version}`);

// 2. 更新 Cargo.toml
const cargoTomlPath = path.join(__dirname, "src-tauri", "Cargo.toml");
let cargoToml = fs.readFileSync(cargoTomlPath, "utf-8");
cargoToml = cargoToml.replace(
  /version\s*=\s*"[0-9a-zA-Z\.\-\+]+"/,
  `version = "${version}"`
);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log(`Updated Cargo.toml to version ${version}`);
