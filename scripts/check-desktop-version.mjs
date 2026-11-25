import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const rustRoot = resolve(repositoryRoot, "rust");
const cargoManifest = await readFile(resolve(rustRoot, "Cargo.toml"), "utf8");
const workspacePackageMarker = cargoManifest.indexOf("[workspace.package]");
const workspacePackage =
  workspacePackageMarker < 0
    ? undefined
    : cargoManifest
        .slice(workspacePackageMarker + "[workspace.package]".length)
        .split(/^\[[^\]]+\]\s*$/m, 1)[0];
const cargoVersion = workspacePackage?.match(
  /^version\s*=\s*"([^"]+)"\s*$/m,
)?.[1];
if (!cargoVersion) {
  throw new Error(
    "Could not read version from [workspace.package] in rust/Cargo.toml",
  );
}

const frontend = JSON.parse(
  await readFile(resolve(rustRoot, "apps/desktop/package.json"), "utf8"),
);
const lockfile = JSON.parse(
  await readFile(resolve(rustRoot, "apps/desktop/package-lock.json"), "utf8"),
);
const tauri = JSON.parse(
  await readFile(
    resolve(rustRoot, "apps/desktop/src-tauri/tauri.conf.json"),
    "utf8",
  ),
);

const versions = {
  "Rust workspace package": cargoVersion,
  "Desktop frontend package": frontend.version,
  "Desktop frontend lockfile": lockfile.version,
  "Desktop lockfile root package": lockfile.packages?.[""]?.version,
  "Tauri application config": tauri.version,
};
const unique = new Set(Object.values(versions));

if (unique.size !== 1) {
  throw new Error(
    `Desktop versions must match:\n${Object.entries(versions)
      .map(([name, version]) => `- ${name}: ${version}`)
      .join("\n")}`,
  );
}

console.log(`Desktop version is consistent: ${cargoVersion}`);
