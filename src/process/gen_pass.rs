use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

// q: what is &[u8]?
// a: &[u8] 是一个字节切片，它是一个引用，指向一个字节数组，它的类型是 &[u8]，表示一个字节切片。
// q: can you explain it in english?
// a: &[u8] is a byte slice, it is a reference that points to a byte array, its type is &[u8], which represents a byte slice.
// q: what does u8 mean in &[u8]?
// a: u8 is an unsigned 8-bit integer, it is a byte type, which represents a byte.
// q: what about &[u8; 26]?
// a: &[u8; 26] is an array of 26 bytes, it is a reference that points to an array of 26 bytes, its type is &[u8; 26], which represents an array of 26 bytes.
// q: what about &[u16]?
// a: &[u16] is a slice of 16-bit unsigned integers, it is a reference that points to an array of 16-bit unsigned integers, its type is &[u16], which represents a slice of 16-bit unsigned integers.

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

// pub fn process_genpass(opts: &GenpassOpts) -> anyhow::Result<()> {
// q: why remove opts: &GenpassOpts?
// a: 因为这里的 process_genpass() 函数不再需要 GenpassOpts 参数，而是直接使用 Opts 参数，所以将 GenpassOpts 参数去掉。
// 直接用单个参数的好处，方便今后这段代码可以直接移植
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    // a: 这里的 rand::thread_rng() 是一个函数，用于生成一个随机数生成器，这个生成器是线程安全的，可以在多线程中使用。
    // q: rand::thread_rng() 返回值是随机生成器，什么是随机生成器？它的值是什么类型？多大范围？
    // a: 随机生成器是一个结构体，它的类型是 ThreadRng，它的范围是 0..u32::MAX。
    // q: u32 是32字节？
    // a: u32 是一个无符号32位整数，它的范围是 0..2^32-1，即 0..4294967295。
    // q: 为什么要用 rand::thread_rng() 生成随机数？
    // a: 因为 rand::thread_rng() 生成的随机数是线程安全的，可以在多线程中使用。
    // q: 那么 rand::thread_rng() 随机生成的数值，在 0..4294967295 范围内，如何使用？
    // a: 可以使用 rand::Rng 的方法，如 gen_range()，生成一个指定范围内的随机数。
    // q: rand::Rng 是什么？它与 rand::thread_rng() 和 gen_range() 如何配合使用？
    // a: rand::Rng 是一个 trait，它定义了生成随机数的方法，rand::thread_rng() 返回的 ThreadRng 结构体实现了 rand::Rng trait，所以可以使用 gen_range() 方法生成随机数。
    // q: rand::Rng trait 是什么？它有哪些方法？
    // a: rand::Rng trait 定义了生成随机数的方法，如 gen_range()、gen()、fill() 等。
    let mut rng = rand::thread_rng();
    let mut password = Vec::new(); //String::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won't be empty in this context"),
        );
    }
    // q: what does b mean in b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"?
    // a: b"ABCDEFGHIJKLMNOPQRSTUVWXYZ" 是一个字节字符串字面量，它的类型是 &[u8; 26]，表示一个包含26个字节的数组。

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won't be empty in this context"),
        );
    }

    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(
            *NUMBERS
                .choose(&mut rng)
                .expect("NUMBERS won't be empty in this context"),
        );
    }

    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(
            *SYMBOLS
                .choose(&mut rng)
                .expect("SYMBOLS won't be empty in this context"),
        );
    }

    // if chars.is_empty() {
    //     anyhow::bail!("At least one of --uppercase, --lowercase, --digits, or --symbols must be specified");
    // }

    // for _ in 0..length - password.len() as u8 {
    //     password.push(*chars.choose(&mut rng).expect("chars won't be empty in this context"));
    // }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    println!("{:?}", password);
    // q: what does from_utf8_lossy() do? compare it with from_utf8()? explain it in english.
    // a: from_utf8_lossy() is a method that converts a byte slice to a string slice, it replaces invalid UTF-8 sequences with the Unicode replacement character U+FFFD, it is lossy because it may lose information, from_utf8() is a method that converts a byte slice to a string slice, it returns an error if the byte slice is not valid UTF-8, it is not lossy because it does not lose information.

    // output password strength in stderr
    let estimate = zxcvbn(&password, &[])?;
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
