use super::f64_to_usize;
use std::fmt::Debug;
use rand;
use rand_distr::{Geometric, Distribution};
use crate::CuckooCountingFilter;
use crate::more_streaming::cuckoo::{CuckooError,DEFAULT_CAPACITY};
use crate::{Hash,Hasher};

/// A wrapper over CuckooCountingFilter that adds nitro (as in NitroSketch) type sampling to it
/// That is, inserts occur with a given probability, but we use geometric distribution to decide
/// each time how many inserts to ignore rather than deciding on each insert whether to accept it

#[derive(Debug)]
pub struct NitroCuckoo<H> {
    counters: CuckooCountingFilter<H>,
    geometric_distribution_provider: Geometric,
    factor: usize,
    item_skip : usize,
}

impl <H>NitroCuckoo<H> 
where
H: Hasher+Default,
{
    pub fn new(sample_probability: f64) -> Self 
    {
            Self::with_capacity(DEFAULT_CAPACITY,sample_probability)
    }  

    /// starts a new filter with a given capacity
    pub fn with_capacity(capacity: usize, sample_probability: f64) -> Self 
    {
        let counters = CuckooCountingFilter::<H>::with_capacity(capacity);
        let geometric_distribution_provider = Geometric::new(sample_probability).unwrap();
        let factor = f64_to_usize((1.0/sample_probability).round());
        let item_skip = 0;
        Self {
            counters,
            geometric_distribution_provider,
            factor,
            item_skip,
        }
    }

    /// "Visit" an element - sampled version - only update sampled counters
    pub fn add<T: ?Sized + Hash>(&mut self, id: &T) -> Result<(), CuckooError>
	{
        if self.item_skip > 0 {
            self.item_skip -= 1;
            Ok(())
        } else {
            self.item_skip = self.geometric_distribution_provider.sample(&mut rand::thread_rng()) as usize;
            self.counters.add(&id)
        }
    }

    /// return an estimate of an item's count
    pub fn get<T: ?Sized + Hash>(&self, id: &T) -> u32
    {
        self.counters.get(&id) * u32::try_from(self.factor).unwrap()
    }

    /// return the capacity of the filter
    pub fn capacity(&self) -> usize {
        self.counters.capacity()
    }

    /// retun the actual number of unique items (fingerprints to be precise) in the filter
    pub fn len(&self) -> usize {
        self.counters.len()
    }

}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    const TEST_PROBABILITY: f64 = 0.01;
    const TEST_N_ITEMS: usize = 30_000;
    const TEST_ERROR_TOLERANCE: usize = 3_000;

    #[test]
    fn test_increment() {
		let mut nitro_filter:super::NitroCuckoo<DefaultHasher> = super::NitroCuckoo::new(TEST_PROBABILITY);
		for _ in 1..=TEST_N_ITEMS {
			let _ = nitro_filter.add("key");
		}
		assert!(TEST_N_ITEMS.abs_diff(usize::try_from(nitro_filter.get("key")).unwrap()) < TEST_ERROR_TOLERANCE, "DIFF nitro_filter = {}", nitro_filter.get("key"));
	}
}