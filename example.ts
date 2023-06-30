import { play } from "./mod.ts"

await play({
	buffer: await Deno.readFile("./tone.mp3"),
	use_spatial: false,
    volume: 0.1
})