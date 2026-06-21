import assert from "node:assert/strict";
import fs from "node:fs";

const spiCargo = fs.readFileSync("crates/sdkwork-llm-spi/Cargo.toml", "utf8");
const runtimeCargo = fs.readFileSync("crates/sdkwork-llm-runtime/Cargo.toml", "utf8");

for (const [path, content] of [
  ["crates/sdkwork-llm-spi/Cargo.toml", spiCargo],
  ["crates/sdkwork-llm-runtime/Cargo.toml", runtimeCargo],
]) {
  for (const forbidden of [
    "axum",
    "actix",
    "rocket",
    "hyper",
    "reqwest",
    "sdkwork-llm-sdk",
    "sdkwork-llm-app-sdk",
    "sdkwork-llm-backend-sdk",
  ]) {
    assert.ok(
      !content.includes(forbidden),
      `${path} must remain provider/framework-neutral and must not depend on ${forbidden}`,
    );
  }
}

const spiLib = fs.readFileSync("crates/sdkwork-llm-spi/src/lib.rs", "utf8");
assert.ok(
  !spiLib.includes("async fn"),
  "SPI lib.rs must remain a lightweight module assembly file",
);
