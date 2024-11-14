use std::fs::File;
use std::io::Read;

// input 是一个文件路径
// get_reader 函数返回一个 Box<dyn Read> 类型的 trait object
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?) // 区别于 fs::read(path)?; fs::read 用于文件较小，一次性读到内存中，比如hash值之类的。
    };
    Ok(reader)
}

// 用 P: AsRef<Path> 改写上述代码
// pub fn get_reader<P: AsRef<Path>>(input: P) -> anyhow::Result<Box<dyn Read>> {
//     let reader: Box<dyn Read> = if input.as_ref() == Path::new("-") {
//         Box::new(io::stdin())
//     } else {
//         Box::new(File::open(input)?)
//     };
//     Ok(reader)
// }

// fn main() -> anyhow::Result<()> {
//     let reader1 = get_reader("file.txt")?;           // &str
//     let reader2 = get_reader(String::from("file.txt"))?; // String
//     let reader3 = get_reader(Path::new("file.txt"))?;    // &Path
//     let reader4 = get_reader(std::path::PathBuf::from("file.txt"))?; // PathBuf

//     // Use the readers...

//     Ok(())
// }

// The preferable version of get_reader is the one that uses impl AsRef<Path> for its parameter, as it offers greater flexibility and ease of use compared to the &str version.

// Advantages of Using impl AsRef<Path>
// Flexibility with Path Types: It allows the function to accept various types of path-like arguments such as &str, String, &Path, and PathBuf. This means users of the function can pass different types without needing to convert them explicitly to a &str or Path.

// Convenience for Callers: Callers do not need to convert their input into a specific format, making the function more convenient and ergonomic to use.

// Rust Idiomatic: Using impl AsRef<Path> is idiomatic in Rust for functions dealing with file paths because it leverages Rust’s powerful type system and trait-based design to accept a broad range of input types.

// Code Reusability: You avoid the need to write multiple overloads or conversions in the function's body, leading to cleaner and more maintainable code.

// q: is the reader a type of Vec<u8>?
// a: No, the reader is a type of Box<dyn Read>.
// q: how can i get the reader's content?
// a: You can get the reader's content by calling the read_to_end() method on the reader.
// q: who owns the read_to_end() method?
// a: The read_to_end() method is owned by the Read trait.
// q: the return value of read_to_end() is a Result type, what is inside the Ok value of Result type? is it a type of Vec<u8>?
// a: Inside the Ok value of Result type is a value of the Vec type, which is a type of Vec<u8>.

// q: what does Box<dyn Read> mean? explain it in english.
// a: Box<dyn Read> is a trait object, which is a pointer to a value of any type that implements the Read trait.
// q: what is the difference between Box<dyn Read> and Box<Read>?
// a: Box<dyn Read> is a trait object, which is a pointer to a value of any type that implements the Read trait, while Box<Read> is a concrete type, which is a pointer to a value of the Read type.
// q: what is the difference between Box<dyn Read> and Box<Read + 'static>?
// a: Box<dyn Read> is a trait object, which is a pointer to a value of any type that implements the Read trait, while Box<Read + 'static> is a trait object, which is a pointer to a value of any type that implements the Read trait and has a 'static lifetime.
// q: what is Read trait?
// a: Read trait is a trait that provides a way to read bytes from a source.
// q: what type of value does Read trait return?
// a: Read trait returns a value of the Result type.
// q: what is inside the Ok value of Result type?
// a: Inside the Ok value of Result type is a value of the Vec type.
// q: what is inside the Vec type?
// a: Inside the Vec type is a value of the u8 type.

// a: stdin() is a function that returns a handle to the standard input stream.
// q: std::io::stdin() is implemented by Stdin struct. Stdin struct implements Read trait? explain all other questions in english.
// a: std::io::stdin() is a function that returns a handle to the standard input stream, which is implemented by Stdin struct. Stdin struct implements Read trait, which provides a way to read bytes from a source. File struct also implements Read trait, which provides a way to read bytes from a file. Box<dyn Read> is a trait object, which is a pointer to a value of any type that implements the Read trait. Box<Read> is a concrete type, which is a pointer to a value of the Read type. Box<Read + 'static> is a trait object, which is a pointer to a value of any type that implements the Read trait and has a 'static lifetime.
