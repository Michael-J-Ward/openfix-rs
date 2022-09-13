use std::collections::HashMap;

pub type FixFieldItems<'a> = HashMap<u32, &'a [u8]>;

const SEP_CHAR: u8 = 0x01;

pub fn split_message_items<'a>(data: &'a [u8]) -> FixFieldItems<'a> {
    data.split(|x| *x == SEP_CHAR)
        .filter_map(|field| {
            let mut fields = field.splitn(2, |x| *x == '=' as u8);

            let field_id = fields.next()?;
            let field_id = std::str::from_utf8(field_id).ok()?;
            let field_id = field_id.parse::<u32>().ok()?;

            let field_data = fields.next()?;

            Some((field_id, field_data))
        })
        .collect()
}
