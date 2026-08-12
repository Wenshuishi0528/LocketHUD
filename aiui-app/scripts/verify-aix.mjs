import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import initAix, { AixReaderWasm } from '@yodaos-pkg/aix/pkg/aix_web.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const packagePath = resolve(root, 'dist/lockethud-photo.aix');
const wasmPath = resolve(root, 'node_modules/@yodaos-pkg/aix/pkg/aix_web_bg.wasm');

await initAix({ module_or_path: await readFile(wasmPath) });
const reader = new AixReaderWasm(await readFile(packagePath));
const names = reader.list().map((entry) => entry.name).sort();
for (const requiredName of [
  'AGENTS.md',
  'VERSION',
  'app.js',
  'app.json',
  'pages/index/index.ink',
  'assets/icon.png',
  'assets/display_default.png',
]) {
  assert.ok(names.includes(requiredName), `AIX is missing ${requiredName}`);
}
assert.equal(reader.get_title(), '照片浮窗');
assert.equal(reader.get_pages().length, 1);
assert.equal(reader.get_tools().length, 1);
console.log('Official @yodaos-pkg/aix verification passed:');
console.log(`- title: ${reader.get_title()}`);
console.log(`- version: ${reader.get_version()}`);
console.log(`- entries: ${names.length}`);
