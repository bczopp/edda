import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';

export class OdinClient {
  private client: any;

  constructor(address: string, port: number) {
    const packageDefinition = protoLoader.loadSync('proto/odin.proto', {
      keepCase: true,
      longs: String,
      enums: String,
      defaults: true,
      oneofs: true,
    });

    const odinProto = grpc.loadPackageDefinition(packageDefinition) as any;
    this.client = new odinProto.odin.OdinService(
      `${address}:${port}`,
      grpc.credentials.createInsecure()
    );
  }

  async processRequest(request: any): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.processRequest(request, (error: any, response: any) => {
        if (error) {
          reject(error);
        } else {
          resolve(response);
        }
      });
    });
  }
}
