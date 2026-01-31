/**
 * Wait for a service to be ready
 */
export async function waitForService(url: string, maxRetries: number): Promise<boolean> {
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
 * Get service URL from environment or use default
 */
export function getServiceUrl(serviceName: string, defaultPort: number): string {
    const envVar = `${serviceName.toUpperCase()}_URL`;
    return process.env[envVar] || `http://localhost:${defaultPort}`;
}
