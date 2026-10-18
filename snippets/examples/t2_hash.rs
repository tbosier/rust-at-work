// region: hash
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// endregion

fn main() {
    // region: hash
    let buckets = 8;
    for member in ["Ada", "Grace", "Alan", "Edsger"] {
        let mut h = DefaultHasher::new();
        member.hash(&mut h);
        let hash = h.finish();
        println!("{member:<7} hash {hash:>20}  bucket {}", hash % buckets);
    }
    // endregion
}
