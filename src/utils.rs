use std::fs::File;
use std::io::Read;

// input 是一个文件路径
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

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
