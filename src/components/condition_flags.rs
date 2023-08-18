

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ConditionFlags{
    FL_POS = 1 << 0, /* P */
    FL_ZRO = 1 << 1, /* Z */
    FL_NEG = 1 << 2, /* N */
}

impl From<ConditionFlags> for  i16{
    fn from(value: ConditionFlags) -> Self {
       match value {
           ConditionFlags::FL_POS => 1,
           ConditionFlags::FL_ZRO => 0,
           ConditionFlags::FL_NEG => -1,
       }
    }
}