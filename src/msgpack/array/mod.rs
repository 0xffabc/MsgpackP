use std::io::Write;

use anyhow::Result;

use crate::{
    msgpack::{ReadFrom, WriteTo},
    reader::Reader,
    value::Value,
};

pub struct Array();

impl Array {
    pub const ARRAY_16_TYPE: u8 = 0xdc;
    pub const ARRAY_32_TYPE: u8 = 0xdd;
    pub const FIXARRAY_TYPE: u8 = 0x90;
}

impl WriteTo for Box<[Value<'_>]> {
    #[inline(always)]
    /**
     * @name write_to
     * @description
     *
     * Implements an Array writer
     */
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let array_length = self.len();

        match array_length {
            /*
             * Fixarr size is 15
             */
            0..=15 => buffer.write_all(&[Array::FIXARRAY_TYPE + array_length as u8])?,

            /*
             * Arr16 size is u16-1
             */
            16..=65534 => {
                let length = array_length.to_be_bytes();

                buffer.write_all(&[Array::ARRAY_16_TYPE, length[0], length[1]])?;
            }

            /*
             * Arr32 size is u32-1
             */
            65535..4294967295 => {
                let length = array_length.to_be_bytes();

                buffer.write_all(&[
                    Array::ARRAY_32_TYPE,
                    length[0],
                    length[1],
                    length[2],
                    length[3],
                ])?;
            }

            _ => return Err(anyhow::anyhow!("Arr64 is not supported by msgpack")),
        }

        for value in self {
            value.write_to(buffer)?;
        }

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for Box<[Value<'a>]> {
    #[inline(always)]
    fn read_from<U: AsRef<[u8]> + 'a>(array_type: u8, reader: &'a mut Reader<U>) -> Result<Self> {
        let array_length = match array_type {
            /*
             * Fixarr ranges from 0x90 to 0x9f
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#map-format-family:~:text=1001xxxx-,0x90%20%2D%200x9f,-fixstr
             */
            0x90..=0x9f => (array_type - 0x90) as usize,

            /*
             * Arr16
             */
            Array::ARRAY_16_TYPE => {
                let bytes = reader.pull(2)?;

                /*
                 * array 16 stores an array whose length is upto (2^16)-1 elements:
                 * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
                 * |  0xdc  |YYYYYYYY|YYYYYYYY|    N objects    |
                 * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
                 */

                u16::from_be_bytes([bytes[0], bytes[1]]) as usize
            }

            /*
             * Arr32
             */
            Array::ARRAY_32_TYPE => {
                let bytes = reader.pull(4)?;

                /*
                 * array 32 stores an array whose length is upto (2^32)-1 elements:
                 * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
                 * |  0xdd  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|    N objects    |
                 * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
                 */

                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
            }
            _ => return Ok(vec![].into_boxed_slice()),
        };

        /*
         * Note: DO NOT USE with_capacity!
         *
         * Preemptive allocations slow down everything **4 times**
         */

        let mut uninit = Box::<[Value]>::new_uninit_slice(array_length);

        unsafe {
            let reader_ptr = reader as *mut Reader<U>;

            for i in 0..array_length {
                uninit[i].write((&mut *reader_ptr).pull_value()?);
            }

            let values = uninit.assume_init();

            Ok(values)
        }
    }
}
