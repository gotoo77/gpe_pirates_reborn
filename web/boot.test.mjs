import { test } from 'node:test';
import assert from 'node:assert/strict';
import { boot } from './boot.mjs';

test('winit handoff continues but real startup errors remain failures', async () => {
  const status = { textContent: '' };
  const handoff = new Error("Using exceptions for control flow, don't mind me. This isn't actually an error!");
  await boot(async () => { throw handoff; }, status);
  assert.match(status.textContent, /Fleches/);
  const failure = new Error('WebGPU unavailable');
  await assert.rejects(boot(async () => { throw failure; }, status), failure);
  assert.match(status.textContent, /WebGPU unavailable/);
  await assert.rejects(boot(async () => { throw new Error('Using exceptions for control flow'); }, status));
});
