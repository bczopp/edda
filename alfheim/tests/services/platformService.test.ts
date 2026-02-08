/**
 * Tests für src/services/platformService.ts (TDD).
 */
import { PlatformService } from '../../src/services/platformService';

describe('PlatformService', () => {
  it('sollte processRequest werfen wenn nicht initialisiert', async () => {
    const service = new PlatformService();
    await expect(service.processRequest({})).rejects.toThrow(
      'Platform service not initialized'
    );
  });

  test('sollte ohne initialize processRequest ablehnen', async () => {
    const service = new PlatformService();
    await expect(service.processRequest({ text: 'hi' })).rejects.toThrow(
      'Platform service not initialized'
    );
  });
});
