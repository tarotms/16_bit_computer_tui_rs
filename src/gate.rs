/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * 
 */

use crate::utils;
use std::sync::atomic::Ordering;

pub fn nand(a: bool, b: bool) -> bool {
    utils::NAND_CALL_COUNT.fetch_add(1, Ordering::Relaxed);
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
