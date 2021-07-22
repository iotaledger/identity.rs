import express from "express";
import { latestDid } from "./routes/latest_did";

const app: express.Application = express();

app.get("/", latestDid);

app.listen(5000, () => {
  console.log("running");
});
