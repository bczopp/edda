import { OdinClient } from '../grpc/odinClient';
import { SettingsManager } from '../utils/config';

export class PlatformService {
  private odinClient: OdinClient | null = null;
  private settingsManager: SettingsManager;

  constructor() {
    this.settingsManager = new SettingsManager();
  }

  async initialize(): Promise<void> {
    await this.settingsManager.load();
    const settings = this.settingsManager.get();
    
    this.odinClient = new OdinClient(settings.odin.address, settings.odin.port);
  }

  async processRequest(request: any): Promise<any> {
    if (!this.odinClient) {
      throw new Error('Platform service not initialized');
    }
    return this.odinClient.processRequest(request);
  }
}
