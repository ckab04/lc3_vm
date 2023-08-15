

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ConditionFlags{
    FL_POS = 1 << 0, /* P */
    FL_ZRO = 1 << 1, /* Z */
    FL_NEG = 1 << 2, /* N */
}