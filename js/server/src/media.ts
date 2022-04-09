import { spawn } from "child_process";

const EXE_PATH = "../driver/target/release/driver.exe";

export function getCurrentlyPlaying(): Promise<{
  [key: string]: string;
}> {
  const executable = spawn(EXE_PATH, ["cp_raw"]);

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
      arg = "pl";
      break;
    case "pause":
      arg = "pa";
      break;
    case "next":
      arg = "nt";
      break;
    case "prev":
      arg = "pt";
      break;
  }

  const executable = spawn(EXE_PATH, [arg]);

  executable.stdout.on("data", (data: Buffer) => console.log(data.toString()));
}
