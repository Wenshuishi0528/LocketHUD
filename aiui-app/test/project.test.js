import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const inkUrl = new URL('../pages/index/index.ink', import.meta.url);

test('renders a bundled local image without cloud access', async () => {
  const ink = await readFile(inkUrl, 'utf8');
  assert.match(ink, /assets\/portrait_default\.png/);
  assert.match(ink, /mode="widthFix"/);
  assert.doesNotMatch(ink, /https?:\/\//);
});

test('keeps all display controls on the Mac side', async () => {
  const ink = await readFile(inkUrl, 'utf8');
  for (const anchor of ['left_top', 'left_middle', 'left_bottom', 'right_top', 'right_middle', 'right_bottom']) {
    assert.match(ink, new RegExp(anchor));
  }
  for (const size of ['small', 'medium', 'large']) assert.match(ink, new RegExp(size));
  for (const opacity of ['opacity_40', 'opacity_60', 'opacity_80', 'opacity_100']) {
    assert.match(ink, new RegExp(opacity));
  }
});
