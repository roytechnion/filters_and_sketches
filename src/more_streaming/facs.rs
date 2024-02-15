use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::{FlowId,more_streaming::nitro_cms::NitroCMS};
use crate::more_streaming::traits::{ItemIncrement,ItemQuery};

/// FAst Combined Sketch
/// In this sketch design, we divide the stream into windows of size window_size
/// During the current window, we add items to a hashmap, or compact hashmap like Counting Cuckoo Filter
/// At the end of the window, we update the permanent sketch with respective values and clear the window hash/cuckoo

const DEFAULT_WINDOW: u32 = 10_000;

#[derive(Debug)]
pub struct FACS {
    window_sketch: HashMap<FlowId, u32>,
    permanent_sketch: NitroCMS<FlowId, u32>,
    next_item: u32,
    window_size: u32,
}

impl FACS
{
    pub fn new(sample_prob: f64) -> Self
    {
        let window_sketch:HashMap<FlowId, u32> = HashMap::new();
        //let permanent_sketch: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, 1.0 , !(config.avoid_mi), ());
        let permanent_sketch: NitroCMS<FlowId, u32> = NitroCMS::new(0.01, 0.01, 1.0 , true, ());
        let next_item = 0;
        let window_size = DEFAULT_WINDOW;
        Self {
            window_sketch,
            permanent_sketch,
            next_item,
            window_size,
        }
    }

    // TODO: add a parameterized constructor

    /// "Visit" an element: add 1 to the item's count in window_sketch
    /// If we completed the window, add all values to the respective items' count in permanent_sketch and reset window_sketch
    pub fn insert(&mut self, id: FlowId) 
	{
        self.window_sketch.item_increment(id);
        self.next_item += 1;
        if self.next_item % self.window_size == 0 { // TODO: spawn in a separate thread
            self.next_item = 0;
            for (key, val) in self.window_sketch.iter() {
                self.permanent_sketch.push(key, val);
            }
            self.window_sketch.clear();
        }
    }

    /// return an item's estimated count by combining the results from the current window with permanent count
    pub fn get(&self, id: FlowId) -> u32
    {
        return self.window_sketch.item_query(id) + self.permanent_sketch.item_query(id);
    }

    /// return the hash table's capacity
    pub fn capacity(&self) -> usize
    {
        // TODO - what should we do here?
        return self.window_sketch.capacity();
    }

    /// return the actual number of unique items in the hash table
    pub fn len(&self) -> usize
    {
        // TODO - what should we do here?
        return self.window_sketch.len();
    }

    /// returns an estimation of the memory used by permanent sketch
	pub fn estimate_permanent_memory_size(&self) -> usize {
        return self.permanent_sketch.estimate_memory_size();
   }
}

#[cfg(test)]
use crate::id_from_line;

mod tests {
    use crate::{id_from_line, FlowId};

    const TEST_PROBABILITY: f64 = 0.01;
    const TEST_N_ITEMS: usize = 1_000_000;
    const TEST_ERROR_TOLERANCE: usize = 1;

    #[test]
    fn test_increment() {
        let id: FlowId = id_from_line("1 2 3 4 5 6 7 8").unwrap();
		let mut facs:super::FACS = super::FACS::new(TEST_PROBABILITY);
		for _ in 0..TEST_N_ITEMS {
			let _ = facs.insert(id);
		}
		assert!(TEST_N_ITEMS.abs_diff(usize::try_from(facs.get(id)).unwrap()) < TEST_ERROR_TOLERANCE, "DIFF facs = {}", facs.get(id));
	}
}