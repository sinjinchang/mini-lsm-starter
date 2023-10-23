mod builder;
mod iterator;

pub use builder::BlockBuilder;
/// You may want to check `bytes::BufMut` out when manipulating continuous chunks of memory
use bytes::{Bytes,Buf,BufMut};
pub use iterator::BlockIterator;
pub const SIZEOF_16: usize = std::mem::size_of<u16>();
/// A block is the smallest unit of read and caching in LSM tree.
/// It is a collection of sorted key-value pairs.
/// The `actual` storage format is as below (After `Block::encode`):
///
/// ----------------------------------------------------------------------------------------------------
/// |             Data Section             |              Offset Section             |      Extra      |
/// ----------------------------------------------------------------------------------------------------
/// | Entry #1 | Entry #2 | ... | Entry #N | Offset #1 | Offset #2 | ... | Offset #N | num_of_elements |
/// ----------------------------------------------------------------------------------------------------
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the tutorial
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        let offsets_len = self.offset.len();
        for offset in &self.offsets{
            buf.put_u16(*offset);
        }
        buf.put_u16(offsets_len as u16);
        buf.into()
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        let entry_offsets_len = (&data[data.len() - SIZEOF_16..]).get_u16() as usize;
        let data_end = data.len() - SIZEOF_16 - entry_offsets_len * SIZEOF_16;
        let offsets_raw = &data[data_end..data.len() - SIZEOF_16];
        let offsets = offsets_raw
            .chunks(SIZEOF_16)
            .map(|mut x| x.get_u16())
            .collect();
        let data = data[0..data_end].to_vec();
        Self {data, offsets}
    }
}

#[cfg(test)]
mod tests;
