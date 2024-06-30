/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * 
 */

/* 
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn notx(
    a: bool,
    _b: bool
) -> bool {
    nand(a, a)
}

pub fn or(
    a: bool,
    b: bool
) -> bool {
    nand(notx(a, a), notx(b, b))
}

pub fn and(
    a: bool,
    b: bool
) -> bool {
    nand(nand(a, b), nand(a, b))
}

pub fn xor(
    a: bool,
    b: bool
) -> bool {
    or(and(a, notx(b, b)), and(notx(a, a), b))
}
 */