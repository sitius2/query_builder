#[allow(dead_code)]
/// Module that provides Functions and structs to easily create SQL-Queries
pub mod query_builder {
    // std imports
    use std::collections::BTreeMap;
    use std::fmt::{Display, Formatter, Result as FormatResult};
    use std::string::ToString;
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        /// ```no_run
        /// // Put single quotes around the varchar to not conflict with e.g. MySQL when inserting data
        /// let v = Value::Varchar("steven");
        /// assert_eq!(v.as_string(), "'steven'");
        ///
        /// // Bools are written in caps to make them stand out in queries
        /// let v = Value::Bool(true);
        /// assert_eq!(v.as_string(), "TRUE");
        ///
        /// // applies to all numeric Values
        /// let v = Value::Int(42);
        /// assert_eq!(v.as_string(), "42");
        /// ```
        pub fn as_string(&self) -> String {
            match *self {
                Value::Varchar(v) => format!("'{}'", v),
                Value::Bool(b) => if b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                },
                Value::Tinyint(t) => format!("{}", t),
                Value::UnsignedTinyint(ut) => format!("{}", ut),
                Value::Smallint(s) => format!("{}", s),
                Value::UnsignedSmallint(us) => format!("{}", us),
                Value::Int(i) => format!("{}", i),
                Value::UnsignedInt(ui) => format!("{}", ui),
                Value::Bigint(bi) => format!("{}", bi),
                Value::UnsignedBigint(ubi) => format!("{}", ubi),
            }
        }
    }

    impl<'c> Display for Value<'c> {
        fn fmt(&self, f: &mut Formatter) -> FormatResult {
            write!(f, "{}", self.as_string())
        }
    }

    #[derive(Debug)]
    /// Rust representation of an SQL Select Query
    pub struct SelectQuery<'a, 'c> {
        select: Vec<&'a str>,
        from: &'a str,
        pub whre: BTreeMap<&'a str, Value<'c>>,
        limit: Option<usize>,
        s: String,
    }

    impl<'a, 'c> SelectQuery<'a, 'c> {
        /// Creates a new `SelectQuery` that selects data from the row/s `rows`
        pub fn select(rows: &[&'a str]) -> SelectQuery<'a, 'c> {
            SelectQuery {
                select: rows.to_vec(),
                from: "",
                whre: BTreeMap::new(),
                limit: None,
                s: String::new(),
            }
        }

        /// Sets the table to select from to the value of `t`
        /// ```no_run
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        ///
        /// assert_eq!(q.as_string(), "SELECT user FROM users")
        /// ```
        pub fn from(mut self, t: &'a str) -> Self {
            self.from = t;
            self
        }

        /// Sets the limit value of the Query to the value of `l`
        /// ```no_run
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        /// q.limit(12);
        ///
        /// assert_eq!(q.as_string(), "SELECT user FROM users LIMIT 12")
        /// ```
        pub fn limit(&mut self, l: usize) {
            self.limit = Some(l);
        }

        /// Removes the limit from the query
        /// ```no_run
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        /// q.limit(42);
        /// assert_eq!(q.as_string(), "SELECT user FORM users LIMIT 42");
        ///
        /// // clear limit
        /// q.clear_limit()
        /// assert_eq!(q.as_string(), "SELECT user FROM users");
        /// ```
        pub fn clear_limit(&mut self) {
            self.limit = None;
        }
        /// Creates the string representation of the query
        /// ```no_run
        /// let mut q = SelectQuery::select(&["*"]).from("users");
        ///
        /// assert_eq!(q.as_string(), "SELECT * FROM users")
        /// ```
        pub fn as_string(&self) -> String {
            let mut res: String = String::new();
            if !self.select.is_empty() {
                res = format!("SELECT {}", self.select[0]);
                if self.select.len() > 1 {
                    for s in self.select[1..].iter() {
                        res = format!("{}, {}", res, s);
                    }
                }
            }

            if self.from.len() > 1 {
                res = format!("{} FROM {}", res, self.from);
            }

            if !self.whre.is_empty() {
                let mut keys = self.whre.keys();
                let key = keys.next().unwrap();
                res = format!("{} WHERE {} = {}", res, key, self.whre[key].as_string());
                for k in keys {
                    res = format!("{} AND {} = {}", res, k, self.whre[k]);
                }
            }

            if let Some(l) = self.limit {
                res = format!("{} LIMIT {}", res, l);
            }

            res
        }
    }

    impl<'a, 'c> Display for SelectQuery<'a, 'c> {
        fn fmt(&self, f: &mut Formatter) -> FormatResult {
            write!(f, "{}", self.as_string())
        }
    }

    pub struct InsertQuery<'a> {
        into: &'a str,
        pub values: BTreeMap<&'a str, Value<'a>>,
    }

    impl<'a> InsertQuery<'a> {
        pub fn into(table: &'a str) -> InsertQuery<'a> {
            InsertQuery {
                into: table,
                values: BTreeMap::new(),
            }
        }

        pub fn as_string(&self) -> String {
            let mut res = String::new();
            let (mut vals, mut vals_list) = (String::new(), String::new());

            res = format!("INSERT INTO {}", self.into);

            if !self.values.is_empty() {
                let mut keys = self.values.keys();
                let key = keys.next().unwrap();
                vals = format!("{}", key);
                vals_list = format!("{}", self.values[key]);

                for k in keys {
                    vals = format!("{}, {}", vals, k);
                    vals_list = format!("{}, {}", vals_list, self.values[k]);
                }
            }

            format!("{}({}) VALUES({})", res, vals, vals_list)
        }
    }
}

#[cfg(test)]
mod tests {
    use query_builder::*;
    #[test]
    fn select_simple() {
        let mut q = SelectQuery::select(&["user"]).from("users");

        assert_eq!(q.as_string(), "SELECT user FROM users")
    }

    #[test]
    fn select_simple_where() {
        let mut q = SelectQuery::select(&["user, name"]).from("users");
        q.whre.insert("name", Value::Varchar("ezio"));

        assert_eq!(
            q.as_string(),
            "SELECT user, name FROM users WHERE name = 'ezio'"
        );
    }

    #[test]
    fn select_simple_where_limt() {
        let mut q = SelectQuery::select(&["user"]).from("users");
        q.whre.insert("name", Value::Varchar("connor"));
        q.limit(42);

        assert_eq!(
            q.as_string(),
            "SELECT user FROM users WHERE name = 'connor' LIMIT 42"
        );
    }

    #[test]
    fn insert_simple() {
        let mut q = InsertQuery::into("users");
        q.values.insert("name", Value::Varchar("greg"));

        assert_eq!(q.as_string(), "INSERT INTO users(name) VALUES('greg')")
    }
}