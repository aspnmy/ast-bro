#!/usr/bin/env node

/**
 * ast-outline CLI wrapper (final 2.1.1 release) — forwards arguments to the Rust binary.
 */

const { execFileSync } = require("child_process");
const { getBinaryPath } = require("./install");

const binary = getBinaryPath();
const args = process.argv.slice(2);

try {
  execFileSync(binary, args, { stdio: "inherit" });
} catch (err) {
  if (err.status !== undefined) {
    process.exit(err.status);
  }
  console.error(err.message);
  process.exit(1);
}
