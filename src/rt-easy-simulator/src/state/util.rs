use crate::Error;
use rtcore::program::BitRange;
use std::ops::Range;

pub fn slice_idx(range: BitRange, idx: BitRange) -> Result<Range<usize>, Error> {
    if !range.contains_range(idx) {
        return Err(Error::Other);
    }

    let (self_msb, self_lsb) = range.msb_lsb();
    let (idx_msb, idx_lsb) = idx.msb_lsb();

    let slice_idx = if self_msb >= self_lsb {
        let start = idx_lsb - self_lsb;
        let end = idx_msb - self_lsb + 1;
        start..end
    } else {
        let start = self_lsb - idx_lsb;
        let end = self_lsb - idx_msb + 1;
        start..end
    };

    Ok(slice_idx)
}
