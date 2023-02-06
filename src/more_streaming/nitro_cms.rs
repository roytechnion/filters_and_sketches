// This file modifies the source code from https://github.com/jedisct1/rust-count-min-sketch/blob/088274e22a3decc986dec928c92cc90a709a0274/src/lib.rs
// as well as from https://github.com/constellation-rs/amadeus/blob/master/amadeus-streaming/src/count_min.rs under the following MIT License:
// The modifications implement the NitroSketch optimization as proposed in https://dl.acm.org/doi/10.1145/3341302.3342076

// Copyright (c) 2022 Roy Friedman - the NitroSketch modifications
// Copyright (c) 2016 Frank Denis

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{
	borrow::Borrow, cmp::max, convert::TryFrom, fmt, hash::{Hash, Hasher}, marker::PhantomData, ops
};
use twox_hash::XxHash;
use super::f64_to_usize;
use super::traits::{Intersect, IntersectPlusUnionIsPlus, New, UnionAssign};
use rand;
use rand_distr::{Geometric, Distribution};
use core::fmt::Debug;

/// An implementation of the NitroSketch optimization as reported in https://dl.acm.org/doi/10.1145/3341302.3342076
/// of a [count-min sketch](https://en.wikipedia.org/wiki/Count–min_sketch) data structure.
pub struct NitroCMS<K: ?Sized, C: New> {
	counters: Vec<Vec<C>>,
	offsets: Vec<usize>, // to avoid malloc/free each push
	mask: usize,
	k_num: usize,
    default: C,
    geo: Geometric,
	sample_prob: f64,
    factor: usize,
    curr_counter: usize,
    next_counter: usize,
    last_index: usize,
	config: <C as New>::Config,
	marker: PhantomData<fn(K)>,
}

impl<K: ?Sized, C> NitroCMS<K, C>
where
	K: Hash,
	C: New + for<'a> UnionAssign<&'a C> + Intersect + Clone + std::convert::TryFrom<usize> + std::ops::Mul<Output = C>,
{
	/// Create an empty `NitroCMS` data structure with the specified error tolerance.
	pub fn new(probability: f64, tolerance: f64, sample_prob: f64, config: C::Config) -> Self {
		let width = Self::optimal_width(tolerance);
		let k_num = Self::optimal_k_num(probability);
		let counters: Vec<Vec<C>> = (0..k_num)
			.map(|_| (0..width).map(|_| C::new(&config)).collect())
			.collect();
        let default = counters[0][0].clone();
		let offsets = vec![0; k_num];
        let geo = Geometric::new(sample_prob).unwrap();
        let factor = f64_to_usize((1.0/sample_prob).round());
        let curr_counter = 0;
        let last_index = 0;
        let next_counter = Self::calc_skip(geo,curr_counter);
		Self {
			counters,
			offsets,
			mask: Self::mask(width),
			k_num,
            default,
            geo,
			sample_prob,
            factor,
            curr_counter,
            next_counter,
            last_index,
			config,
			marker: PhantomData,
		}
	}

    /// "Visit" an element.
	pub fn push<Q: ?Sized, V: ?Sized>(&mut self, key: &Q, value: &V) -> C
	where
		Q: Hash,
		K: Borrow<Q>,
		C: for<'a> ops::AddAssign<&'a V> + IntersectPlusUnionIsPlus,
	{
        if self.sample_prob < 1.0 {
            self.sampled_push(key, value)
        } else {
            self.full_push(key, value)
        }
	}

    /// "Visit" an element - sampled version - only update sampled cpunters
	fn sampled_push<Q: ?Sized, V: ?Sized>(&mut self, key: &Q, value: &V) -> C
	where
		Q: Hash,
		K: Borrow<Q>,
		C: for<'a> ops::AddAssign<&'a V> + IntersectPlusUnionIsPlus,
	{
        if self.next_counter - self.curr_counter > self.k_num {
            self.curr_counter += self.k_num;
        } else if self.next_counter - self.curr_counter + self.last_index >= self.k_num {
            self.curr_counter += self.k_num - self.last_index;
			self.last_index = 0;
        } else { 
            loop {
                self.curr_counter = self.next_counter;
                self.last_index = self.curr_counter % self.k_num;
                let offset = usize::try_from(self.single_offset(key,self.last_index)).unwrap();
                self.counters[self.last_index][offset] += value;
                self.next_counter = Self::calc_skip(self.geo,self.curr_counter);
                if self.next_counter - self.curr_counter + self.last_index >= self.k_num {
                    break;
                }
            }
        }
        self.default.clone()
    }

	/// "Visit" an element - full version - taken from the original CMS implementation
	fn full_push<Q: ?Sized, V: ?Sized>(&mut self, key: &Q, value: &V) -> C
	where
		Q: Hash,
		K: Borrow<Q>,
		C: for<'a> ops::AddAssign<&'a V> + IntersectPlusUnionIsPlus,
	{
		let offsets = self.offsets(key);
		if !<C as IntersectPlusUnionIsPlus>::VAL {
			self.offsets
				.iter_mut()
				.zip(offsets)
				.for_each(|(offset, offset_new)| {
					*offset = offset_new;
				});
			let mut lowest = C::intersect(
				self.offsets
					.iter()
					.enumerate()
					.map(|(k_i, &offset)| &self.counters[k_i][offset]),
			)
			.unwrap();
			lowest += value;
			self.counters
				.iter_mut()
				.zip(self.offsets.iter())
				.for_each(|(counters, &offset)| {
					counters[offset].union_assign(&lowest);
				});
			lowest
		} else {
			C::intersect(
				self.counters
					.iter_mut()
					.zip(offsets)
					.map(|(counters, offset)| {
						counters[offset] += value;
						&counters[offset]
					}),
			)
			.unwrap()
		}
	}

	/// Union the aggregated value for `key` with `value`.
	pub fn union_assign<Q: ?Sized>(&mut self, key: &Q, value: &C)
	where
		Q: Hash,
		K: Borrow<Q>,
	{
		let offsets = self.offsets(key);
		self.counters
			.iter_mut()
			.zip(offsets)
			.for_each(|(counters, offset)| {
				counters[offset].union_assign(value);
			})
	}

	/// Retrieve an estimate of the aggregated value for `key`.
	pub fn get<Q: ?Sized>(&self, key: &Q) -> C // TODO: multiply by sample probability
	where
		Q: Hash,
		K: Borrow<Q>,
        <C as TryFrom<usize>>::Error:Debug
	{
		C::intersect(
			self.counters
				.iter()
				.zip(self.offsets(key))
				.map(|(counters, offset)| &counters[offset]),
		)
		.unwrap() * C::try_from(self.factor).unwrap()
	}

	pub fn estimate_memory_size(&self) -> usize {
	 	return self.counters.len() * std::mem::size_of::<C>() * self.k_num;
	}

	/// Clears the `NitroCMS` data structure, as if it was new.
	pub fn clear(&mut self) {
		let config = &self.config;
		self.counters
			.iter_mut()
			.flat_map(|x| x.iter_mut())
			.for_each(|counter| {
				*counter = C::new(config);
			})
	}

	fn optimal_width(tolerance: f64) -> usize {
		let e = tolerance;
		let width = f64_to_usize((2.0 / e).round());
		max(2, width)
			.checked_next_power_of_two()
			.expect("Width would be way too large")
	}

	fn mask(width: usize) -> usize {
		assert!(width > 1);
		assert_eq!(width & (width - 1), 0);
		width - 1
	}

	fn optimal_k_num(probability: f64) -> usize {
		max(
			1,
			f64_to_usize(((1.0 - probability).ln() / 0.5_f64.ln()).floor()),
		)
	}

	fn offsets<Q: ?Sized>(&self, key: &Q) -> impl Iterator<Item = usize>
	where
		Q: Hash,
		K: Borrow<Q>,
	{
		let mask = self.mask;
		hashes(key).map(move |hash| usize::try_from(hash & u64::try_from(mask).unwrap()).unwrap())
	}

    fn calc_skip(geo: Geometric, current_counter: usize) -> usize {
        let v = geo.sample(&mut rand::thread_rng()) as usize;
        return current_counter + v
    }

    fn single_offset<Q: ?Sized + Hash>(&self, key: &Q, index: usize) -> u64 {
        let mut hasher = XxHash::default();
        key.hash(&mut hasher);
        for _ in 0..index {
            hasher.write(&[123]);
        }
        let offest = hasher.finish();
        return offest & u64::try_from(self.mask).unwrap()
    }
}

fn hashes<Q: ?Sized>(key: &Q) -> impl Iterator<Item = u64>
where
	Q: Hash,
{
	#[allow(missing_copy_implementations, missing_debug_implementations)]
	struct X(XxHash);
	impl Iterator for X {
		type Item = u64;
		fn next(&mut self) -> Option<Self::Item> {
			let ret = self.0.finish();
			self.0.write(&[123]);
			Some(ret)
		}
	}
	let mut hasher = XxHash::default();
	key.hash(&mut hasher);
	X(hasher)
}

impl<K: ?Sized, C: New + Clone> Clone for NitroCMS<K, C> {
	fn clone(&self) -> Self {
		Self {
			counters: self.counters.clone(),
			offsets: vec![0; self.offsets.len()],
			mask: self.mask,
			k_num: self.k_num,
            default: self.default.clone(),
            geo: self.geo,
			sample_prob: self.sample_prob,
            factor: self.factor,
            curr_counter: self.curr_counter,
            next_counter: self.next_counter,
            last_index: self.last_index,
			config: self.config.clone(),
			marker: PhantomData,
		}
	}
}
impl<K: ?Sized, C: New> fmt::Debug for NitroCMS<K, C> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("NitroCMS")
			// .field("counters", &self.counters)
			.finish()
	}
}

#[cfg(test)]
mod tests {
	type NitroCMS8<K> = super::NitroCMS<K, u8>;
	type NitroCMS32<K> = super::NitroCMS<K, u32>;
	type NitroCMS64<K> = super::NitroCMS<K, u64>;

	#[ignore] // release mode stops panic
	#[test]
	#[should_panic]
	fn test_overflow() {
		let mut cms = NitroCMS8::<&str>::new(0.95, 10.0 / 100.0, 0.1, ());
		for _ in 0..300 {
			let _ = cms.push("key", &1);
		}
		// assert_eq!(cms.get("key"), &u8::max_value());
	}

	#[test]
	fn test_increment() {
		let mut cms = NitroCMS32::<&str>::new(0.95, 2.0 / 100.0, 0.1, ());
		for _ in 0..300_000 {
			let _ = cms.push("key", &1);
		}
		//assert_eq!(cms.get("key"), 3000);
		assert!(300_000u32.abs_diff(cms.get("key")) < 30_000, "key = {}", cms.get("key"));
	}

	#[test]
	#[cfg_attr(miri, ignore)]
	fn test_increment_multi() {
		let mut cms = NitroCMS64::<u64>::new(0.99, 2.0 / 100.0, 0.1, ());
		for i in 0..10_000_000 {
			let _ = cms.push(&(i % 100), &1);
		}
		for key in 0..100 {
			assert!(cms.get(&key) >= 90_000, "ACTUAL({}) = {}", key, cms.get(&key));
		}
		// cms.reset();
		// for key in 0..100 {
		//     assert!(cms.get(&key) < 11_000);
		// }
	}
}