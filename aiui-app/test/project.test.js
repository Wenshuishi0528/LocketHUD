import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';

const inkUrl = new URL('../pages/index/index.ink', import.meta.url);

test('renders one precomposed 448 x 352 frame without cloud access', async () => {
  const ink = await readFile(inkUrl, 'utf8');
  assert.match(ink, /assets\/display_default\.png/);
  assert.match(ink, /mode="scaleToFill"/);
  assert.match(ink, /\.frame/);
  assert.doesNotMatch(ink, /widthFix/);
  assert.doesNotMatch(ink, /https?:\/\//);
});

test('keeps all display controls on the Mac side', async () => {
  const ink = await readFile(inkUrl, 'utf8');
  assert.doesNotMatch(ink, /left_top|right_bottom|small|medium|large/);
  assert.doesNotMatch(ink, /opacity_40|opacity_60|opacity_80|opacity_100/);
});
