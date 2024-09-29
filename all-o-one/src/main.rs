use std::collections::HashMap;

struct AllOne {
    data: HashMap<String, u16>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            size: 0,
        }
    }

    fn inc(&mut self, key: String) {
        *self.data.entry(key).or_insert(1) += 1;
    }

    fn dec(&mut self, key: String) {
        *self.data.entry(key.clone()).or_insert(1) -= 1;
        if self.data.get(&key) == Some(&0) {
            self.data.remove(&key);
        }
    }

    fn get_max_key(&self) -> String {}

    fn get_min_key(&self) -> String {}
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

fn main() {
    println!("Hello, world!");
}
