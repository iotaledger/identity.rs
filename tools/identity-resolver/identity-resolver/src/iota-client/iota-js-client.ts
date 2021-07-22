import { SingleNodeClient } from "../client-lib/index-browser.js";
import type { IotaClient } from "./client"

export class IotaJsClient implements IotaClient {
  client: SingleNodeClient;

  constructor(node: string) {
    this.client = new SingleNodeClient(node);
  }

  async findMessageById(messageId: string): Promise<Object> {
    return await this.client.message(messageId);
  }

  async findMessagesByIndex(index: string) {
    const res = await this.client.messagesFind(index);
    return res.messageIds as [string];
  }

  async run() {}
}
