import { doAction, getCurrentlyPlaying } from "./media";

import Express from "express";
const app = Express();
const PORT = 3000;

// TODO Change all these get requests to `put` or something viable

app.get("/play", (req, res) => {
  doAction("play");
  console.log("Trying to Play");
  res.send("Okay. Go back <a href='/'>home</a>");
});
app.get("/pause", (req, res) => {
  doAction("pause");
  console.log("Trying to Pause");
  res.send("Okay. Go back <a href='/'>home</a>");
});
app.get("/next", (req, res) => {
  doAction("next");
  console.log("Trying to skip to the next song");
  res.send("Okay. Go back <a href='/'>home</a>");
});
app.get(["/prev", "/previous"], (req, res) => {
  doAction("prev");
  console.log("Trying to skip to the previous song");
  res.send("Okay. Go back <a href='/'>home</a>");
});
app.get(["/", "/curplay", "cur", "/playing"], (req, res) => {
  getCurrentlyPlaying().then((i) => res.send(i));
});

app.listen(PORT, () => {
  console.log(`Running app at http://localhost:${PORT}`);
});
