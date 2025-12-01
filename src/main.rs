use sha2::{Sha256, Digest};
fn main() {
    let data = vec!["Tx1", "Tx2", "Tx3", "Tx4"];
    
    let mut leaves: Vec<String> = Vec::new();

    for e in data {
        leaves.push(hash(e));
    }

    for e in &leaves {
        println!("{e}");
    }

    let mut layers: Vec<Vec<String>> = Vec::new();
    layers.push(leaves);

    while layers.last().unwrap().len() > 1 {
        let previous_layer = layers.last().unwrap();
        let mut next_layer: Vec<String> = Vec::new();

        for pair in previous_layer.chunks(2) {
            let left = &pair[0];
            let right;
            
            if pair.len() == 2 {
                right = &pair[1];
            } else {
                right = &pair[0];
            }

            let combined = format!("{}{}", left, right);
            next_layer.push(hash(&combined));
        }

        layers.push(next_layer);
    }

    println!("\n--- Merkle Root ---");
    println!("{}", layers.last().unwrap()[0]);
    
    println!("Total Tree Height: {}", layers.len());
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    format!("{:x}", result)
}
