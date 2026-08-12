import { execFile } from 'node:child_process';
import { randomUUID } from 'node:crypto';
import { readFile, rm, stat, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';

const execFileAsync = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const importFolder = resolve(root, 'dist/craft-import');
const outputFile = resolve(root, 'dist/lockethud-photo.aix');

await import(`./prepare-craft-import.mjs?run=${Date.now()}`);
const version = randomUUID();
await writeFile(resolve(importFolder, 'VERSION'), version, 'utf8');
await rm(outputFile, { force: true });
await execFileAsync('/usr/bin/zip', ['-X', '-q', '-r', outputFile, '.'], { cwd: importFolder });

const metadata = await stat(outputFile);
const magic = (await readFile(outputFile)).subarray(0, 4).toString('hex');
if (magic !== '504b0304') throw new Error(`Unexpected AIX ZIP signature: ${magic}`);
console.log(`AIX package created: ${outputFile}`);
console.log(`VERSION: ${version}`);
console.log(`Size: ${metadata.size} bytes`);
