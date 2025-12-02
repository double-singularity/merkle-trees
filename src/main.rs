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

    let root = layers.last().unwrap()[0].clone();
    
    println!("Total Tree Height: {}", layers.len());

    let target_txn = "Tx2";
    let target_idx = 1;
    let proof = get_proof(&layers, target_idx);

    let is_valid = verify(&root, target_idx, target_txn, &proof);
    println!("Is Tx2 valid? {}", is_valid); // Should be true

    // Try verify with wrong data
    let is_valid_fake = verify(&root, target_idx, "Tx99", &proof);
    println!("Is Tx99 valid? {}", is_valid_fake); // Should be false
}

fn get_proof(layers: &Vec<Vec<String>>, index: usize) -> Vec<String> {
    let mut proof = Vec::new();
    let mut current_index = index;
    for i in 0..layers.len() - 1 {
        let sibling_index;
        let layer = &layers[i];

        if layer.len() % 2 == 1 && current_index == layer.len() - 1 {
            sibling_index = current_index;
        } else {
            if current_index % 2 == 0 {
                sibling_index = current_index + 1;
            } else {
                sibling_index = current_index - 1;
            }
        }

        if sibling_index < layer.len() {
            proof.push(layer[sibling_index].clone());
        } else {
            proof.push(layer[current_index].clone());
        }

        current_index = current_index / 2;
    }

    proof
}

fn verify(root: &str, index: usize, data: &str, proof: &Vec<String>) -> bool {
    let mut current_hash = hash(data);
    let mut current_index = index;

    for sibling_hash in proof {
        
        let combined;

        if current_index % 2 == 0 {
            combined = format!("{}{}", current_hash, sibling_hash);
        } else {
            combined = format!("{}{}", sibling_hash, current_hash);
        }

        current_hash = hash(&combined);
        
        current_index /= 2;
    }

    current_hash == root
}

fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    format!("{:x}", result)
}
