// std imports
use std::collections::BTreeMap;
use std::fmt::{Result as fres, Formatter, Display};

/// This Enum represents the most common datatypes used in SQL-land
/// More may be added in the future
pub enum Value<'c> {
    /// Rust representation of the SQL VARCHAR
    Varchar(&'c str),
    Bool(bool),
    Tinyint(i8),
    UnsignedTinyint(u8),
    Smallint(i16),
    UnsignedSmallint(u16),
    Int(i32),
    UnsignedInt(u32),
    Bigint(i64),
    UnsignedBigint(u64),
}

#[dervice(Debug)]
impl<'c> Display for Value<'c> {
    fn fmt(&self, f: &mut Formatter) -> fres {
        match *self {
            Value::Varchar(v)           => write!(f, "'{}'", v),
            Value::Bool(b)              => if b { write!(f, "TRUE") } else { write!(f, "FALSE") },
            Value::Tinyint(t)           => write!(f, "{}", t),
            Value::UnsignedTinyint(ut)  => write!(f, "{}", ut),
            Value::Smallint(s)          => write!(f, "{}", s),
            Value::UnsignedSmallint(us) => write!(f, "{}", us),
            Value::Int(i)               => write!(f, "{}", i),
            Value::UnsignedInt(ui)      => write!(f, "{}", ui),
            Value::Bigint(bi)           => write!(f, "{}", bi),
            Value::UnsignedBigint(ubi)  => write!(f, "{}", ubi),
        }
    }
}

impl<'c> Value<'c> {

}

/// SQL Insert Query represented as Rust object.
#[derive(Debug)]
pub struct InsertQuery<'a, 'c> {
    into: &'a str,
    pub values: BTreeMap<&'a str, Value<'c>>,
    pub whre: BTreeMap<&'a str, Value<'c>>,
    limit: Option<usize>,
    s: String;
}

impl<'a, 'c> InsertQuery<'a, 'c> {
    /// Create a new Query that inserts into `table`
    pub fn into(table: &'a str) -> InsertQuery {
        InsertQuery {
            into: table,
            values: BTreeMap::new(),
            whre: BTreeMap::new(),
            limit: None,
            s: String::new(),
        }
    }
    /// Set the Limit for the Query to `l`
    pub fn limit(&mut self, l: usize) {
        self.limit = Some(l);
    }

    /// Return the limit of the Query, if there is any
    pub fn get_limit(&self) -> Option<usize> {
        if let Some(l)  = self.limit {
            Some(l)
        } else {
            None
        }
    }

    /// Remove the limit from the Query
    pub fn clear_limit(&mut self) {
        self.limit = None
    }
}