use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl WriteTo for f32 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Reads a f32 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * float 32 stores a floating point number in IEEE 754 single precision floating point number format:
         * +--------+--------+--------+--------+--------+
         * |  0xca  |XXXXXXXX|XXXXXXXX|XXXXXXXX|XXXXXXXX|
         * +--------+--------+--------+--------+--------+
         */

        writer.write_all(&[Families::FLOAT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for f32 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a f32 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(4)?;

        /*
         * float 32 stores a floating point number in IEEE 754 single precision floating point number format:
         * +--------+--------+--------+--------+--------+
         * |  0xca  |XXXXXXXX|XXXXXXXX|XXXXXXXX|XXXXXXXX|
         * +--------+--------+--------+--------+--------+
         */

        Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl WriteTo for f64 {
    #[inline(always)]
    /*
     * @name write_to
     * @description
     *
     * Reads a f64 to the underlying buffer
     */
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();

        /*
         * float 64 stores a floating point number in IEEE 754 double precision floating point number format:
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         * |  0xcb  |YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         */

        writer.write_all(&[
            Families::FLOAT64,
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

impl<'a> ReadFrom<'a> for f64 {
    #[inline(always)]
    /*
     * @name read_from
     * @description
     *
     * Reads a f64 from the underlying buffer
     */
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(8)?;

        /*
         * float 64 stores a floating point number in IEEE 754 double precision floating point number format:
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         * |  0xcb  |YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|YYYYYYYY|
         * +--------+--------+--------+--------+--------+--------+--------+--------+--------+
         */

        Ok(f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}
