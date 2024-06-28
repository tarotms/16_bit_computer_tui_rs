/* 
 *  _ _  _   _ _
 * (/\///_/// //
 * Created by Onigirazu Nori
 * www.youtube.com/@OnigirazuNori
 * 
 */

pub fn nand(x: bool, y: bool) -> bool {
    !(x && y)
}

pub fn notx(x: bool, _y: bool) -> bool {
    nand(x, x)
}

pub fn noty(_x: bool, y: bool) -> bool {
    nand(y, y)
}

pub fn or(x: bool, y: bool) -> bool {
    nand(notx(x, y), noty(x, y))
}

pub fn and(x: bool, y: bool) -> bool {
    nand(nand(x, y), nand(x, y))
}

pub fn xor(x: bool, y: bool) -> bool {
    or(and(x, noty(x, y)), and(notx(x, y), y)) 
}

