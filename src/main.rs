use cradle::prelude::*;

fn main() {
    let directory = match std::env::args().nth(1) {
        Some(it) => it,
        None => {
            eprintln!("usage: cradle-of-tar directory-to-archive");
            std::process::exit(-1);
        }
    };
    run!(%format!("tar -cf archive.tar {}", directory));
}
