// We need a hashmap of size 1/epsilon CMAX and remember this to avoid recalc
// The hashmap stores the flowid and counter
// We need a capacity counter C
// For each arrival of x:
//   if x in HM, increment its counter
//   else if C < CMAX
//      insert x to HM with counter 1
//   else {
//      find min counter
//      if not RAP or (with probability 1/(min+1))
//         replace entry with x and counter=(min+1)
//   }
//
// In the first iteration, we will do this for a fixed type and finding minimum by scanning (iter.min)
// In the second iteration, we will extend to general id type and counter size
// In the third iteration, we will replace scan for minimum with a heap

use std::collections::HashMap;
use std::hash::Hash;
//use std::str::FromStr;
use super::f64_to_usize;
use rand::Rng;
use std::fmt::Debug;
use increment::*;
//use crate::FlowId;
//use super::traits::VtoUsize;

/// An implementation of the space saving algorithm of Metwally, Agrawal, and El Abbadi w/out the
/// RAP optimization of Ben Basat, Chen, Einziger, Friedman, and Kassner

#[derive(Debug)]
pub struct SpaceSaving<K, V> {
    counters: HashMap<K, V>,
    capacity: usize,
    num: usize,
    rap: bool,
}

impl <K, V>SpaceSaving<K,V> 
where
K: Clone + Hash + std::cmp::Eq,
V: std::cmp::Ord + std::ops::Add<Output=V> + std::ops::AddAssign + TryFrom<u8> + Copy + super::traits::VtoUsize + std::fmt::Debug + increment::Incrementable
{
    pub fn new(error: f64, rap: bool) -> Self {
        let capacity = f64_to_usize((1.0/error).round());
        let counters = HashMap::with_capacity(capacity);
        let num = 0;
        Self {
            counters,
            capacity,
            num,
            rap
        }
    }

    pub fn insert(&mut self, id: K) 
    where <V as TryFrom<u8>>::Error: Debug
    {
        if let Some(counter) = self.counters.get_mut(&id) {
            *counter = increment!(*counter).unwrap();
        } else {
            if self.num < self.capacity {
                self.counters.insert(id, V::try_from(1_u8).unwrap());
                self.num += 1;
            } else {
                let (minkey,minval) = self.find_minimum();
                let added = increment!(minval).unwrap();
                if !self.rap || self.coin_flip(added.v_to_usize()) {
                    self.counters.remove(&minkey);
                    self.counters.insert(id,added);
                }
            }
        }
    }

    pub fn get(&self, id: K) -> V {
        if let Some(val) = self.counters.get(&id) {
            return *val;
        }
        let (_,val) = self.find_minimum();
        return val;

    }

    fn find_minimum(&self) -> (K,V) {
        let (id,val) = self.counters.iter()
           .min_by_key(|(_,b)| *b).unwrap();
        return (id.clone(),*val)
    }

    fn coin_flip(&self, prob: usize) -> bool {
        let mut rng = rand::thread_rng();
        return rng.gen_range(0..prob) == 0;
    }
}