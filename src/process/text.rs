// cargo add blake3
// cargo add ed25519_dalek

// cargo run -- text sign
// 先用 genpass 生成 private key 的例子
// cargo run -- genpass -l 32 > fixtures/blake3.txt // 生成一个 32 字节的密码
// cargo run -- text sign -i fixtures/blake3.txt -k fixtures/blake3.tx -f blake3 > fixtures/blake3.sig // 用 blake3 签名

// 再改写代码，用 generate 方法分别生成 blake3 的 sk，以及 ed25519 的 sk 与 pk。
// cargo run -- text generate -o fixtures // 生成一个 blake3.txt 文件，里面包含了一个 32 字节的 key
// cargo run -- text generate -o fixtures --format ed25519 // 将生成 ed25519.sk 与 ed25519.pk 两个保存私钥公钥的文件

// blake3
// cargo run -- text sign -k fixtures/blake3.txt // 等待命令行对话框交互，输入任意字符串比如 hello world!，然后ctrl+d两次
// 最终获得 hashed value: ****
// 验证，首先把 blake3.txt 中的 hashed value(类似本例中的 signature)，拷贝，然后，加在下述命令行后面的 --sig 之后
// cargo run -- text verify -k fixtures/blake3.txt --sig *****

// ed25519
// cargo run -- text sign -k fixtures/ed25519.sk --format ed25519
// 一个bug：hello world!
// cargo run -- text verify -k fixtures/ed25519.pk --format ed25519 --sig -P1Ncx2h36smy_NWgP8hb-iTQPc0RKt7qsKysITbs-cNUL1H_QW30q34POfvkJrlhpJmQuU2_Ep_8K06Iv2EBQ
// error: unexpected argument '-P' found

// 测试
// cargo nextest run -- test_blake3_sign_verify --nocapture //with --nocapture, You will see all the println! output in real-time, regardless of whether the test passes or fails. 方便 println! 调试。
// cargo nextest run -- test_ed2519_sign_verify

use std::{fs, io::Read, path::Path};

use crate::{get_reader, process_genpass, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

// use super::gen_pass;
// q: is the SigningKey implementing the Signer trait? while the VerifyingKey implementing the Verifier trait?
// a: Yes, the SigningKey implements the Signer trait, which defines the sign function for generating signatures, while the VerifyingKey implements the Verifier trait, which defines the verify function for verifying signatures. The SigningKey is used to sign data, while the VerifyingKey is used to verify the signature of the data.

pub struct Blake3 {
    key: [u8; 32],
}

pub struct ED25519Signer {
    // key: [u8; 64],
    key: SigningKey,
}

pub struct ED25519Verifier {
    // key: [u8; 32],
    key: VerifyingKey,
}

// q: what is SigningKey and VerifyingKey?
// a: SigningKey 是一个签名密钥，用于对数据进行签名，VerifyingKey 是一个验证密钥，用于验证数据的签名。
// q: is SigningKey sort of private key? explain it in english.
// a: Yes, SigningKey is a private key, which is used to sign data, and VerifyingKey is a public key, which is used to verify the signature.
// q: how can i generate a SigningKey?
// a: You can generate a SigningKey by calling SigningKey::generate() function.
// q: what is algorithm used by SigningKey::generate()?
// a: The algorithm used by SigningKey::generate() is Ed25519.
// q: what is Ed25519?
// a: Ed25519 is a digital signature algorithm, which is based on the elliptic curve cryptography.
// q: in lower version of Ed25519, what function it offers in place of Signingkey::generate()?
// a: In lower version of Ed25519, you can use SigningKey::random() function to generate a SigningKey.
// q: what is the difference between SigningKey::generate() and SigningKey::random()?
// a: SigningKey::generate() is a deterministic function, which generates the same key every time, while SigningKey::random() is a non-deterministic function, which generates a random key every time.
// q: Keypair::generate vs SigningKey::generate
// a: Keypair::generate is a function that generates a key pair, which contains a SigningKey and a VerifyingKey, while SigningKey::generate is a function that generates a SigningKey.
// q: are SigningKey and VerifyingKey a pair? are they a type of [u8; n]?
// a: Yes, SigningKey and VerifyingKey are a pair, which contains a private key and a public key, and they are not a type of [u8; n].
// q: what type of data SigningKey and VerifyingKey are?
// a: SigningKey and VerifyingKey are a type of struct, which contains a private key and a public key.

pub trait TextSign {
    /// Sign the data from the reader and return the signature
    // fn sign(&self, data: &str) -> Result<Vec<u8>>;
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify the data from the reader with the signature
    // fn verify(&self, reader: &dyn Read, sig: &[u8]) -> Result<bool>; // 动态分配
    fn verify(&self, reader: &mut impl Read, sig: &[u8]) -> Result<bool>; // 动态分配
                                                                          // fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>; // 静态分配
                                                                          // "reader: &dyn Read", or "reader: impl Read", or "<R: Read>(reader: R)"?
                                                                          // a: "reader: &dyn Read" 和 "reader: impl Read" 都是动态分配，而 "<R: Read>(reader: R)" 是静态分配。
                                                                          // q: what is the difference between dynamic dispatch and static dispatch?
                                                                          // a: 动态分配是在运行时确定函数的调用，而静态分配是在编译时确定函数的调用。
                                                                          // q: "reader: &dyn Read" 和 "reader: impl Read" 都是动态分配，那么它们之间有什么区别呢？
                                                                          // a: "reader: &dyn Read" 是一个引用，而 "reader: impl Read" 是一个实现，引用是一个指向数据的指针，而实现是一个数据的具体类型。
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized; // 这里的 Sized 是一个 trait bound，指明数据结构需要固定长度，即不能是动态长度的。not str, [u8], Vec<u8>, Box<dyn Read>。
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>; // 生成一个（blake3），或者多个密钥（ED25519 的 SigningKey，VerifyingKey）
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32]; // 取 key 文件的前 32 个字节，作为 key，这里的 key 是一个 &[u8] 类型的数组，是 slice。
        let key = key.try_into()?; // 将 key 转换为 [u8; 32] 类型的数组
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl ED25519Signer {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&key),
        }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = ED25519Signer::new(key);
        Ok(signer)
    }
}

impl ED25519Verifier {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key: VerifyingKey::from_bytes(&key).unwrap(),
        }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let verifier = ED25519Verifier::new(key);
        Ok(verifier)
    }

    // pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
    //     let key = fs::read(path)?;
    //     Self::try_new(&key)
    // }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec()) // 需要 cargo add blake3，然后 Cargo.toml 中会自动添加 blake3 = "1.5.1"。
                                                                    // q: what does blake3::hash(&buf).as_bytes().to_vec() do?
                                                                    // a: blake3::hash(&buf) 是一个 blake3 算法的哈希函数，用于计算 buf 的哈希值，as_bytes() 是将哈希值转换为字节数组，to_vec() 是将字节数组转换为 Vec 类型。
                                                                    // q: what is blake3 of blake3::hash(&buf)? 为什么我可以在这里使用它，而不需要导入它？
                                                                    // a: blake3 是一个 crate，用于计算哈希值，这里的 blake3::hash(&buf) 是一个函数调用，用于计算 buf 的哈希值，不需要导入，因为它是一个全局函数。
                                                                    // q: 请把这段话翻译为英语：“为什么我可以在这里使用它，而不需要导入它？”
                                                                    // a: Why can I use it here without importing it?
                                                                    // q: 在 Cargo.toml 中添加了 blake3 = "1.5.1" 之后，为什么不需要导入 blake3 crate？
                                                                    // a: 因为 blake3 crate 是一个全局 crate，不需要导入就可以使用。
                                                                    // q: 在 Cargo.toml 的 [dependencies] 中罗列了很多 crate，这些都是全局 crate 吗？
                                                                    // a: 不是，只有一些常用的 crate 是全局 crate，其他的 crate 需要导入才能使用。
                                                                    // q: anyhow 是不是全局 crate？
                                                                    // a: 是的，anyhow 是一个全局 crate，不需要导入就可以使用。
                                                                    // q: 在我现有的 Cargo.toml 的 [dependencies] 中罗列了很多 crate，哪些是全局 crate？
                                                                    // a: 在你现有的 Cargo.toml 的 [dependencies] 中，anyhow、base64、ed25519_dalek、rcli、serde、serde_json、structopt、thiserror 这些都是全局 crate。
    }
}
// In above code, Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec()) generates a hash value using the BLAKE3 hashing algorithm with a key, making it a keyed hash. This keyed hash acts as a signature because it’s unique to both the data (&buf) and the key (&self.key). The verification process then involves creating a new hash from the same data and comparing it to the signature to ensure they match, confirming the data’s integrity.
// So, in summary, the signature in your code is a keyed hash value produced by the BLAKE3 algorithm, which serves as a cryptographic signature for the data. It’s used to verify that the data has not been tampered with and that it was indeed signed with the corresponding private key.

impl TextSign for ED25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?; // test 中的例子，可能是命令行交互输入：hello world!。
                                       // println!("buf: {:?}", String::from_utf8_lossy(&buf));
                                       // q: what does String::from_utf8_lossy(&buf) do?
                                       // a: String::from_utf8_lossy(&buf) is a function that converts a Vec<u8> into a String, replacing any invalid UTF-8 sequences with the Unicode replacement character �.
                                       // q: URL_SAFE_NO_PAD.encode(&buf) or String::from_utf8_lossy(&buf)?
                                       // a: URL_SAFE_NO_PAD.encode(&buf) is a function that encodes a Vec<u8> into a base64 string that is safe to include in URLs and does not have any padding characters at the end, while String::from_utf8_lossy(&buf) is a function that converts a Vec<u8> into a String, replacing any invalid UTF-8 sequences with the Unicode replacement character �.
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes(); // return a reference to [u8, 32]
                                    // println!("new sig: {}", URL_SAFE_NO_PAD.encode(hash)); // 此处的 hash 类型就是一个引用
        Ok(hash == sig) // 这里的 sig 其实不是签名，而是哈希值。与 ED25519Verifier 中的签名不同。
    }
}

impl TextVerify for ED25519Verifier {
    fn verify(&self, reader: &mut impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        // 因为已经确定是文件路径，所以，这里用 AsRef<Path>，而不再是 &str。
        let key = fs::read(path)?; // fs::read 用于文件较小，一次性读到内存中，比如hash值之类的。
        Self::try_new(&key)
    }
}

impl KeyLoader for ED25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for ED25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        // let key = [0u8, 32];
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.into_bytes(); // key.into_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for ED25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng); //generate方法需要添加 rand_core，Available on crate feature rand_core only. // cargo add ed25519_dalek --features rand_core
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();

        Ok(vec![sk, pk])
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::ED25519 => {
            let signer = ED25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };

    let signed = URL_SAFE_NO_PAD.encode(signed);
    // q: is URL_SAFE_NO_PAD a constant? how could a constant have a method?
    // a: URL_SAFE_NO_PAD is a constant, which is a static variable, and it can have a method, because it is a type of Engine, which is a trait.
    // when you use URL_SAFE_NO_PAD.encode(&signed), it encodes the signed data into a base64 string that is safe to include in URLs and does not have any padding characters at the end.
    // println!("{}", signed);

    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> anyhow::Result<bool> {
    let mut reader = get_reader(input)?; // reader is not an appropriate name, it should be something like signature_reader here.

    let sig = URL_SAFE_NO_PAD.decode(sig.as_bytes())?; // 此处 signature 是一个 base64 编码的字符串，不是文件，需要解码成字节数组。
    let sig = sig.as_slice();

    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, sig)?
        }
        TextSignFormat::ED25519 => {
            let verifier = ED25519Verifier::load(key)?;
            verifier.verify(&mut reader, sig)?
        }
    };

    // if verified {
    //     println!("Signature is valid");
    // } else {
    //     println!("Signature is invalid");
    // }

    Ok(verified)
}
// q: what is key: &str?
// a: key: &str 是一个字符串引用，用于指向 key 文件的路径。

pub fn process_text_generate(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::ED25519 => ED25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign() -> anyhow::Result<()> {
        let key = [0; 32];
        let signer = Blake3::new(key);
        let data = b"hello world!";
        let sig = signer.sign(&mut &data[..])?;
        assert_eq!(sig.len(), 32);
        Ok(())
    }

    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        // let key = [0; 32];
        // let signer = Blake3::new(key);
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world!";
        let sig = blake3.sign(&mut &data[..])?;
        // println!("sig: {}", URL_SAFE_NO_PAD.encode(&sig));
        // q: why do i need to encode the signature?
        // a: You need to encode the signature because it is a binary data, and you need to convert it to a string so that you can print it to the console.
        // q: so , encoding is a way to convert binary data to string?
        // a: Yes, encoding is a way to convert binary data to a string, and decoding is a way to convert a string back to binary data.
        let verified = blake3.verify(&mut &data[..], &sig)?;
        assert!(verified);
        Ok(())
    }

    #[test]
    fn test_ed2519_sign_verify() -> anyhow::Result<()> {
        let sk = ED25519Signer::load("fixtures/ed25519.sk")?;
        let pk = ED25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"hello world!";
        let sig = sk.sign(&mut &data[..])?;
        let verified = pk.verify(&mut &data[..], &sig)?;
        assert!(verified);

        Ok(())
    }
}
