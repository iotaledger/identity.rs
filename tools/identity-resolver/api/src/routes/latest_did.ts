import express from "express";
import { ClientBuilder,  } from "@iota/client";

export async function latestDid(req: express.Request, res: express.Response): Promise<void> {
  // // client will connect to testnet by default
  // const client = new ClientBuilder().build();

  // const message = await client
  //   .message()
  //   .index("random index 123123123")
  //   .data("some utf based datasome utf based datasome utf based datasome utf baseddatasome utf based datasome utf based datasome utf baseddatasome utf based datasome utf based datasome utf baseddatasome utf based datasome utf based datasome utf based datasome utf based datasome utf based datasome utf based datasome utf based data")
  //   .submit();

  // console.log(message);


const client = new ClientBuilder().network('testnet').disableNodeSync().build();

// const message_data = await client.getMessage().index("random index 123123123");

const message_data = await client.getMessage().index("EuqRR1hFbtdyWQBoDtNYbFjBG4SJjWeWPKc157pz8By2");

console.log(">>>>>>>>>>>>>>> all messages:");
console.log(message_data);


let message_data2 = await client.getMessage().data(message_data[0]);

console.log(">>>>>>>>>>>>>>>>>>>");

// console.log(((message_data2.message.payload as any ).data));


console.log(">>>>>>>>>>>>>>>>>>>");
let output = Buffer.from(((message_data2.message.payload as any ).data), 'hex').toString();
console.log(JSON.parse(output));

console.log(">>>>>>>>>>>>>>>>>>>");
console.log(">>>>>>>>>>>>>>>>>>>");
console.log(">>>>>>>>>>>>>>>>>>>");



 message_data2 = await client.getMessage().data(message_data[1]);

console.log(">>>>>>>>>>>>>>>>>>>");

// console.log(((message_data2.message.payload as any ).data));


console.log(">>>>>>>>>>>>>>>>>>>");
output = Buffer.from(((message_data2.message.payload as any ).data), 'hex').toString();
console.log(JSON.parse(output));

  res.send(message_data2);
}
