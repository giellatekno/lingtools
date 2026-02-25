/*
/// Parse a weight from a [`&str`]. This functionality exists because weight strings use
/// ',' as the decimal point (instead of `.`).
///
/// It's like parsing a normal f64, but the machinery in Rust that parses floats expects
/// the decimal point to be a `.`, and not a `,`.
#[inline(always)]
pub fn parse_weight(weight: &str) -> Result<f64, std::num::ParseFloatError> {
    use std::mem::MaybeUninit;

    // The strategy here to avoid allocating, is to stack allocate a buffer that is big
    // enough to hold the longest stringified f64. The size of that buffer would be:
    // (NOTE: scientifically written floats not checked).
    const MAX_F64_CHARS: usize = const {
        1 // an optional + or - sign
        + f64::DIGITS // max number of digits in the charismatic
        + 1 // the comma (or dot)
        + f64::MANTISSA_DIGITS // max number number of digits in the mantissa
    } as usize;

    // Stack allocate the buffer
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; MAX_F64_CHARS];

    // We then copy over the weight we're given in `weight` into this buffer, but
    // write a '.' instead of a ',' if we see one.
    let len: usize = weight.len();
    weight
        .bytes()
        .map(|ch| if ch != b',' { ch } else { b'.' })
        .enumerate()
        .for_each(|(i, ch)| {
            buf[i].write(ch);
        });

    // Turn our buffer of "maybe uninitialized bytes" (`MaybeUninit<u8>`) into a string
    // slice (&str). We know that the bytes constitute a valid &str, because we copied
    // each byte from the input string `weight` directly over to the buffer.

    // SAFETY: Everything has been initialized up to character number `len`
    let s = unsafe { std::mem::transmute::<&[MaybeUninit<u8>], &str>(&buf[..len]) };

    // Finally call the normal rust float parsing machinery to actually parse it
    Ok(s.parse()?)
}
*/
