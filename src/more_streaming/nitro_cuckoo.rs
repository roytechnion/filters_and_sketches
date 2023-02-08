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
    geo: Geometric, // CR: use clear variable names instead of 'geo'
    factor: usize,
    curr_index: usize, // CR: use clear variable names instead of 'curr_index'. 'current_index' is better.
    next_index: usize,
}

impl <H>NitroCuckoo<H> 
where
H: Hasher+Default,
{
    pub fn new(sample_prob: f64) -> Self // CR: use clear variable names instead of 'sample_prob'
    {
            Self::with_capacity(DEFAULT_CAPACITY,sample_prob)
    }  

    /// starts a new filter with a given capacity
    pub fn with_capacity(cap: usize, sample_prob: f64) -> Self // CR: use clear variable names instead of 'cap'
    {
        let counters = CuckooCountingFilter::<H>::with_capacity(cap);
        let geo = Geometric::new(sample_prob).unwrap();
        let factor = f64_to_usize((1.0/sample_prob).round());
        let curr_index = 0;
        let next_index = 0;
        Self {
            counters,
            geo,
            factor,
            curr_index,
            next_index,
        }
    }

    /// "Visit" an element - sampled version - only update sampled cpunters
    // CR: concider mentioning in the function name what is added?
    pub fn add<T: ?Sized + Hash>(&mut self, id: &T) -> Result<(), CuckooError>
	{
        // CR: Why not 'self.next_index > self.curr_index'?
        if self.next_index - self.curr_index > 0 {
            // CR: You may expose yourself to integer overflow like this. read more at https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
            self.curr_index += 1;
            Ok(())
        } else {
            self.calc_skip();
            self.counters.add(&id)
        }
        // CR: Suggest: You can actually use only one index variable:
        //             - Make it equal to calc_skip, then decrease untill 0.
        //             - When 0, Preform the .add(), then make the index equal to calc_skip again.
        //             - So, you will avoid overflows which will eventually come.
    }

    /// return an estimate of an item's 
    // Should it be clean in the function name that it is an estimation?
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

    // calculate how many updates should be skipped before the next one to be fully processed
    // CR: use clear variable names instead of 'calc_skip', may mention which kind of skips
    fn calc_skip(&mut self) -> () {
        self.next_index = self.curr_index + self.geo.sample(&mut rand::thread_rng()) as usize
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_increment() {
        // CR: use clear variable names instead of 'nh'
        // CR: Replace 0.01 by a declared constant, with indecative name
		let mut nh:super::NitroCuckoo<DefaultHasher> = super::NitroCuckoo::new(0.01);
        // CR: Replace 30_000 by a declared constant, with indecative name
		for _ in 0..30_000 {
			let _ = nh.add("key");
		}
        // CR: Replace 3_000 by a declared constant, with indecative name
		assert!(30_000u32.abs_diff(nh.get("key")) < 3_000, "DIFF nh = {}", nh.get("key"));
	}
}