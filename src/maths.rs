
use num_bigint::BigInt;
use num_traits::{One, FromPrimitive};
use num::{Integer, NumCast, ToPrimitive};

pub fn factorial<N: Integer + ToPrimitive>(n: N) -> BigInt {
    let mut f: BigInt = One::one();
    let end: usize = NumCast::from(n).expect("Number too big or negative number used.");
    for i in 1..(end + 1) {
        let bu: BigInt = FromPrimitive::from_usize(i).unwrap();
        f = f * bu;
    }
    f
}