//! # Binary
//!
//! A simple binary encoding and decoding library.

#![feature(maybe_uninit_array_assume_init)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Contains the Binary macro for deriving the Binary trait.
#[cfg(feature = "derive")]
pub mod derive {
    /// A derive macro which should work for most situations. Please file an issue if it isn't working for
    /// you explaining why.
    pub use binary_derive::Binary;
}

/// Types which can be serialized and deserialized into a binary format.
pub trait Binary: Sized {
    /// Deserialize self from bytes, potentially leaving more input.
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])>;
    /// Serialize self to the vector.
    fn unparse(&self, bs: &mut Vec<u8>);
    /// Encodes the given object.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bs = Vec::new();
        self.unparse(&mut bs);
        bs
    }
    /// Parses from bytes, only returning Some when the input is exactly the right length.
    fn from_bytes(bs: &[u8]) -> Option<Self> {
        let (x, bs) = Self::parse(bs)?;
        if bs.len() == 0 {
            Some(x)
        } else {
            None
        }
    }
}

/// Parse the given number of bytes into a fixed length array. This can be helpful for writing
/// implementations of Binary.
pub fn parse_bytes<const N: usize>(bs: &[u8]) -> Option<(&[u8; N], &[u8])> {
    if bs.len() >= N {
        Some((
            <&[u8; N] as TryFrom<&[u8]>>::try_from(&bs[0..N])
                .expect(&format!("all length {}+ bytestrings should parse here", N)),
            &bs[N..],
        ))
    } else {
        None
    }
}

impl<A: Binary, B: Binary> Binary for (A, B) {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (a, bs) = A::parse(bs)?;
        let (b, bs) = B::parse(bs)?;
        Some(((a, b), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.0.unparse(bs);
        self.1.unparse(bs);
    }
}

impl<A: Binary, B: Binary, C: Binary> Binary for (A, B, C) {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (a, bs) = A::parse(bs)?;
        let (b, bs) = B::parse(bs)?;
        let (c, bs) = C::parse(bs)?;
        Some(((a, b, c), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.0.unparse(bs);
        self.1.unparse(bs);
        self.2.unparse(bs);
    }
}

// TODO implement more tuples via a proc macro

impl Binary for () {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        Some(((), bs))
    }

    fn unparse(&self, _bs: &mut Vec<u8>) {}
}

impl<const LENGTH: usize, A: Binary> Binary for [A; LENGTH] {
    fn parse(mut bs: &[u8]) -> Option<(Self, &[u8])> {
        use std::mem::MaybeUninit;
        let mut marray: [MaybeUninit<A>; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..LENGTH {
            let (x, bs_prime) = A::parse(bs)?;
            marray[i] = MaybeUninit::new(x);
            bs = bs_prime;
        }

        let array = unsafe { MaybeUninit::array_assume_init::<LENGTH>(marray) };

        Some((array, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        for i in 0..LENGTH {
            self[i].unparse(bs);
        }
    }
}

impl<A: Binary> Binary for Vec<A> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut v = Vec::new();
        for _i in 0..n {
            let (a, bs_prime) = A::parse(bs)?;
            v.push(a);
            bs = bs_prime;
        }
        Some((v, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for a in self.iter() {
            a.unparse(bs);
        }
    }
}

impl Binary for i128 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (i128_bytes, bs) = parse_bytes::<16>(bs)?;
        Some((i128::from_le_bytes(i128_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for u128 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (u128_bytes, bs) = parse_bytes::<16>(bs)?;
        Some((u128::from_le_bytes(u128_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for u64 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (u64_bytes, bs) = parse_bytes::<8>(bs)?;
        Some((u64::from_le_bytes(u64_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for i64 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (i64_bytes, bs) = parse_bytes::<8>(bs)?;
        Some((i64::from_le_bytes(i64_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for u32 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (u32_bytes, bs) = parse_bytes::<4>(bs)?;
        Some((u32::from_le_bytes(u32_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for i32 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (i32_bytes, bs) = parse_bytes::<4>(bs)?;
        Some((i32::from_le_bytes(i32_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes())
    }
}

impl Binary for u16 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (u16_bytes, bs) = parse_bytes::<2>(bs)?;
        Some((u16::from_le_bytes(u16_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes());
    }
}

impl Binary for i16 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (i16_bytes, bs) = parse_bytes::<2>(bs)?;
        Some((i16::from_le_bytes(i16_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes());
    }
}

impl Binary for u8 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (u8_byte, bs) = parse_bytes::<1>(bs)?;
        Some((u8::from_le_bytes(u8_byte.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.push(*self);
    }
}
impl Binary for i8 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (i8_byte, bs) = parse_bytes::<1>(bs)?;
        Some((i8::from_le_bytes(i8_byte.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes());
    }
}

impl Binary for bool {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (b, bs) = u8::parse(bs)?;
        if b == 1 {
            Some((true, bs))
        } else if b == 0 {
            Some((false, bs))
        } else {
            None
        }
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.push(if *self { 1 } else { 0 });
    }
}

impl Binary for char {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, bs) = u32::parse(bs)?;
        Some((char::from_u32(n)?, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        u32::from(*self).unparse(bs);
    }
}

impl Binary for String {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (ss, bs) = <Vec<u8> as Binary>::parse(bs)?;
        match String::from_utf8(ss) {
            Err(_e) => None,
            Ok(s) => Some((s, bs)),
        }
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        bs.extend_from_slice(self.as_bytes());
    }
}

impl Binary for f32 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (f32_bytes, bs) = parse_bytes::<4>(bs)?;
        Some((f32::from_le_bytes(f32_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes());
    }
}

impl Binary for f64 {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (f64_bytes, bs) = parse_bytes::<8>(bs)?;
        Some((f64::from_le_bytes(f64_bytes.clone()), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_le_bytes());
    }
}

impl<Key: Binary + std::hash::Hash + Eq, Value: Binary> Binary for HashMap<Key, Value> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = HashMap::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            let (v, bs_prime) = Value::parse(bs_prime)?;
            m.insert(k, v);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for (k, v) in self {
            k.unparse(bs);
            v.unparse(bs);
        }
    }
}

impl<Key: Binary + Ord, Value: Binary> Binary for BTreeMap<Key, Value> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = BTreeMap::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            let (v, bs_prime) = Value::parse(bs_prime)?;
            m.insert(k, v);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for (k, v) in self {
            k.unparse(bs);
            v.unparse(bs);
        }
    }
}

impl<Key: Binary + std::hash::Hash + Eq> Binary for HashSet<Key> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = HashSet::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            m.insert(k);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for k in self {
            k.unparse(bs);
        }
    }
}

impl<Key: Binary + Ord> Binary for BTreeSet<Key> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = BTreeSet::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            m.insert(k);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for k in self {
            k.unparse(bs);
        }
    }
}

impl<Key: Binary + Ord> Binary for BinaryHeap<Key> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = BinaryHeap::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            m.push(k);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for k in self {
            k.unparse(bs);
        }
    }
}

impl<Key: Binary> Binary for VecDeque<Key> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = VecDeque::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            m.push_back(k);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for k in self {
            k.unparse(bs);
        }
    }
}

impl<Key: Binary> Binary for LinkedList<Key> {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (n, mut bs) = u64::parse(bs)?;
        let mut m = LinkedList::new();
        for _i in 0..n {
            let (k, bs_prime) = Key::parse(bs)?;
            m.push_back(k);
            bs = bs_prime;
        }
        Some((m, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        (self.len() as u64).unparse(bs);
        for k in self {
            k.unparse(bs);
        }
    }
}

#[cfg(feature = "bls12_381")]
use bls12_381::{G1Affine, G1Projective, G2Affine, G2Projective, Scalar};

#[cfg(feature = "bls12_381")]
impl Binary for Scalar {
    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_bytes());
    }

    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (scalar_bytes, bs) = parse_bytes::<32>(bs)?;
        let scalar = Option::from(Scalar::from_bytes(scalar_bytes))?;
        Some((scalar, bs))
    }
}

#[cfg(feature = "bls12_381")]
impl Binary for G1Affine {
    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_compressed());
    }

    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (g1affine_bytes, bs) = parse_bytes::<48>(bs)?;
        let g1affine = Option::from(G1Affine::from_compressed(g1affine_bytes))?;
        Some((g1affine, bs))
    }
}

#[cfg(feature = "bls12_381")]
impl Binary for G1Projective {
    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&G1Affine::from(self).to_compressed());
    }

    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (g1affine_bytes, bs) = parse_bytes::<48>(bs)?;
        let g1projective = Option::from(G1Affine::from_compressed(g1affine_bytes))
            .map(|x: G1Affine| G1Projective::from(x))?;
        Some((g1projective, bs))
    }
}

#[cfg(feature = "bls12_381")]
impl Binary for G2Affine {
    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&self.to_compressed());
    }

    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (g2affine_bytes, bs) = parse_bytes::<96>(bs)?;
        let g2affine = Option::from(G2Affine::from_compressed(g2affine_bytes))?;
        Some((g2affine, bs))
    }
}

#[cfg(feature = "bls12_381")]
impl Binary for G2Projective {
    fn unparse(&self, bs: &mut Vec<u8>) {
        bs.extend_from_slice(&G2Affine::from(self).to_compressed());
    }

    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (g2affine_bytes, bs) = parse_bytes::<96>(bs)?;
        let g2projective = Option::from(G2Affine::from_compressed(g2affine_bytes))
            .map(|x: G2Affine| G2Projective::from(x))?;
        Some((g2projective, bs))
    }
}

#[cfg(feature = "curve25519-dalek")]
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar as RistrettoScalar,
};

#[cfg(feature = "curve25519-dalek")]
impl Binary for CompressedRistretto {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (compressed_bytes, bs) = <[u8; 32] as Binary>::parse(bs)?;
        Some((CompressedRistretto::from_slice(&compressed_bytes).ok()?, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.as_bytes().unparse(bs);
    }
}

#[cfg(feature = "curve25519-dalek")]
impl Binary for RistrettoPoint {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (cr, bs) = CompressedRistretto::parse(bs)?;
        Some((CompressedRistretto::decompress(&cr)?, bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.compress().unparse(bs);
    }
}

#[cfg(feature = "curve25519-dalek")]
impl Binary for RistrettoScalar {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (scalar_bytes, bs) = <[u8; 32] as Binary>::parse(bs)?;
        Some((
            Option::from(RistrettoScalar::from_canonical_bytes(scalar_bytes))?,
            bs,
        ))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.as_bytes().unparse(bs);
    }
}

#[cfg(feature = "blake3")]
impl Binary for blake3::Hash {
    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
        let (hash_bytes, bs) = <[u8; 32] as Binary>::parse(bs)?;
        Some((blake3::Hash::from_bytes(hash_bytes), bs))
    }

    fn unparse(&self, bs: &mut Vec<u8>) {
        self.as_bytes().unparse(bs);
    }
}

#[cfg(test)]
mod test {
    use super::{derive, parse_bytes, Binary};

    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

    use rand::{
        distributions::{Alphanumeric, Distribution, Standard},
        thread_rng,
    };

    #[cfg(feature = "bls12_381")]
    use bls12_381::Scalar;

    fn test_serialization<T>(samples: usize)
    where
        Standard: Distribution<T>,
        T: Binary + PartialEq + std::fmt::Debug,
    {
        for _i in 0..samples {
            let x = rand::random::<T>();
            assert_eq!(x, T::from_bytes(&x.to_bytes()).unwrap());
        }
    }

    #[test]
    fn test_primitives_binary() {
        let samples = 10000;
        test_serialization::<u8>(samples);
        test_serialization::<u16>(samples);
        test_serialization::<u32>(samples);
        test_serialization::<u64>(samples);
        test_serialization::<u128>(samples);
        test_serialization::<i8>(samples);
        test_serialization::<i16>(samples);
        test_serialization::<i32>(samples);
        test_serialization::<i64>(samples);
        test_serialization::<i128>(samples);
        test_serialization::<f32>(samples);
        test_serialization::<f64>(samples);
        test_serialization::<bool>(samples);
        test_serialization::<char>(samples);
    }

    #[test]
    fn test_string_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let s: String = Alphanumeric
                .sample_iter(&mut rng)
                .take(length)
                .map(char::from)
                .collect();
            assert_eq!(s, String::from_bytes(&s.to_bytes()).unwrap());
        }
    }

    #[test]
    fn test_vec_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: Vec<u8> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(v, <Vec<u8> as Binary>::from_bytes(&v.to_bytes()).unwrap());
        }
    }

    #[test]
    fn test_heap_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: BinaryHeap<u8> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(
                v.iter().collect::<Vec<_>>(),
                <BinaryHeap<u8> as Binary>::from_bytes(&v.to_bytes())
                    .unwrap()
                    .iter()
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_btreeset_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: BTreeSet<i32> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(
                v,
                <BTreeSet<i32> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_hashset_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: HashSet<i32> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(
                v,
                <HashSet<i32> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_btreemap_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let keys: Vec<u64> = Standard.sample_iter(&mut rng).take(length).collect();
            let v: BTreeMap<u64, u128> = keys
                .iter()
                .copied()
                .zip(Standard.sample_iter(&mut rng))
                .take(length)
                .collect();
            assert_eq!(
                v,
                <BTreeMap<u64, u128> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_hashmap_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let keys: Vec<u64> = Standard.sample_iter(&mut rng).take(length).collect();
            let v: HashMap<u64, u128> = keys
                .iter()
                .copied()
                .zip(Standard.sample_iter(&mut rng))
                .take(length)
                .collect();
            assert_eq!(
                v,
                <HashMap<u64, u128> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_linkedlist_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: LinkedList<i32> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(
                v,
                <LinkedList<i32> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_vecdeque_binary() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let length: usize = Standard.sample(&mut rng);
            let length = length % 100;
            let v: VecDeque<i32> = Standard.sample_iter(&mut rng).take(length).collect();
            assert_eq!(
                v,
                <VecDeque<i32> as Binary>::from_bytes(&v.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_array() {
        use std::mem::MaybeUninit;
        let mut rng = thread_rng();
        let samples = 1000;
        for _i in 0..samples {
            let array = {
                let mut marray: [MaybeUninit<u64>; 1000] = [MaybeUninit::uninit(); 1000];
                for i in 0..1000 {
                    marray[i] = MaybeUninit::new(Standard.sample(&mut rng));
                }
                unsafe { MaybeUninit::array_assume_init(marray) }
            };
            assert_eq!(
                array,
                <[u64; 1000] as Binary>::from_bytes(&array.to_bytes()).unwrap()
            );
        }
    }

    #[test]
    fn test_tuple() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let t: (u128, char) = (Standard.sample(&mut rng), Standard.sample(&mut rng));
            assert_eq!(
                t,
                <(u128, char) as Binary>::from_bytes(&t.to_bytes()).unwrap()
            );
            let x: i128 = Standard.sample(&mut rng);
            let t = (t.0, t.1, x);
            assert_eq!(
                t,
                <(u128, char, i128) as Binary>::from_bytes(&t.to_bytes()).unwrap()
            );
        }
    }

    #[derive(derive::Binary, Debug, PartialEq)]
    struct Example {
        a: u128,
        b: i64,
        c: f32,
    }

    #[cfg(feature = "bls12_381")]
    #[derive(derive::Binary, Debug, PartialEq)]
    struct Nested {
        m: BTreeMap<u64, Scalar>,
    }

    #[cfg(feature = "bls12_381")]
    #[test]
    fn test_nested() {
        use ff::Field;
        let mut rng = thread_rng();
        let samples = 100;
        for _i in 0..samples {
            let mut m = BTreeMap::new();
            for _j in 0..100 {
                let n: u64 = Standard.sample(&mut rng);
                let n = n % 100;
                m.insert(n, Scalar::random(&mut rng));
            }
            let n = Nested { m };
            assert_eq!(n, Nested::from_bytes(&n.to_bytes()).unwrap());
        }
    }

    #[test]
    fn test_custom_named_struct() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let example = Example {
                a: Standard.sample(&mut rng),
                b: Standard.sample(&mut rng),
                c: Standard.sample(&mut rng),
            };
            assert_eq!(example, Example::from_bytes(&example.to_bytes()).unwrap());
        }
    }

    #[derive(derive::Binary, Debug, PartialEq)]
    struct Other(u128, i64);

    #[test]
    fn test_custom_unnamed_struct() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let other = Other(Standard.sample(&mut rng), Standard.sample(&mut rng));
            assert_eq!(other, Other::from_bytes(&other.to_bytes()).unwrap());
        }
    }

    #[derive(derive::Binary, Debug, PartialEq)]
    enum WhatsIt {
        GoesEr(u128, u64),
        Pozer { x: f32, y: f64, z: i32 },
        Whaner,
    }

    #[test]
    fn test_custom_enum() {
        let mut rng = thread_rng();
        let samples = 10000;
        for _i in 0..samples {
            let choice: u8 = Standard.sample(&mut rng);
            let choice = choice % 3;
            let whatsit = if choice == 0 {
                WhatsIt::GoesEr(Standard.sample(&mut rng), Standard.sample(&mut rng))
            } else if choice == 1 {
                WhatsIt::Pozer {
                    x: Standard.sample(&mut rng),
                    y: Standard.sample(&mut rng),
                    z: Standard.sample(&mut rng),
                }
            } else {
                WhatsIt::Whaner
            };
            assert_eq!(whatsit, WhatsIt::from_bytes(&whatsit.to_bytes()).unwrap());
        }
    }

    #[test]
    fn test_parse_bytes() {
        let bs = [1u8, 5, 3, 1, 2, 4, 5, 6];
        assert!(parse_bytes::<9>(&bs).is_none());
        assert!(parse_bytes::<8>(&bs).is_some());
        assert!(parse_bytes::<7>(&bs).is_some());
        assert!(parse_bytes::<6>(&bs).is_some());
        assert!(parse_bytes::<5>(&bs).is_some());
        assert!(parse_bytes::<4>(&bs).is_some());
        assert!(parse_bytes::<3>(&bs).is_some());
        assert!(parse_bytes::<2>(&bs).is_some());
        assert!(parse_bytes::<1>(&bs).is_some());
        assert!(parse_bytes::<0>(&bs).is_some());
    }

    #[cfg(feature = "bls12_381")]
    #[test]
    fn test_g1affine() {
        use bls12_381::{G1Affine, G1Projective};
        use group::Group;
        let samples = 1000;
        let mut rng = thread_rng();
        for _i in 0..samples {
            let g1: G1Affine = G1Projective::random(&mut rng).into();
            assert_eq!(g1, G1Affine::from_bytes(&g1.to_bytes()).unwrap());
        }
    }

    #[cfg(feature = "bls12_381")]
    #[test]
    fn test_g2affine() {
        use bls12_381::{G2Affine, G2Projective};
        use group::Group;
        let samples = 1000;
        let mut rng = thread_rng();
        for _i in 0..samples {
            let g1: G2Affine = G2Projective::random(&mut rng).into();
            assert_eq!(g1, G2Affine::from_bytes(&g1.to_bytes()).unwrap());
        }
    }

    #[cfg(feature = "bls12_381")]
    #[test]
    fn test_scalar() {
        use bls12_381::Scalar;
        use ff::Field;
        let samples = 10000;
        let mut rng = thread_rng();
        for _i in 0..samples {
            let g1: Scalar = Scalar::random(&mut rng);
            assert_eq!(g1, Scalar::from_bytes(&g1.to_bytes()).unwrap());
        }
    }

    #[cfg(feature = "curve25519-dalek")]
    #[test]
    fn test_ristretto() {
        use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
        let samples = 10000;
        let mut rng = thread_rng();
        for _i in 0..samples {
            let r = RistrettoPoint::random(&mut rng);
            assert_eq!(r, RistrettoPoint::from_bytes(&r.to_bytes()).unwrap());
            let cr = r.compress();
            assert_eq!(cr, CompressedRistretto::from_bytes(&cr.to_bytes()).unwrap());
        }
    }

    #[cfg(feature = "curve25519-dalek")]
    #[test]
    fn test_ristretto_scalar() {
        use curve25519_dalek::scalar::Scalar as RistrettoScalar;
        let samples = 10000;
        let mut rng = thread_rng();
        for _i in 0..samples {
            let r = RistrettoScalar::random(&mut rng);
            assert_eq!(r, RistrettoScalar::from_bytes(&r.to_bytes()).unwrap());
        }
    }

    #[cfg(feature = "blake3")]
    #[test]
    fn test_hash() {
        use rand::Fill;
        let samples = 10000;
        let mut rng = thread_rng();
        let mut bytes = [0u8; 100];
        for _i in 0..samples {
            bytes.try_fill(&mut rng).unwrap();
            let r = blake3::hash(&bytes);
            assert_eq!(
                r,
                <blake3::Hash as Binary>::from_bytes(&r.to_bytes()).unwrap()
            );
        }
    }
}
