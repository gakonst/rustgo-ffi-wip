 #[derive(Debug, Clone)]
pub struct PublicKey {
    pub x: u64
}

#[derive(Debug)]
pub struct EpochBlock {
    pub index: u16,
    pub maximum_non_signers: u32,
    pub new_pubkeys: Vec<PublicKey>,
}

#[derive(Debug)]
#[repr(C)]
pub struct EpochBlockFFI {
    pub index: u16,
    pub maximum_non_signers: u32,
    pub new_pubkeys: *const PublicKey,
    pub pubkeys_len: usize,
}

use std::slice;

impl From<&EpochBlockFFI> for EpochBlock {
    fn from(src: &EpochBlockFFI) -> EpochBlock {
        dbg!(src.pubkeys_len);
        let added_public_keys = unsafe {
            slice::from_raw_parts(src.new_pubkeys, src.pubkeys_len as usize)
        };

        EpochBlock {
            index: src.index,
            maximum_non_signers: src.maximum_non_signers,
            new_pubkeys: added_public_keys.to_vec(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn verify(
    epoch: *const EpochBlockFFI,
) -> bool {
    let epoch = EpochBlock::from(&*epoch);
    dbg!(epoch);
    true
}