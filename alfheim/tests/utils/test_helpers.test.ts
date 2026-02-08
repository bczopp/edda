/**
 * Tests für tests/utils/test_helpers.ts.
 * Nur HTTP-Helper – gRPC-Services (Odin, Thor) werden nicht über URLs angesprochen.
 */
import { describe, test, expect, afterEach } from 'bun:test';
import { getHttpServiceUrl } from './test_helpers';

describe('getHttpServiceUrl', () => {
  afterEach(() => {
    delete (process.env as any).API_URL;
    delete (process.env as any).DASHBOARD_URL;
  });

  test('sollte Default-URL liefern wenn Umgebungsvariable fehlt', () => {
    delete (process.env as any).API_URL;
    expect(getHttpServiceUrl('api', 8080)).toBe('http://localhost:8080');
  });

  test('sollte Service-Name in Umgebungsvariable uppercase verwenden', () => {
    (process.env as any).API_URL = 'http://api:8080';
    expect(getHttpServiceUrl('api', 8080)).toBe('http://api:8080');
  });

  test('sollte anderen HTTP-Service und Port unterstützen', () => {
    delete (process.env as any).DASHBOARD_URL;
    expect(getHttpServiceUrl('dashboard', 3000)).toBe('http://localhost:3000');
  });
});
