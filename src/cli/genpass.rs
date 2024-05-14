use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(short, long, default_value = "16")]
    pub length: u8,
    // q: default_value = "16" or default_value = "32")
    // a: 这里的 default_value 是一个字符串字面量，所以，可以是任意长度的字符串，只要能转换为 u8 类型即可。
    // q: is 16 the byte length or the character length?
    // a: 这里的 16 是字符长度，不是字节长度。
    // q: 如果包括中文与英文字符混合呢？怎么计算？
    // a: 这里的 16 是字符长度，不是字节长度，所以，无论是中文还是英文，都是一个字符，都是一个长度。
    // q: default_value or default_value_t
    // a: default_value_t 是一个泛型，可以指定类型，而 default_value 是一个字符串字面量，只要能转换为指定类型即可。
    // q: what is the difference between default_value = "16" and default_value_t = 16?
    // a: default_value = "16" 是一个字符串字面量，需要转换为 u8 类型，而 default_value_t = 16 是一个 u8 类型，不需要转换。
    // q: default_value = "16" or default_value = 16, which statement is better?
    // a: default_value = "16" is better, because it's more flexible, and can be used in more situations.
    #[arg(long, default_value = "true")]
    pub uppercase: bool,

    #[arg(long, default_value = "true")]
    pub lowercase: bool,

    #[arg(long, default_value = "true")]
    pub numbers: bool,

    #[arg(long, default_value = "true")]
    pub symbols: bool,
}
