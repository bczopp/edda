/**
 * Prüft, ob ein HTTP-Endpunkt erreichbar ist (z. B. Web-UI, REST-API).
 * Nur für HTTP-Services – nicht für gRPC (Odin, Thor, etc.). gRPC nutzt address+port, siehe config.
 */
export async function waitForHttpService(url: string, maxRetries: number): Promise<boolean> {
    for (let i = 0; i < maxRetries; i++) {
        try {
            const response = await fetch(url);
            if (response.ok) {
                return true;
            }
        } catch (e) {
            // Service not ready yet
        }
        await new Promise(resolve => setTimeout(resolve, 500));
    }
    return false;
}

/**
 * Liefert die HTTP-URL eines Services aus Env oder Default.
 * Nur für HTTP-Services – nicht für gRPC. Odin/Thor etc. werden über address+port (config) angesprochen.
 */
export function getHttpServiceUrl(serviceName: string, defaultPort: number): string {
    const envVar = `${serviceName.toUpperCase()}_URL`;
    return process.env[envVar] || `http://localhost:${defaultPort}`;
}
