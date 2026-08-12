import { cp, mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const output = resolve(root, 'dist/craft-import');
const runtimeFiles = [
  'AGENTS.md',
  'app.js',
  'app.json',
  'pages/index/index.ink',
  'assets/icon.png',
  'assets/portrait_default.png',
];

await rm(output, { recursive: true, force: true });
await mkdir(output, { recursive: true });
for (const relativePath of runtimeFiles) {
  const sourcePath = resolve(root, relativePath);
  const destinationPath = resolve(output, relativePath);
  await mkdir(dirname(destinationPath), { recursive: true });
  await cp(sourcePath, destinationPath);
}

const app = JSON.parse(await readFile(resolve(root, 'app.json'), 'utf8'));
await writeFile(resolve(root, 'dist/CRAFT_IMPORT.json'), `${JSON.stringify({
  generatedAt: new Date().toISOString(),
  project: 'lockethud-aiui',
  pages: app.pages,
  permissions: app.permissions,
  files: runtimeFiles,
}, null, 2)}\n`, 'utf8');

console.log(`Craft import folder prepared: ${output}`);
console.log(`Runtime files: ${runtimeFiles.length}`);
