use std::collections::BTreeMap;

pub struct InsertQuery<'a, 'c> {
    into: &'a str,
    values: BTreeMap<&'a str, Value<'c>,
    whre: BTreeMap<&'a str, Value<'c>,
}