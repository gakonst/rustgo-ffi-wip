// use std::io::Read;
// use std::slice;
// use algebra::{serialize::CanonicalDeserialize, Bls12_377};
// use std::fmt::Display;

// type Proof = groth16::Proof<Bls12_377>;
// type VerifyingKey = groth16::VerifyingKey<Bls12_377>;

/// cbindgen:field-names=[x]
/// cbindgen:derive-eq
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

// Deserializes the VK / Proof and verifies them against 
#[no_mangle]
pub unsafe extern "C" fn verify(
    first_epoch: *const EpochBlockFFI,
) -> bool {
    let first_epoch = EpochBlock::from(&*first_epoch);
    dbg!(first_epoch);
    true
}

// fn verify_proof(vk: &VerifyingKey, proof: &Proof, first_epoch: &EpochBlock, last_epoch: &EpochBlock) -> Result<(), ()> {
//     Ok(())
// }

// fn read_slice<C: CanonicalDeserialize>(ptr: *const u8, len: u32) -> Result<C, ()> {
//     let data: Vec<u8> = unsafe { slice::from_raw_parts(ptr, len as usize).to_vec() };
//     let ret = C::deserialize(&mut &data[..]).unwrap();
//     Ok(ret)
// }



// #[cfg(test)]
// mod tests {

//     // generate a proof for a c

// }