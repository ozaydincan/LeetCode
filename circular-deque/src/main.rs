struct MyCircularDeque {
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
    data: Vec<Option<i32>>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let capacity = k as usize + 1;
        Self {
            data: vec![None; capacity],
            front: 0,
            rear: 0,
            size: 0,
            capacity,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.front = (self.front + self.capacity - 1) % self.capacity;
        self.data[self.front] = Some(value);
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.rear] = Some(value);
        self.rear = (self.rear + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.data[self.front] = None;
        self.front = (self.front + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.rear = (self.rear + self.capacity - 1) % self.capacity;
        self.data[self.rear] = None;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front].unwrap()
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let rear_index = (self.rear + self.capacity - 1) % self.capacity;
        self.data[rear_index].unwrap()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity - 1
    }
}
fn main() {
    println!("Hello, world!");
}
