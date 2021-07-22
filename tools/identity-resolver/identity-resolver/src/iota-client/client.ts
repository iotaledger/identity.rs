export interface IotaClient {
  findMessagesByIndex(index: string): Promise<[String]>;
  findMessageById(messageId: string): Object;
}
