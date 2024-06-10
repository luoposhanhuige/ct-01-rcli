// cargo run -- base64
// cargo run -- base64 encode
// cargo run -- base64 decode
// cargo run -- base64 encode --format urlsafe -i Cargo.toml > fixtures/b64.txt
// cargo run -- base64 decode --format urlsafe -i fixtures/b64.txt
// cargo nextest run
// cargo nextest run -- test_process_encode

// cargo add base64?
// q: what is base64?
// a: base64 是一个编码算法，用于将二进制数据编码为文本数据。
// q: how to check whether the base64 is added to the dependencies?
// a: 可以查看 Cargo.toml 文件，查看是否有 base64 这个依赖。
// q: how to check whether the base64 is added through the command line?
// a: 可以查看 Cargo.toml 文件，查看是否有 base64 这个依赖。

use crate::{get_reader, Base64Format};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    // q: what does this mean? why we need Vec to store the data?
    // a: 这里的 Vec 是一个动态数组，用于存储读取的数据，因为读取的数据的大小是不确定的，所以需要使用动态数组来存储。

    reader.read_to_end(&mut buf)?;
    // q: since reader is a Box type, how does the compiler know that it is a Read type?
    // a: 因为 Box 类型实现了 Read trait，所以编译器知道 reader 是一个 Read 类型。
    // q: since buf is a Vec type, what does the compiler store the data into buf? by assigning or by pushing?
    // a: 编译器会将读取的数据存储到 buf 变量中，通过 push() 方法。
    // q: 通过push方法，那么最终 buf 中存储的数据是什么？是一个个字节吗？
    // a: 是的，buf 中存储的数据是一个个字节，即 u8 类型的数据。
    // q: how does the compiler handle newline in the data read from the reader?
    // a: 编译器会将读取的数据中的换行符转换为 \n 字符。
    // q: why does reader need to be mutated?
    // a: reader 需要被修改，是因为 read_to_end() 函数需要修改 reader 变量，以便读取数据。
    // q: why do we need to read the data to the end?
    // a: 读取数据到末尾，是为了读取所有的数据，而不是只读取一部分数据。
    // q: why does &mut to be used before buf?
    // a: &mut 是一个引用，用于将 buf 变量的所有权借给 read_to_end() 函数，这样 read_to_end() 函数就可以修改 buf 变量
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    // println!("{}", encoded);
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    // q: why is the result of "get_reader(input)?" a Box type, while the result of "get_reader(input)" is a Result type?
    // q: "get_reader(input)?" 与 "get_reader(input)" 返回的类型有什么不同？

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newline
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // let decoded = String::from_utf8(decoded)?;
    // println!("{}", decoded);

    Ok(decoded)
}

// q: what does this mean?
// a: 这里的 from_utf8() 是一个 trait，用于将一个 &[u8] 类型的数据转换为 String 类型。
// q: what is the return value of from_utf8()?
// a: from_utf8() 的返回值是一个 Result 类型，用于处理转换失败的情况。
// q: vec 是 &[u8] 类型？
// a: 是的，vec 是一个 &[u8] 类型，用于存储二进制数据。

// q: what does this mean with if input == "-"?
// a: 这里的 if input == "-" 是一个条件判断，用于判断 input 是否为 "-"。
// q: what does Box::new(std::io::stdin()) mean?
// a: Box::new(std::io::stdin()) 是一个函数调用，用于创建一个 Box 类型的对象，这个对象是一个标准输入流。
// q: input 如果为 "-"，意味着什么？
// a: input 为 "-"，意味着用户没有指定输入文件，所以需要从标准输入中读取数据。
// q: File::open(input)? 的返回值是什么？为什么要用 ? 运算符？
// a: File::open(input)? 的返回值是一个 Result 类型，用于处理文件打开失败的情况，? 运算符用于处理 Result 类型的错误。
// q: ? 运算符是属于 anyhow 的特性吗？
// a: 不是，? 运算符是 Rust 语言的特性，用于处理 Result 类型的错误。

#[cfg(test)]
mod tests {
    use super::*;
    // what does this mean?
    // a: 这里的 super::* 是一个通配符，用于导入父模块中的所有内容。
    // q: 这里的父模块指的是哪个具体的模块？
    // a: 这里的父模块指的是当前模块的父模块，即当前模块的父模块是 process 模块。
    // q: 为什么要导入父模块中的所有内容？这跟测试有什么关系？
    // a: 导入父模块中的所有内容，是为了在测试中使用父模块中的函数和结构体。

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        // process_encode(input, Base64Format::Standard).unwrap();
        let format = Base64Format::Standard;
        // assert_eq!(process_encode(input, format).is_ok(), true);
        assert!(process_encode(input, format).is_ok());
        // q: what is the difference between "process_encode(input, Base64Format::Standard).unwrap();" and "assert!(process_encode(input, format).is_ok());"?
        // a: "process_encode(input, Base64Format::Standard).unwrap();" 会直接 panic，而 "assert!(process_encode(input, format).is_ok());" 会返回一个 bool 值。
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        // assert!(process_decode(input, format).is_ok());
        process_decode(input, format).unwrap();
    }
}
