use std::hash::Hash;
use std::collections::HashMap;
use super::f64_to_usize;
use std::fmt::Debug;
use rand;
use rand_distr::{Geometric, Distribution};

/// A simple hash table of counters with geometric sampling

#[derive(Debug)]
pub struct NitroHash<K: Hash + std::cmp::Eq, V> {
    counters: HashMap<K, V>,
    geometric_distribution_provider: Geometric,
    factor: V,
    item_skip: usize,
}

impl <K, V>NitroHash<K,V> 
where
K: Clone + Hash + std::cmp::Eq,
V: std::ops::Add<Output=V> + std::ops::AddAssign + TryFrom<usize> + TryFrom<u64> + TryFrom<u32> + TryFrom<u16> + TryFrom<u8> + Copy + super::traits::VtoUsize + std::fmt::Debug +  std::ops::Mul<Output = V>
{
    pub fn new(sample_prob: f64) -> Self 
    where <V as TryFrom<usize>>::Error: Debug
    {
        let counters = HashMap::new();
        let geometric_distribution_provider = Geometric::new(sample_prob).unwrap();
        let factor = V::try_from(f64_to_usize((1.0/sample_prob).round())).unwrap();
        let item_skip = 0;
        Self {
            counters,
            geometric_distribution_provider,
            factor,
            item_skip,
        }
    }

    /// "Visit" an element - sampled version - only update sampled cpunters
    pub fn insert(&mut self, id: K) 
    where <V as TryFrom<u8>>::Error: Debug
	{
        if self.item_skip > 0 {
            self.item_skip -= 1;
        } else { 
            if let Some(counter) = self.counters.get_mut(&id) {
                *counter += V::try_from(1_u8).unwrap();
            } else {
                self.counters.insert(id,V::try_from(1_u8).unwrap());
            }
            self.item_skip = self.geometric_distribution_provider.sample(&mut rand::thread_rng()) as usize;

        }
    }

    /// return an item's estimated count
    pub fn get(&self, id: K) -> V
    where <V as TryFrom<u8>>::Error: Debug
    {
        if let Some(val) = self.counters.get(&id) {
            return *val * self.factor;
        }
        return V::try_from(0_u8).unwrap();
    }

    /// return the hash table's capacity
    pub fn capacity(&self) -> usize
    {
        return self.counters.capacity();
    }

    /// return the actual number of unique items in the hash table
    pub fn len(&self) -> usize
    {
        return self.counters.len();
    }
}

#[cfg(test)]
mod tests {
    const TEST_PROBABILITY: f64 = 0.01;
    const TEST_N_ITEMS: usize = 30_000;
    const TEST_ERROR_TOLERANCE: usize = 3_000;

    #[test]
    fn test_increment() {
		let mut nitrohash:super::NitroHash<&str,u32> = super::NitroHash::new(TEST_PROBABILITY);
		for _ in 0..TEST_N_ITEMS {
			let _ = nitrohash.insert("key");
		}
		assert!(TEST_N_ITEMS.abs_diff(usize::try_from(nitrohash.get("key")).unwrap()) < TEST_ERROR_TOLERANCE, "DIFF nitrohash = {}", nitrohash.get("key"));
	}
}