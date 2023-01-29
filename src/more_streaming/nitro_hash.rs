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
    geo: Geometric,
    factor: V,
    curr_index: usize,
    next_index: usize,
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
        let geo = Geometric::new(sample_prob).unwrap();
        let factor = V::try_from(f64_to_usize((1.0/sample_prob).round())).unwrap();
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
    pub fn insert(&mut self, id: K) 
    where <V as TryFrom<u8>>::Error: Debug
	{
        if self.next_index - self.curr_index > 0 {
            self.curr_index += 1;
        } else { 
            if let Some(counter) = self.counters.get_mut(&id) {
                *counter += V::try_from(1_u8).unwrap();
            } else {
                self.counters.insert(id,V::try_from(1_u8).unwrap());
            }
            self.next_index = Self::calc_skip(self.geo,self.curr_index);
        }
    }

    pub fn get(&self, id: K) -> V
    where <V as TryFrom<u8>>::Error: Debug
    {
        if let Some(val) = self.counters.get(&id) {
            return *val * self.factor;
        }
        return V::try_from(0_u8).unwrap();
    }

    fn calc_skip(geo: Geometric, current_index: usize) -> usize {
        let v = geo.sample(&mut rand::thread_rng()) as usize;
        return current_index + v
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_increment() {
		let mut nh:super::NitroHash<&str,u32> = super::NitroHash::new(0.01);
		for _ in 0..30_000 {
			let _ = nh.insert("key");
		}
		assert!(30_000u32.abs_diff(nh.get("key")) < 3_000, "DIFF nh = {}", nh.get("key"));
	}
}