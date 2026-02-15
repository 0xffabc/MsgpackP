use crate::constants::Families;
use crate::msgpack::WriteTo;
use anyhow::Result;
use std::io::Write;

impl WriteTo for String {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let len = self.len();

        if len <= 31 {
            /*
             * Fixstr ranges from 0xa0 to 0xbf (https://github.com/msgpack/msgpack/blob/master/spec.md#:~:text=101xxxxx-,0xa0%20%2D%200xbf,-nil)
             *
             * The distance between is obviously 31, therefore we would never overflow
             * and write an incorrect byte.
             */

            writer.write_all(&[Families::FIXSTR + len as u8])?;
        } else if len <= 255 {
            /*
             * str 8 stores a byte array whose length is upto (2^8)-1 bytes:
             *
             * +--------+--------+========+
             * |  0xd9  |YYYYYYYY|  data  |
             * +--------+--------+========+
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#:~:text=8)%2D1%20bytes%3A%0A%2B%2D%2D%2D%2D%2D%2D%2D%2D%2B%2D%2D%2D%2D%2D%2D%2D%2D%2B%3D%3D%3D%3D%3D%3D%3D%3D%2B%0A%7C-,0xd9,-%7CYYYYYYYY%7C%20%20data%20%20%7C%0A%2B%2D%2D%2D%2D%2D%2D%2D%2D%2B%2D%2D%2D%2D%2D%2D%2D%2D%2B%3D%3D%3D%3D%3D%3D%3D%3D%2B%0A%0Astr
             */

            writer.write_all(&[Families::STR8, len as u8])?;
        } else if len <= 65535 {
            /*
             * str 16 stores a byte array whose length is upto (2^16)-1 bytes:
             * +--------+--------+--------+========+
             * |  0xda  |ZZZZZZZZ|ZZZZZZZZ|  data  |
             * +--------+--------+--------+========+
             */

            let bytes = (len as u16).to_be_bytes();

            writer.write_all(&[Families::STR16, bytes[0], bytes[1]])?;
        } else if len <= 2147483648 {
            /*
             * str 32 stores a byte array whose length is upto (2^32)-1 bytes:
             * +--------+--------+--------+--------+--------+========+
             * |  0xdb  |AAAAAAAA|AAAAAAAA|AAAAAAAA|AAAAAAAA|  data  |
             * +--------+--------+--------+--------+--------+========+
             */

            let bytes = (len as u32).to_be_bytes();

            writer.write_all(&[Families::STR32, bytes[0], bytes[1], bytes[2], bytes[3]])?;
        } else {
            /*
             * Msgpack format doesn't support 64 bit length strings.
             *
             * However, you can implement an extension for that.
             */

            return Err(anyhow::anyhow!(
                "Error: Overflow while writing a string. Expected maximum string length is 2^31."
            ));
        }

        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}
