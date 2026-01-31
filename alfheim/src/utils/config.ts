export interface OdinConfig {
  address: string;
  port: number;
}

export interface AlfheimSettings {
  odin: OdinConfig;
}

export const defaultSettings: AlfheimSettings = {
  odin: {
    address: "127.0.0.1",
    port: 50051,
  },
};

export class SettingsManager {
  private settings: AlfheimSettings = defaultSettings;

  async load(): Promise<void> {
    // Load settings from storage
    // For now, use defaults
    this.settings = defaultSettings;
  }

  get(): AlfheimSettings {
    return this.settings;
  }
}
