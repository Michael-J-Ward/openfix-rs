use std::io::{self, Write};
use std::num::Wrapping;

const NUM_BEGIN_STRING: &str = "8";
const NUM_BODY_LENGTH: &str = "9";
const NUM_CHECK_SUM: &str = "10";

/// This class help to add few standard fields to a already
/// generated message. Including:
/// - begin string (default: FIX.4.4)
/// - length
/// - check sum
///
/// Bug may remains when payload is empty or do not end with '\x01'.
#[derive(Debug)]
pub struct FixEnvelopeBuilder {
    begin_string: String,
}

impl FixEnvelopeBuilder {
    pub fn new() -> Self {
        Self {
            begin_string: "FIX.4.4".to_string(),
        }
    }

    pub fn begin_string(mut self, value: &str) -> Self {
        self.begin_string = value.into();
        self
    }

    pub fn build_message<W>(&self, writer: &mut W, data: &[u8]) -> io::Result<()>
    where
        W: Write,
    {
        let header = format!(
            "{}={}\x01{}={}\x01",
            NUM_BEGIN_STRING,
            self.begin_string,
            NUM_BODY_LENGTH,
            data.len(),
        );

        macro_rules! bytes_sum {
            ($x:expr) => {
                $x.iter().map(|x| Wrapping(*x)).sum::<Wrapping<u8>>()
            };
        }

        let check_sum = bytes_sum!(header.as_bytes()) + bytes_sum!(data);

        writer.write_all(header.as_bytes())?;
        writer.write_all(data)?;
        write!(writer, "{}={:03}\x01", NUM_CHECK_SUM, check_sum)?;

        Ok(())
    }
}
