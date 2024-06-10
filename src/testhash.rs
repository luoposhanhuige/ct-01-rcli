// Add the required crates
// extern crate ed25519_dalek;
// extern crate rand;

use rand::rngs::OsRng;
use ed25519_dalek::{Signature, SigningKey};

fn main() {
    // Create an instance of OsRng
    let mut csprng = OsRng;

    // Generate a signing key for Ed25519
    // let signing_key = SigningKey::random(&mut csprng);
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);


    // Extract and display the private and public keys
    let private_key_bytes = signing_key.to_bytes();
    // Obtain the verifying key (public key) from the signing key
    let verifying_key = signing_key.verifying_key();
    let public_key_bytes = verifying_key.to_bytes();

    println!("Private Key: {:?}", private_key_bytes);
    println!("Public Key: {:?}", public_key_bytes);
}



// q: can i rustc testhash.rs to compile this file?
// a: 是的，你可以使用 rustc testhash.rs 命令编译这个文件。
// q: but the compiler tells me that it cannot find the crate ed25519_dalek and rand, why?
// a: 这是因为你没有在 Cargo.toml 文件中添加这两个 crate，你需要在 Cargo.toml 文件中添加这两个 crate，然后再编译这个文件。
// q: how to add these two crates to Cargo.toml?
// a: 你可以使用 cargo add ed25519_dalek rand 命令，将这两个 crate 添加到 Cargo.toml 文件中。
// q: the Cargo.toml already has the dependencies, why do i need to add them again?
// a: 这是因为你的项目中可能有多个 Cargo.toml 文件，你需要在你的项目的根目录下的 Cargo.toml 文件中添加这两个 crate，然后再编译这个文件。
// q: is there any other way to add these two crates to Cargo.toml?
// a: 是的，你可以手动编辑 Cargo.toml 文件，然后添加这两个 crate，然后再编译这个文件。
// q: is it necessary to put this file within the same directory as the Cargo.toml?
// a: 不是的，你可以将这个文件放在任何目录中，只要你在编译这个文件时，指定正确的路径即可。
// q: how can i rustc testhash.rs with telling the path of Cargo.toml?
// a: 你可以使用 rustc --extern ed25519_dalek=ed25519_dalek --extern rand=rand testhash.rs 命令，指定这两个 crate 的路径，然后编译这个文件。
// q: rustc --extern ed25519_dalek=ed25519_dalek --extern rand=rand testhash.rs, still doesn't work
// a: 这是因为你没有指定这两个 crate 的路径，你需要在 --extern 参数后面，指定这两个 crate 的路径，然后再编译这个文件。
// q: how to specify the path of these two crates?
// a: 你可以使用 rustc --extern ed25519_dalek=ed25519_dalek=/path/to/ed25519_dalek --extern rand=rand=/path/to/rand testhash.rs 命令，指定这两个 crate 的路径，然后编译这个文件。
// q: usually, which path should i specify for these two crates?
// a: 你可以使用 cargo locate ed25519_dalek 和 cargo locate rand 命令，查找这两个 crate 的路径，然后再编译这个文件。
// q: error: no such command: `locate`
// a: 这是因为你没有安装 cargo-locate 插件，你需要使用 cargo install cargo-locate 命令，安装这个插件，然后再查找这两个 crate 的路径。
// q: error: could not find `cargo-locate` in registry `crates-io` with version `*`
// a: 这是因为你的 cargo 源没有这个插件，你需要使用 cargo install cargo-locate --git
// q: ❯ cargo local, Error: failed to read `/Users/xhui/dev/rust-chentian/rcli/src/Cargo.toml
// a: 这是因为你没有在正确的目录下运行这个命令，你需要在你的项目的根目录下运行这个命令，然后再查找这两个 crate 的路径。
