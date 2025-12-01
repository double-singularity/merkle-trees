use sha2::{Sha256, Digest};
fn main() {
    println!("{}", hash("hello world"));
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    format!("{:x}", result)
}
