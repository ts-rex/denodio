extern crate deno_bindgen;
extern crate rodio;

use deno_bindgen::deno_bindgen as binding;
use rodio::{Decoder, OutputStream, SpatialSink};
use std::{io::{BufReader, Cursor}, convert::TryInto};

#[binding]
pub struct Options {
    /// Uint8Array
    buffer: Vec<u8>,
    volume: Option<f32>,
    speed: Option<f32>,
    use_spatial: bool,
    /// position of the sound emitter
    /// Ignored if `use_spacial` is false
    emitter_pos: Option<Vec<f32>>,
    /// position of the left ear of the listener
    /// Ignored if `use_spacial` is false
    left_ear: Option<Vec<f32>>,
    /// position of the left ear of the listener
    /// Ignored if `use_spacial` is false
    right_ear: Option<Vec<f32>>
}

fn pos(vec: Vec<f32>) -> [f32; 3] {
    let [x, y, z]: [f32; 3] = vec.try_into().unwrap();
    [x, y, z]
}

// Play a sound
// import { play } from "./mod.ts"
// await play({ buffer: Array.from(await Deno.readFile("./tone.mp3")) });
#[binding(non_blocking)]
pub fn play(options: Options) {
    let internal_options: InternalOptions = InternalOptions {
        volume: if options.volume.is_some() { options.volume.unwrap() } else { 1.0 },
        speed: if options.speed.is_some() { options.speed.unwrap() } else { 1.0 },
        use_spatial: options.use_spatial,
        emitter_pos: if options.use_spatial && options.emitter_pos.is_some() { pos(options.emitter_pos.unwrap()) } else { [0.0, 0.0, 0.0] },
        left_ear: if options.use_spatial && options.left_ear.is_some() { pos(options.left_ear.unwrap()) } else { [0.0, 0.0, 0.0] },
        right_ear: if options.use_spatial && options.right_ear.is_some() { pos(options.right_ear.unwrap()) } else { [0.0, 0.0, 0.0] },
        buffer: unsafe { std::mem::transmute(options.buffer.as_slice()) },
    };
    _play(internal_options)
}

struct InternalOptions {
    buffer: &'static [u8],
    volume: f32,
    speed: f32,
    use_spatial: bool,
    /// position of the sound emitter
    /// Ignored if `use_spacial` is false
    emitter_pos: [f32; 3],
    /// position of the left ear of the listener
    /// Ignored if `use_spacial` is false
    left_ear: [f32; 3],
    /// position of the left ear of the listener
    /// Ignored if `use_spacial` is false
    right_ear: [f32; 3]
}

fn _play(options: InternalOptions) -> () {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = SpatialSink::try_new(&handle, options.emitter_pos, options.left_ear, options.right_ear).unwrap();
    sink.set_volume(options.volume);
    sink.set_speed(options.speed);
    let cursor: Cursor<&[u8]> = Cursor::new(options.buffer);
    let buffer_reader: BufReader<Cursor<&[u8]>> = BufReader::new(cursor);
    sink.append(Decoder::new(buffer_reader).unwrap());
    sink.sleep_until_end();
}