const { renameSync, readFileSync } = require("node:fs");
const path = require("node:path");
const toml = require("toml");

// Convert all backslashes to forward slashes to make it easier to work with paths

const filePath = process.argv[2].replace(/\\/g, "/")
// Prefix the last part of the path with "bruh" to make it easier to find
const tomlFile = toml.parse(readFileSync(path.join(process.cwd(), "Cargo.toml"), "utf8"));
const newFilePath = path.join(path.dirname(filePath), `spotify-np-v${tomlFile.package.version}.${path.basename(filePath)}`);

renameSync(filePath, newFilePath);