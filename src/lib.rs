extern crate deno_bindgen;
extern crate rodio;

use deno_bindgen::deno_bindgen as binding;
use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor};

#[binding]
struct Options {
    /// Uint8Array
    buffer: Vec<u8>,
}

#[binding(non_blocking)]
// Play a sound
// import { play } from "./mod.ts"

// await play({ buffer: Array.from(await Deno.readFile("./tone.mp3")) });
pub fn play(options: Options) {
    print!("bp 0");
    let internal_options = InternalOptions {
        buffer: unsafe { std::mem::transmute(options.buffer.as_slice()) },
    };
    print!("bp 1");
    _play(internal_options)
}

struct InternalOptions {
    buffer: &'static [u8],
}

fn _play(options: InternalOptions) -> () {
    print!("bp 2");
    let (_stream, handle) = OutputStream::try_default().unwrap();
    print!("bp 3");
    let sink = Sink::try_new(&handle).unwrap();
    print!("bp 4");
    let cursor: Cursor<&[u8]> = Cursor::new(options.buffer);
    print!("bp 5");
    let buffer_reader: BufReader<Cursor<&[u8]>> = BufReader::new(cursor);
    print!("bp 6");
    sink.append(Decoder::new(buffer_reader).unwrap());
    sink.sleep_until_end();
}
