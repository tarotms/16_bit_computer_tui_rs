/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 * 
 */

 /* 
use std::sync::atomic::Ordering;
use crate::atomic;

pub fn nand(a: bool, b: bool) -> bool {
    atomic::NAND_CALL_COUNTS.fetch_add(1, Ordering::SeqCst);
    !(a && b)
}

pub fn not(
    a: bool,
) -> bool {
    nand(a, a)
}

pub fn or(
    a: bool,
    b: bool
) -> bool {
    nand(not(a), not(b))
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
    or(and(a, not(b)), and(not(a), b))
}
 */