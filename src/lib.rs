#![no_std]

//! no_std wrapper around a simple key-value array designed for use in an ISR; the interface is
//!
//! ```ignore
//! let mut array: MinArray<T> = MinArray::new();
//!
//! // Push a value onto the array with the given key
//! array.push(key: u32, value: T);
//!
//! // Take all elements from the array with key less than `filter`
//! for key, value in array.take_less_than(filter: u32) {
//!   // do stuff
//! }
//! ```

const ARRAY_SIZE: usize = 8;

pub struct MinArray<T> {
    data: [Option<(u32, T)>; ARRAY_SIZE],
    pub min: u32,
    pub len: usize,
}

impl<T> MinArray<T> {
    pub const fn new() -> MinArray<T> {
        MinArray {
            data: [None, None, None, None, None, None, None, None],
            min: 0xffffff,
            len: 0,
        }
    }

    pub fn push(&mut self, key: u32, value: T) -> Result<(), T> {
        if self.len >= ARRAY_SIZE {
            return Err(value);
        }

        self.data[self.len] = Some((key, value));
        if key < self.min {
            self.min = key;
        }
        self.len += 1;
        Ok(())
    }

    pub fn take_less_than(&mut self, key: u32) -> MinArrayIterator<'_, T> {
        MinArrayIterator {
            array: self,
            index: 0,
            key,
        }
    }
}

pub struct MinArrayIterator<'a, T> {
    array: &'a mut MinArray<T>,
    index: usize,
    key: u32,
}

impl<'a, T> MinArrayIterator<'a, T> {
    fn swap_remove(&mut self) -> Option<(u32, T)> {
        self.array.len -= 1;
        self.array.data.swap(self.index, self.array.len);
        self.array.data[self.array.len].take()
    }

    fn recompute_min(&mut self) {
        self.array.min = 0xffffffff;
        for i in 0..self.array.len {
            if let Some((key, _)) = self.array.data[i] {
                if key < self.array.min {
                    self.array.min = key;
                }
            }
        }
    }
}

impl<'a, T> Iterator for MinArrayIterator<'a, T> {
    type Item = (u32, T);
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.array.len {
            if let Some((key, _)) = self.array.data[self.index] {
                if key < self.key {
                    let (_, value) = self.swap_remove().unwrap();
                    return Some((key, value));
                } else {
                    self.index += 1;
                }
            }
        }
        self.recompute_min();
        None
    }
}

#[cfg(test)]
#[path = "./test.rs"]
mod test;
