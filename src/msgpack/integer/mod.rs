use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl WriteTo for u8 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes a u8 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match self {
            /*
             * Fixint is 0x00 - 0x7f according to the spec
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#map-format-family:~:text=0xxxxxxx-,0x00%20%2D%200x7f,-fixmap
             */
            0..=0x7f => writer.write_all(&[*self])?,
            _ => writer.write_all(&[Families::UINT8, *self])?,
        }

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for u8 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a u8 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, _reader: &mut Reader<T>) -> Result<Self> {
        Ok(packet_type)
    }
}

impl WriteTo for u16 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes a u16 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * uint 16 stores a 16-bit big-endian unsigned integer
         * +--------+--------+--------+
         * |  0xcd  |ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+
         */

        writer.write_all(&[Families::UINT16, bytes[0], bytes[1]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for u16 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a u16 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let byte = reader.pull(2)?;

        Ok(u16::from_be_bytes([byte[0], byte[1]]))
    }
}

impl WriteTo for u32 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes a u32 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * uint 32 stores a 32-bit big-endian unsigned integer
         * +--------+--------+--------+--------+--------+
         * |  0xce  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+
         */

        writer.write_all(&[Families::UINT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for u32 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a u32 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let byte = reader.pull(4)?;

        Ok(u32::from_be_bytes([byte[0], byte[1], byte[2], byte[3]]))
    }
}

impl WriteTo for u64 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes a u64 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * uint 64 stores a 64-bit big-endian unsigned integer
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         * |  0xcf  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         */

        writer.write_all(&[
            Families::UINT64,
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
        ])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for u64 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a u64 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let byte = reader.pull(8)?;

        Ok(u64::from_be_bytes([
            byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7],
        ]))
    }
}

impl WriteTo for i8 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes an i8 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match self {
            -32..=-1 => {
                let positive: u8 = u8::try_from(-self)?;

                /*
                 * Negative fixint
                 */

                writer.write_all(&[0xe0 + positive])?;
            }

            /*
             * int 8 stores a 8-bit signed integer
             * +--------+--------+
             * |  0xd0  |ZZZZZZZZ|
             * +--------+--------+
             */
            _ => writer.write_all(&[Families::INT8, *self as u8])?,
        }

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for i8 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads an i8 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        if packet_type >= 0xe0 {
            /*
             * Read a negative fixint
             */

            return Ok(-((!packet_type as i8).wrapping_add(1)));
        }

        /*
         * int 8 stores a 8-bit signed integer
         * +--------+--------+
         * |  0xd0  |ZZZZZZZZ|
         * +--------+--------+
         */

        let byte = reader.pull(1)?;

        Ok(i8::from_be_bytes([byte[0]]))
    }
}

impl WriteTo for i16 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes an i16 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * int 16 stores a 16-bit big-endian signed integer
         * +--------+--------+--------+
         * |  0xd1  |ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+
         */

        writer.write_all(&[Families::INT16, bytes[0], bytes[1]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for i16 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads an i16 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(2)?;

        /*
         * int 16 stores a 16-bit big-endian signed integer
         * +--------+--------+--------+
         * |  0xd1  |ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+
         */

        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }
}

impl WriteTo for i32 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes an i32 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * int 32 stores a 32-bit big-endian signed integer
         * +--------+--------+--------+--------+--------+
         * |  0xd2  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+
         */

        writer.write_all(&[Families::INT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for i32 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads an i32 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(4)?;

        /*
         * int 32 stores a 32-bit big-endian signed integer
         * +--------+--------+--------+--------+--------+
         * |  0xd2  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+
         */

        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl WriteTo for i64 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Writes an i64 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * int 64 stores a 64-bit big-endian signed integer
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         * |  0xd3  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         */

        writer.write_all(&[
            Families::INT64,
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
        ])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for i64 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads an i64 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(8)?;

        /*
         * int 64 stores a 64-bit big-endian signed integer
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         * |  0xd3  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         */

        Ok(i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}
