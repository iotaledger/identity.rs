import { Messages } from "../stores/stores";
import { createClient } from "./client-factory";

export async function getIntegrationMessageChain(index: string) {
  const client = createClient();

  let messageIds: [string] = await client.findMessagesByIndex(index);

  let first100Ids = messageIds;

  // if (messageIds.length > 50) {
  //   first100Ids = messageIds.slice(0, 50);
  // } else {
  //   first100Ids = messageIds;
  // }

  console.log(first100Ids);

  Messages.set([]);

  first100Ids.forEach(async (id) => {
    let message = await client.findMessageById(id);
    Messages.update((messages) => [...messages, message]);
  });
}
