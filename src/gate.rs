/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 * 
 */

pub fn nand(a: bool, b: bool) -> bool {
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

pub fn nor(
    a: bool,
    b: bool
) -> bool {
    not(or(a, b))
}

pub fn mux(
    a: bool,
    b: bool,
    sel: bool
) -> bool {
    if sel {
        b
    } else {
        a
    }
}

pub fn not16(
    a: &[bool;16]
) -> [bool;16] {
    let mut out: [bool; 16] = [false; 16];
    for index in 0..16 {
        out[index] = not(a[index]);
    }
    out
}

pub fn and16(
    a: &[bool;16],
    b: &[bool;16]
) -> [bool;16] {
    let mut out: [bool; 16] = [false; 16];
    for index in 0..16 {
        out[index] = and(a[index], b[index]);
    }
    out
}

pub fn or16(
    a: &[bool;16],
    b: &[bool;16]
) -> [bool;16] {
    let mut out: [bool; 16] = [false; 16];
    for index in 0..16 {
        out[index] = or(a[index], b[index]);
    }
    out
}

pub fn mux16(
    a: &[bool;16],
    b: &[bool;16],
    sel: bool
) -> [bool;16] {
    let mut out: [bool; 16] = [false; 16];
    if sel {
        out = *b;
    } else {
        out = *a;
    }
    out
}
