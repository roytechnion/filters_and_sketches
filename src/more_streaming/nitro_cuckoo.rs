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
    geo: Geometric,
    factor: usize,
    curr_index: usize,
    next_index: usize,
}

impl <H>NitroCuckoo<H> 
where
H: Hasher+Default,
{
    pub fn new(sample_prob: f64) -> Self 
    {
            Self::with_capacity(DEFAULT_CAPACITY,sample_prob)
    }  
//        let counters = CuckooCountingFilter::<K>::new();
//        let geo = Geometric::new(sample_prob).unwrap();
//        let factor = f64_to_usize((1.0/sample_prob).round());
//        let curr_index = 0;
//        let next_index = 0;
//        Self {
//            counters,
//            geo,
//            factor,
//            curr_index,
//            next_index,
//        }
//    }

    pub fn with_capacity(cap: usize, sample_prob: f64) -> Self 
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
    pub fn add<T: ?Sized + Hash>(&mut self, id: &T) -> Result<(), CuckooError>
	{
        if self.next_index - self.curr_index > 0 {
            self.curr_index += 1;
            Ok(())
        } else {
            self.calc_skip();
            self.counters.add(&id)
        }
    }

    pub fn get<T: ?Sized + Hash>(&self, id: &T) -> u32
    {
        self.counters.get(&id)
    }

    fn calc_skip(&mut self) -> () {
        self.next_index = self.curr_index + self.geo.sample(&mut rand::thread_rng()) as usize
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_increment() {
		let mut nh:super::NitroCuckoo<DefaultHasher> = super::NitroCuckoo::new(0.01);
		for _ in 0..30_000 {
			let _ = nh.add("key");
		}
		assert!(30_000u32.abs_diff(nh.get("key")) < 3_000, "DIFF nh = {}", nh.get("key"));
	}
}