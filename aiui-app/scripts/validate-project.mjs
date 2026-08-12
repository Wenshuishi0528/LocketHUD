import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const app = JSON.parse(await readFile(resolve(root, 'app.json'), 'utf8'));
const ink = await readFile(resolve(root, 'pages/index/index.ink'), 'utf8');

assert.deepEqual(app.pages, ['pages/index/index']);
assert.deepEqual(app.permissions, []);
assert.match(ink, /width: 448px/);
assert.match(ink, /height: 352px/);
assert.match(ink, /assets\/portrait_default\.png/);
assert.match(ink, /mode="widthFix"/);
assert.doesNotMatch(ink, /wx\.request/);
assert.doesNotMatch(ink, /emoji/i);
console.log('AIUI photo HUD project validation passed.');
