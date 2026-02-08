/**
 * Tests für src/utils/config.ts (TDD – Settings/Config).
 */
import { describe, test, expect } from 'bun:test';
import {
  defaultSettings,
  SettingsManager,
} from '../src/utils/config';

describe('defaultSettings', () => {
  test('sollte Odin mit address 127.0.0.1 und port 50051 liefern', () => {
    expect(defaultSettings.odin).toEqual({
      address: '127.0.0.1',
      port: 50051,
    });
  });

  test('sollte ein gültiges AlfheimSettings-Objekt sein', () => {
    expect(defaultSettings).toHaveProperty('odin');
    expect(typeof defaultSettings.odin.address).toBe('string');
    expect(typeof defaultSettings.odin.port).toBe('number');
  });
});

describe('SettingsManager', () => {
  test('sollte nach get() die Default-Settings liefern (ohne load)', () => {
    const manager = new SettingsManager();
    const settings = manager.get();
    expect(settings.odin.address).toBe('127.0.0.1');
    expect(settings.odin.port).toBe(50051);
  });

  test('sollte load() ohne Fehler ausführen', async () => {
    const manager = new SettingsManager();
    await expect(manager.load()).resolves.toBeUndefined();
  });

  test('sollte nach load() weiterhin Settings liefern', async () => {
    const manager = new SettingsManager();
    await manager.load();
    const settings = manager.get();
    expect(settings).toBeDefined();
    expect(settings.odin).toBeDefined();
  });
});
