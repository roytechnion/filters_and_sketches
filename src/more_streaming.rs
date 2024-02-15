pub mod nitro_cms;
pub mod space_saving;
pub mod nitro_hash;
pub mod cuckoo;
pub mod nitro_cuckoo;
pub mod facs;
pub mod traits;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_lossless
)]
pub fn f64_to_usize(a: f64) -> usize {
    assert!(a.is_sign_positive() && a <= usize::max_value() as f64 && a.fract() == 0.0);
    a as usize
}
