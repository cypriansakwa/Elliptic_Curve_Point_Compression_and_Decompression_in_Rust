use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use std::ops::Shl;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ECPoint {
    x: BigUint,
    y: BigUint,
    is_infinity: bool,
}

struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn is_on_curve(&self, point: &ECPoint) -> bool {
        if point.is_infinity {
            return true;
        }
        let lhs = (&point.y * &point.y) % &self.p;
        let rhs = ((&point.x * &point.x * &point.x) + &self.a * &point.x + &self.b) % &self.p;
        lhs == rhs
    }

    fn compress(&self, point: &ECPoint) -> Option<(BigUint, bool)> {
        if point.is_infinity {
            return None;
        }
        Some((point.x.clone(), point.y.is_odd()))
    }

    fn decompress(&self, x: BigUint, is_odd: bool) -> Option<ECPoint> {
        let rhs = ((&x * &x * &x) + &self.a * &x + &self.b) % &self.p;
        let y = modular_sqrt(&rhs, &self.p)?;
        let selected_y = if y.is_odd() == is_odd { y } else { (&self.p - y) % &self.p };

        Some(ECPoint {
            x,
            y: selected_y,
            is_infinity: false,
        })
    }
}

fn modular_sqrt(a: &BigUint, p: &BigUint) -> Option<BigUint> {
    if *p == BigUint::from(2u8) {
        return Some(a.clone());
    }

    if legendre_symbol(a, p) != 1 {
        return None;
    }

    let mut q = p - BigUint::one();
    let mut s = 0u32;

    while (&q & BigUint::one()).is_zero() {
        q /= 2u8;
        s += 1;
    }

    if s == 1 {
        let r = a.modpow(&((&*p + BigUint::one()) / 4u8), p);
        return Some(r);
    }

    let mut z = BigUint::one();
    while legendre_symbol(&z, p) != -1 {
        z += 1u8;
    }

    let mut m = BigUint::from(s);
    let mut c = z.modpow(&q, p);
    let mut t = a.modpow(&q, p);
    let mut r = a.modpow(&((&q + BigUint::one()) / 2u8), p);

    while t != BigUint::one() {
        let mut i = BigUint::zero();
        let mut t2i = t.clone();
        while t2i != BigUint::one() {
            t2i = (&t2i * &t2i) % p;
            i += 1u8;
        }

        let power = (&m - &i - BigUint::one()).to_u32().unwrap(); // Correct usage of `to_u32`
        let b = c.modpow(&BigUint::one().shl(power), p);
        m = i.clone();
        c = (&b * &b) % p;
        t = (&t * &c) % p;
        r = (&r * &b) % p;
    }

    Some(r)
}

fn legendre_symbol(a: &BigUint, p: &BigUint) -> i8 {
    let ls = a.modpow(&((p - BigUint::one()) / 2u8), p);
    if ls.is_zero() {
        0
    } else if ls == BigUint::one() {
        1
    } else {
        -1
    }
}

trait BigUintExt {
    fn is_odd(&self) -> bool;
}

impl BigUintExt for BigUint {
    fn is_odd(&self) -> bool {
        self % 2u8 == BigUint::one()
    }
}

fn main() {
    let a = BigUint::from(3u8);
    let b = BigUint::from(4u8);
    let p = BigUint::from(7u8);

    let curve = EllipticCurve { a, b, p };

    let point = ECPoint {
        x: BigUint::from(2u8),
        y: BigUint::from(5u8),
        is_infinity: false,
    };

    assert!(curve.is_on_curve(&point));

    let compressed = curve.compress(&point);
    println!("Compressed: {:?}", compressed);

    if let Some((x, is_odd)) = compressed {
        let decompressed = curve.decompress(x, is_odd);
        println!("Decompressed: {:?}", decompressed);
    }
}
