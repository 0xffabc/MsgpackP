use std::mem::MaybeUninit;

use anyhow::Result;

/**
 * @name try_new_uninit_slice
 * @description
 *
 * MaybeUninit implementation of nightly-only try_new_uninit_slice
 */
pub fn try_new_uninit_slice<T>(len: usize) -> Result<Box<[MaybeUninit<T>]>> {
    let mut vec = Vec::new();

    vec.try_reserve(len)?;

    unsafe {
        vec.set_len(len);

        /*
         * Safety: We've just made a Vec, so it's aligned
         */

        let ptr = vec.as_mut_ptr() as *mut MaybeUninit<T>;

        let slice = std::slice::from_raw_parts_mut(ptr, len);

        Ok(Box::from_raw(slice as *mut [MaybeUninit<T>]))
    }
}
