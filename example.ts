import { play } from "./mod.ts"

await play({ buffer: Array.from(await Deno.readFile("./tone.mp3")) });