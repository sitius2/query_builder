// std imports
use std::collections::BTreeMap;
use std::fmt::{Formatter, Display, Result as FormatResult};
use std::string::ToString;

#[derive(Debug)]
/// Enum representing common SQL-datatypes
pub enum Value<'c> {
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

impl<'c> Value<'c> {
	/// Convert the Value to a `String`
	pub fn to_string(&self) -> String {
		match *self {
			Value::Varchar(v)	=> format!("'{}'", v),
			Value::Bool(b)		=> if b { "TRUE".to_string() } else { "FALSE".to_string() },
			Value::Tinyint(t)	=> format!("{}", t),
			Value::UnsignedTinyint(ut) => format!("{}", ut),
			Value::Smallint(s)	=> format!("{}", s),
			Value::UnsignedSmallint(us)	=> format!("{}", us),
			Value::Int(i)		=> format!("{}", i),
			Value::UnsignedInt(ui)	=> format!("{}", ui),
			Value::Bigint(bi)	=> format!("{}", bi),
			Value::UnsignedBigint(ubi)	=> format!("{}", ubi),
		}
	}
}

impl<'c> Display for Value<'c> {
	fn fmt(&self, f: &mut Formatter) -> FormatResult {
		write!(f, "{}", self.to_string())
	}
}

#[derive(Debug)]
/// Rust representation of an SQL Select Query
pub struct SelectQuery<'a, 'c> {
	select: Vec<&'a str>,
	from: &'a str,
	pub whre: BTreeMap<&'a str, Value<'c>>,
	limit: Option<usize>
}

impl<'a, 'c> SelectQuery<'a, 'c> {
	/// Creates a new `SelectQuery` that selects data from the row/s `rows`
	pub fn select(rows: Vec<&'a str>) -> SelectQuery {
		SelectQuery {
			select: rows,
			from: "",
			whre: BTreeMap::new(),
			limit: None,
		}
	}

	/// Sets the table to select from to the value of `t`
	pub fn from(mut self, t: &'a str) -> Self {
		self.from = t;
		self
	}

	/// Sets the limit value of the Query to the value of `l`
	pub fn limit(&mut self, l: usize) {
		self.limit = Some(l);
	}

	/// Removes the limit from the query
	pub fn clear_limit(&mut self) {
		self.limit = None;
	}

	pub fn as_str(&self) -> &str {
		let res: &str = "",
		if !self.select.is_empty() {
			
		}
	}
}