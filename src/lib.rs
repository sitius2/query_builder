#[allow(dead_code)]
#[allow(unused_assignments)]
/// Module that provides Functions and structs to easily create SQL-Queries
pub mod query_builder {
    // std imports
    use std::collections::BTreeMap;
    use std::fmt::{Display, Formatter, Result as FormatResult};
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
        /// ```ignore
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
        /// ```ignore
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        ///
        /// assert_eq!(q.as_string(), "SELECT user FROM users")
        /// ```
        pub fn from(mut self, t: &'a str) -> Self {
            self.from = t;
            self
        }

        /// Sets the limit value of the Query to the value of `l`
        /// ```ignore
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        /// q.limit(12);
        ///
        /// assert_eq!(q.as_string(), "SELECT user FROM users LIMIT 12")
        /// ```
        pub fn limit(&mut self, l: usize) {
            self.limit = Some(l);
        }

        /// Return whether or not the `SelectQuery` has a limit
        /// ```ignore
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        /// q.limit(12);
        /// assert!(q.has_limit);
        /// q.clear_limit();
        /// assert!(!q.has_limit);
        /// ```
        pub fn has_limit(&self) -> bool {
        	if let Some(_) = self.limit {
        		return true;
        	}

        	false
        }

        /// Returns the value of the Limit of the `SelectQuery` if there is one
        /// ```ignore
        /// let mut q = SelectQuery::select(&["user"]).from("users");
        /// assert_eq!(q.get_limit(), None);
        /// q.limit(12);
        /// assert_eq!(q.get_limit(), Some(12));
        /// ```
        pub fn get_limit(&self) -> Option<usize> {
        	self.limit
        }

        /// Removes the limit from the query
        /// ```ignore
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
        /// ```ignore
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

    /// Struct representing an SQL Insert Statement
    pub struct InsertQuery<'a> {
        into: &'a str,
        pub values: BTreeMap<&'a str, Value<'a>>,
    }

    impl<'a> Display for InsertQuery<'a> {
    	fn fmt(&self, f: &mut Formatter) -> FormatResult {
    		write!(f, "{}", self.as_string())
    	}
    }

    impl<'a> InsertQuery<'a> {
    	/// Creates a new `InsertQuery` that inserts data in table specified by `table`
        pub fn into(table: &'a str) -> InsertQuery<'a> {
            InsertQuery {
                into: table,
                values: BTreeMap::new(),
            }
        }

        /// Returns a `String` that represents the `InsertQuery` in a valid SQL statement
        /// 
        /// ```ignore
        /// let mut q = InsertQuery::into("users");
        /// q.values.insert("name", Value::Varchar("greg"));
        ///
        /// assert_eq!(q.as_string(), "INSERT INTO users(name) VALUES(greg)")
        /// ```
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

    /// Struct representing a SQL Delete Statement
    pub struct DeleteQuery<'a, 'c> {
    	from: &'a str,
    	pub whre: BTreeMap<&'a str, Value<'c>>,
    	limit: Option<usize>
    }

    impl<'a, 'c> DeleteQuery<'a, 'c> {
    	/// Return a new `DeleteQuery` that deletes data from table `table`
    	pub fn from(table: &'a str) -> DeleteQuery {
    		DeleteQuery {
    			from: table,
    			whre: BTreeMap::new(),
    			limit: None
    		}
    	}

    	/// Sets the limit of items to delete
    	pub fn limit(&mut self, limit: usize) {
    		self.limit = Some(limit);
    	}

    	/// Returns the limit of the `DeleteQuery`
    	pub fn get_limit(&self) -> Option<usize> {
    		self.limit
    	}

    	/// Removes the limit from the `DeleteQuery`
    	pub fn clear_limit(&mut self) {
    		self.limit = None;
    	}

    	/// Return a `String` representing the struct
    	pub fn as_string(&self) -> String {
    		let mut res = String::new();

    		res = format!("DELETE FROM {}", self.from);

    		if !self.whre.is_empty() {
    			let mut keys = self.whre.keys();
    			let key = keys.next().unwrap();
    			res = format!("{} WHERE {} = {}", res, key, self.whre[key]);
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

    /// Struct representing an SQL Update statement
    pub struct UpdateQuery<'a, 'c> {
    	update: &'a str,
    	pub set: BTreeMap<&'a str, Value<'c>>,
    	pub whre: BTreeMap<&'a str, Value<'c>>,
    	limit: Option<usize>
    }

    impl<'a, 'c> UpdateQuery<'a, 'c> {
    	/// Returns a new `UpdateQuery` that updates the table `table`
    	pub fn update(table: &'a str) -> UpdateQuery {
    		UpdateQuery {
    			update: table,
    			set: BTreeMap::new(),
    			whre: BTreeMap::new(),
    			limit: None,
    		}
    	}

    	/// Set the limit of the Query to the value of `l`
    	pub fn limit(&mut self, l: usize) {
    		self.limit = Some(l);
    	}

    	/// Returns whether or not the `UpdateQuery` has a limit
    	pub fn has_limit(&self) -> bool {
    		if let Some(_) = self.limit {
    			return true;
    		}

    		false
    	}
    	/// Returns the limit of the `UpdateQuery` if there is one
    	pub fn get_limit(&self) -> Option<usize> {
    		self.limit
    	}

    	/// Returns the String representation of the `UpdateQuery`
    	pub fn as_string(&self) -> String {
    		let mut res = String::new();

    		res = format!("UPDATE {}", self.update);

    		if !self.set.is_empty() {
    			let mut keys = self.set.keys();
    			let key = keys.next().unwrap();

    			res = format!("{} SET {} = {}", res, key, self.set[key]);

    			for k in keys {
    				res = format!("{}, {} = {}", res, k, self.set[k]);
    			}
    		}

    		if !self.whre.is_empty() {
    			let mut keys = self.whre.keys();
    			let key = keys.next().unwrap();

    			res = format!("{} WHERE {} = {}", res, key, self.whre[key]);

    			for k in keys {
    				res = format!("{} AND {} = {}", res, k, self.whre[k]);
    			}
    		}

    		if let Some(l)	= self.limit {
    			res = format!("{} LIMIT {}", res, l);
    		}

    		res
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

    #[test]
    fn delete_simple() {
    	let mut q = DeleteQuery::from("users");
    	q.whre.insert("name", Value::Varchar("george"));

    	assert_eq!(q.as_string(), "DELETE FROM users WHERE name = 'george'")
    }

    #[test]
    fn delete_simple_limit() {
    	let mut q = DeleteQuery::from("countries");
    	q.limit(1);

    	assert_eq!(q.as_string(), "DELETE FROM countries LIMIT 1")
    }

    #[test]
    fn update_simple() {
    	let mut q = UpdateQuery::update("users");
    	q.set.insert("name", Value::Varchar("george"));

    	assert_eq!(q.as_string(), "UPDATE users SET name = 'george'")
    }

    #[test]
    fn update_simple_where() {
    	let mut q = UpdateQuery::update("users");
    	q.set.insert("name", Value::Varchar("george"));
    	q.whre.insert("name", Value::Varchar("steve"));

    	assert_eq!(q.as_string(), "UPDATE users SET name = 'george' WHERE name = 'steve'")
    }

    #[test]
    fn update_simple_where_limit() {
    	let mut q = UpdateQuery::update("users");
    	q.set.insert("name", Value::Varchar("george"));
    	q.whre.insert("name", Value::Varchar("steve"));
    	q.limit(1);

    	assert_eq!(q.as_string(), "UPDATE users SET name = 'george' WHERE name = 'steve' LIMIT 1");
    }
}