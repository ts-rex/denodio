import * as internal from "./bindings/bindings.ts"

export type Options = {
	/**
	 * Uint8Array
	 */
	buffer: Uint8Array
	volume?: number
	speed?: number
	use_spatial: boolean
	/**
	 * position of the sound emitter
	 * Ignored if `use_spacial` is false
	 */
	emitter_pos?: [number, number, number]
	/**
	 * position of the left ear of the listener
	 * Ignored if `use_spacial` is false
	 */
	left_ear?: [number, number, number]
	/**
	 * position of the left ear of the listener
	 * Ignored if `use_spacial` is false
	 */
	right_ear?: [number, number, number]
}

// Basically the same as the rust code, where we are modifying the values.

export async function play(options: Options) {
    // @ts-ignore: Generated types don't always match what the library actually needs.
	const newOptions: internal.Options = {
		...options,
		buffer: Array.from(options.buffer),
	}
    if(newOptions.speed && (newOptions.speed > 1 || newOptions.speed < 0)) {
        throw new Error('speed must be between 0 and 1')
    }
    if(newOptions.volume && (newOptions.volume > 1 || newOptions.volume < 0)) {
        throw new Error('volume must be between 0 and 1')
    }

	return await internal.play(newOptions)
}
