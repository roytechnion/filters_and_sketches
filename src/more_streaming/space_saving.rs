// We need a priority queue of size 1/epsilon CMAX and remember this to avoid recalc
// The priority queueu stores the flowid and counter as its priority
// We need a capacity counter C
// For each arrival of x:
//   if x in PQ, increment its counter (priority)
//   else if C < CMAX
//      insert x to HM with counter (priority) 1
//   else {
//      find min counter
//      if not RAP or (with probability 1/(min+1))
//         replace entry with x and counter=(min+1) (priority=(min+1))
//   }

use std::hash::Hash;
use priority_queue::DoublePriorityQueue;
use super::f64_to_usize;
use rand::Rng;
use std::fmt::Debug;
use increment::*;

/// An implementation of the space saving algorithm of Metwally, Agrawal, and El Abbadi w/out the
/// RAP optimization of Ben Basat, Chen, Einziger, Friedman, and Kassner

#[derive(Debug)]
pub struct SpaceSaving<K: Hash + std::cmp::Eq, V: std::cmp::Ord> {
    counters: DoublePriorityQueue<K, V>,
    capacity: usize,
    num: usize,
    rap: bool, // indicates whether we execute the RAP optimization
}

impl <K, V>SpaceSaving<K,V> 
where
K: Clone + Hash + std::cmp::Eq,
V: std::cmp::Ord + std::ops::Add<Output=V> + std::ops::AddAssign + TryFrom<u8> + Copy + super::traits::VtoUsize + std::fmt::Debug + increment::Incrementable
{
    pub fn new(error: f64, rap: bool) -> Self {
        let capacity = f64_to_usize((1.0/error).round());
        let counters = DoublePriorityQueue::with_capacity(capacity);
        let num = 0;
        Self {
            counters,
            capacity,
            num,
            rap // are we implementaing the RAP optimization
        }
    }

    /// insert an item into space saving (or add its counter if exists)
    pub fn insert(&mut self, id: K) 
    where <V as TryFrom<u8>>::Error: Debug
    {
        if let Some(counter) = self.counters.get_priority(&id) {
            self.counters.change_priority(&id,increment!(*counter).unwrap());
        } else {
            if self.num < self.capacity {
                self.counters.push(id, V::try_from(1_u8).unwrap());
                self.num += 1;
            } else {
                let (_minkey,minval) = self.counters.peek_min().unwrap();
                let added = increment!(*minval).unwrap();
                if !self.rap || self.coin_flip(added.v_to_usize()) {
                    self.counters.pop_min(); // todo - assert that we got the same as in peek
                    self.counters.push(id,added);
                }
            }
        }
    }

    /// return an estimate of an item's value from space saving
    pub fn get(&self, id: K) -> V {
        if let Some(val) = self.counters.get_priority(&id) {
            return *val;
        }
        let (_,val) = self.counters.peek_min().unwrap();
        return *val;

    }

    /// returns the number of entries in space saving
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    fn coin_flip(&self, probability: usize) -> bool {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..probability) == 0;
    }
}

#[cfg(test)]
mod tests {
    const TEST_ERROR: f64 = 0.01;
    const TEST_N_ITEMS: usize = 30_000;
    const TEST_ERROR_TOLERANCE: usize = 400;

    #[test]
    fn test_increment() {
		let mut spacesaving:super::SpaceSaving<&str,u32> = super::SpaceSaving::new(TEST_ERROR,false);
		for _ in 0..TEST_N_ITEMS {
			let _ = spacesaving.insert("key");
		}
		assert!(TEST_N_ITEMS.abs_diff(usize::try_from(spacesaving.get("key")).unwrap()) < TEST_ERROR_TOLERANCE, "DIFF spacesaving = {}", spacesaving.get("key"));
	}
    #[test]
    fn test_rap_increment() {
		let mut spacesaving:super::SpaceSaving<&str,u32> = super::SpaceSaving::new(TEST_ERROR,true);
		for _ in 0..TEST_N_ITEMS {
			let _ = spacesaving.insert("key");
		}
		assert!(TEST_N_ITEMS.abs_diff(usize::try_from(spacesaving.get("key")).unwrap()) < TEST_ERROR_TOLERANCE, "DIFF spacesaving = {}", spacesaving.get("key"));
	}
}