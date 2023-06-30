// Auto-generated with deno_bindgen
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}

function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}

// deno-lint-ignore no-explicit-any
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}

const url = new URL(
  "https://github.com/marinastudios/denodio/releases/download/0.0.2",
  import.meta.url,
)

import { dlopen, FetchOptions } from "https://deno.land/x/plug@1.0.1/mod.ts"
let uri = url.toString()
if (!uri.endsWith("/")) uri += "/"

let darwin: string | { aarch64: string; x86_64: string } = uri

const opts: FetchOptions = {
  name: "denodio",
  url: {
    darwin,
    windows: uri,
    linux: uri,
  },
  suffixes: {
    darwin: {
      aarch64: "_arm64",
    },
  },
  cache: "use",
}
const { symbols } = await dlopen(opts, {
  play: { parameters: ["buffer", "usize"], result: "void", nonblocking: true },
})
export type Options = {
  /**
   * Uint8Array
   */
  buffer: Array<number>
  volume: number | undefined | null
  speed: number | undefined | null
  use_spatial: boolean
  /**
   * position of the sound emitter
   * Ignored if `use_spacial` is false
   */
  emitter_pos: Array<number> | undefined | null
  /**
   * position of the left ear of the listener
   * Ignored if `use_spacial` is false
   */
  left_ear: Array<number> | undefined | null
  /**
   * position of the left ear of the listener
   * Ignored if `use_spacial` is false
   */
  right_ear: Array<number> | undefined | null
}
export function play(a0: Options) {
  const a0_buf = encode(JSON.stringify(a0))

  const rawResult = symbols.play(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
