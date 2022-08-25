import { spawn } from "child_process";

const EXE_PATH = "../../target/release/window.exe";

export function getCurrentlyPlaying(): Promise<{
  [key: string]: string;
}> {
  const executable = spawn(EXE_PATH, ["current-json"]);

  return new Promise((resolve) => {
    executable.stdout.on("data", (data: Buffer) => {
      resolve(JSON.parse(data.toString()));
    });
  });
}

type Action = "play" | "pause" | "next" | "prev";

export function doAction(action: Action) {
  let arg = "";

  switch (action) {
    case "play":
      arg = "play";
      break;
    case "pause":
      arg = "pause";
      break;
    case "next":
      arg = "next";
      break;
    case "prev":
      arg = "previous";
      break;
  }

  const executable = spawn(EXE_PATH, [arg]);

  executable.stdout.on("data", (data: Buffer) => console.log(data.toString()));
}
