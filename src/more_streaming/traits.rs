use crate::{FlowId,NitroHash,SpaceSaving,NitroCMS,CuckooCountingFilter,NitroCuckoo};
use amadeus_streaming::CountMinSketch;
use dashmap::DashMap;
use crate::Hasher;

/// Increment an item's count
pub trait ItemIncrement {
	fn item_increment(&mut self,id: FlowId);
}
impl ItemIncrement for NitroHash<FlowId,u32> {
	fn item_increment(&mut self,id: FlowId) {
		self.insert(id);
	}
}
impl ItemIncrement for SpaceSaving<FlowId,u32> {
	fn item_increment(&mut self,id: FlowId) {
		self.insert(id);
	}
}
impl ItemIncrement for CountMinSketch<FlowId,u32> {
	fn item_increment(&mut self,id: FlowId) {
		self.push(&id,&1);
	}
}
impl ItemIncrement for NitroCMS<FlowId,u32> {
	fn item_increment(&mut self,id: FlowId) {
		self.push(&id,&1);
	}
}
impl ItemIncrement for DashMap<FlowId,u32> {
	fn item_increment(&mut self,id: FlowId) {
		if let Some(mut count) = self.get_mut(&id) {
			*count+=1;
		} else {
			self.insert(id,1);
		}
	}
}
impl <H>ItemIncrement for CuckooCountingFilter<H> 
where H:Hasher + Default,
{
	fn item_increment(&mut self,id: FlowId) {
		self.add(&id);
	}
}
impl <H>ItemIncrement for NitroCuckoo<H> 
where H:Hasher + Default,
{
	fn item_increment(&mut self,id: FlowId) {
		self.add(&id);
	}
}

pub trait ItemQuery {
	type Item;
	fn item_query(&self,id: FlowId) -> Self::Item;
}
impl ItemQuery for NitroHash<FlowId,u32> {
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(id)
	}
}
impl ItemQuery for SpaceSaving<FlowId,u32> {
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(id)
	}
}
impl ItemQuery for CountMinSketch<FlowId,u32> {
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(&id)
	}
}
impl ItemQuery for NitroCMS<FlowId,u32> {
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(&id)
	}
}
impl ItemQuery for DashMap<FlowId,u32> {
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return *self.get(&id).unwrap();
	}
}
impl <H>ItemQuery for CuckooCountingFilter<H> 
where H:Hasher + Default,
{
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(&id);
	}
}
impl <H>ItemQuery for NitroCuckoo<H> 
where H:Hasher + Default,
{
	type Item = u32;
	fn item_query(&self,id: FlowId) -> u32 {
		return self.get(&id);
	}
}


/// translate from a generic parameter to usize
pub trait VtoUsize {
	fn v_to_usize(&self) -> usize;
}

impl VtoUsize for u8 {
	fn v_to_usize(&self) -> usize {
		return usize::from(*self);
	}
}
impl VtoUsize for u16 {
	fn v_to_usize(&self) -> usize {
		return usize::from(*self);
	}
}
impl VtoUsize for u32 {
	fn v_to_usize(&self) -> usize {
		return usize::try_from(*self).unwrap();
	}
}
impl VtoUsize for u64 {
	fn v_to_usize(&self) -> usize {
		return usize::try_from(*self).unwrap();
	}
}
impl VtoUsize for usize {
	fn v_to_usize(&self) -> usize {
		return usize::from(*self);
	}
}

/// Union `Self` with `Rhs` in place.
pub trait UnionAssign<Rhs = Self> {
	/// Union.
	fn union_assign(&mut self, rhs: Rhs);
}
impl<'a> UnionAssign<&'a u8> for u8 {
	fn union_assign(&mut self, rhs: &'a Self) {
		*self = (*self).max(*rhs);
	}
}
impl<'a> UnionAssign<&'a u16> for u16 {
	fn union_assign(&mut self, rhs: &'a Self) {
		*self = (*self).max(*rhs);
	}
}
impl<'a> UnionAssign<&'a u32> for u32 {
	fn union_assign(&mut self, rhs: &'a Self) {
		*self = (*self).max(*rhs);
	}
}
impl<'a> UnionAssign<&'a u64> for u64 {
	fn union_assign(&mut self, rhs: &'a Self) {
		*self = (*self).max(*rhs);
	}
}
impl UnionAssign for usize {
	fn union_assign(&mut self, rhs: Self) {
		*self = (*self).max(rhs);
	}
}
impl<'a> UnionAssign<&'a usize> for usize {
	fn union_assign(&mut self, rhs: &'a Self) {
		*self = (*self).max(*rhs);
	}
}

/// Intersect zero or more `&Self` to create `Option<Self>`.
pub trait Intersect {
	/// Intersect.
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a;
}
impl Intersect for u8 {
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a,
	{
		iter.copied().min()
	}
}
impl Intersect for u16 {
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a,
	{
		iter.copied().min()
	}
}
impl Intersect for u32 {
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a,
	{
		iter.copied().min()
	}
}
impl Intersect for u64 {
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a,
	{
		iter.copied().min()
	}
}
impl Intersect for usize {
	fn intersect<'a>(iter: impl Iterator<Item = &'a Self>) -> Option<Self>
	where
		Self: Sized + 'a,
	{
		iter.copied().min()
	}
}

/// New instances are instantiable given a specified input of `<Self as New>::Config`.
pub trait New {
	/// The type of data required to instantiate a new `Self`.
	type Config: Clone;
	/// Instantiate a new `Self` with the given `<Self as New>::Config`.
	fn new(config: &Self::Config) -> Self;
}
impl New for u8 {
	type Config = ();
	fn new(_config: &Self::Config) -> Self {
		0
	}
}
impl New for u16 {
	type Config = ();
	fn new(_config: &Self::Config) -> Self {
		0
	}
}
impl New for u32 {
	type Config = ();
	fn new(_config: &Self::Config) -> Self {
		0
	}
}
impl New for u64 {
	type Config = ();
	fn new(_config: &Self::Config) -> Self {
		0
	}
}
impl New for usize {
	type Config = ();
	fn new(_config: &Self::Config) -> Self {
		0
	}
}

/// An optimisation for cases like putting a HyperLogLog inside a Count–min sketch, where intersecting, adding a val, and then unioning that with counters is the same as simply adding the val to the counters.
pub trait IntersectPlusUnionIsPlus {
	/// Apply optimisation or not
	const VAL: bool;
}

macro_rules! impl_ipuip {
	($($t:ty)*) => ($(
		impl IntersectPlusUnionIsPlus for $t {
			const VAL: bool = false;
		}
	)*)
}

impl_ipuip!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);