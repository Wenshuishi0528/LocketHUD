import { execFile } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, dirname, resolve } from 'node:path';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';

const execFileAsync = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const adb = process.env.ADB || '/Users/apple/Library/Android/sdk/platform-tools/adb';
const aixPath = resolve(root, 'dist/lockethud-photo.aix');
const packageJson = JSON.parse(await readFile(resolve(root, 'package.json'), 'utf8'));
const aix = await readFile(aixPath);
const md5 = createHash('md5').update(aix).digest('hex');
const agentId = 'fea33d142f1443b282eb9c3a62d54183';
const legacyAgentId = 'lockethud-photo';
const remoteDirectory = '/sdcard/jsai/package';
const remoteFile = `${remoteDirectory}/${agentId}_${packageJson.version}_${md5.slice(0, 8)}.aix`;
const component = 'com.rokid.os.sprite.assistserver/com.rokid.os.sprite.jsai.JsaiService';
const temporaryDirectory = await mkdtemp(resolve(tmpdir(), 'lockethud-aiui-device-'));
const pulledIndex = resolve(temporaryDirectory, 'agents_index.original.json');
const patchedIndex = resolve(temporaryDirectory, 'agents_index.json');

async function run(args, allowFailure = false) {
  try {
    return await execFileAsync(adb, args, { maxBuffer: 8 * 1024 * 1024 });
  } catch (error) {
    if (allowFailure) return { stdout: '', stderr: String(error) };
    throw error;
  }
}

try {
  if ((await run(['get-state'])).stdout.trim() !== 'device') {
    throw new Error('ADB device is not ready');
  }
  await run(['shell', 'mkdir', '-p', remoteDirectory]);
  await run(['pull', `${remoteDirectory}/agents_index.json`, pulledIndex], true);

  let agents = [];
  try {
    const existing = JSON.parse(await readFile(pulledIndex, 'utf8'));
    if (Array.isArray(existing.agents)) agents = existing.agents;
  } catch {}

  const previousFiles = agents
    .filter((agent) => agent.agentId === agentId || agent.agentId === legacyAgentId)
    .map((agent) => agent.filePath)
    .filter((filePath) => typeof filePath === 'string'
      && (filePath.startsWith(`${remoteDirectory}/${agentId}_`)
        || filePath.startsWith(`${remoteDirectory}/${legacyAgentId}_`))
      && filePath !== remoteFile);
  agents = agents.filter((agent) => agent.agentId !== agentId && agent.agentId !== legacyAgentId);
  agents.push({
    agentId,
    agentName: '照片浮窗',
    agentDesc: 'Display a locally transferred photo or animated GIF',
    agentLogo: '',
    url: '',
    permissions: [],
    nativeVersion: packageJson.version,
    fileMd5: md5,
    filePath: remoteFile,
    updatedAt: Date.now(),
  });
  await writeFile(patchedIndex, `${JSON.stringify({ agents })}\n`, 'utf8');

  await run(['shell', 'am', 'force-stop', 'com.rokid.os.sprite.assistserver']);
  for (const previousFile of previousFiles) await run(['shell', 'rm', '-f', previousFile], true);
  await run(['push', aixPath, remoteFile]);
  await run(['push', patchedIndex, `${remoteDirectory}/agents_index.json`]);
  await run(['push', patchedIndex, `${remoteDirectory}/agents_index.json.bak`]);

  await run(['shell', 'input', 'keyevent', '224']);
  await run([
    'shell', 'am', 'startservice', '-n', component,
    '-a', 'com.rokid.os.sprite.jsai.OPEN_PAGE',
    '--es', 'open_params', `'${JSON.stringify({ agentId })}'`,
    '--es', 'test_run_id', `lockethud-aiui-${Date.now()}`,
  ]);
  console.log(`Installed and opened ${basename(aixPath)} on the connected glasses.`);
  console.log(`MD5: ${md5}`);
  console.log(`Device path: ${remoteFile}`);
} finally {
  await rm(temporaryDirectory, { recursive: true, force: true });
}
