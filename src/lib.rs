//! # About
//! This crate is intended to be easy to use for creating
//! SQL-Queries dynamically as you need it.
//!
//! # Usage
//! For creating a simple SELECT-Query you first need to add
//! `query_builder = "*"` to your `Cargo.toml` file (you may of cource replace * with any version)<br>
//! In your code write `extern crate query_builder` and `use query_builder::*` to
//! import all structs and enums. <br>
//! Finally creating the [`SelectQuery`] looks like this:
//! ```no_run
//! extern crate query_builder;
//! use query_builder::SelectQuery;
//!
//! let query = SelectQuery::select(&["*"]).from("users");
//! // make sure the query looks like expected
//! assert_eq!(query.as_string(), "SELECT * FROM users");
//! ```
//! <br>
//! Creating a [`InsertQuery`] works similar:
//! ```no_run
//! extern crate query_buider;
//! use query_builder::{InsertQuery, Value};
//!
//! // create the basic query
//! let mut query = InsertQuery::into("users");
//! // add values to the query
//! query.values.insert("name", Value::Varchar("george"));
//!
//! // make sure that the query looks like expected
//! assert_eq!(query.as_string(), "INSERT INTO users(name) VALUES('george')");
//! ```
//! <br>
//! More detailed explanations and examples can be found at the corresponding sections
//! to the structs and enums
//!
//! [`SelectQuery`]: ./struct.SelectQuery.html
//! [`InsertQuery`]: ./struct.InsertQuery.html
//! [`Value`]: ./enum.Value.html


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

#[allow(unused_assignments)]
impl<'c> Value<'c> {
    /// Convert the Value to a `String`
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::Value;
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// Representing the way to Format the ORDER BY clause of some queries
pub enum OrderBy<'b> {
    Row(&'b str),
    Expression(&'b str),
}

impl<'b> OrderBy<'b> {
    pub fn as_string(&self) -> String {
        match *self {
            OrderBy::Row(r) => format!("ORDER BY {}", r),
            OrderBy::Expression(e)  => format!("ORDER BY {}", e),
        }
    }
}

impl<'b> Display for OrderBy<'b> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Debug)]
/// Struct representing a SQL-INSERT Query
/// A simple query to select everything from a table can be created like this:
/// ```no_run
/// extern crate query_builder;
/// use query_builder::SelectQuery;
///
/// // create the query
/// let query = SelectQuery::select(&[*]).from("users");
///
/// // make sure it looks like you would expect it to look
/// assert_eq!(query.as_string(), "SELECT * FROM users");
/// ```
pub struct SelectQuery<'a, 'c> {
    select: Vec<&'a str>,
    from: &'a str,
    pub whre: BTreeMap<&'a str, Value<'c>>,
    limit: Option<usize>,
    order_by: Option<OrderBy<'c>>
}

impl<'a, 'c> Display for SelectQuery<'a, 'c> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.as_string())
    }
}


#[allow(unused_assignments)]
impl<'a, 'c> SelectQuery<'a, 'c> {
    /// Creates a new [`SelectQuery`] that selects data from the row/s `rows`
    ///
    /// [`SelectQuery`]: ./struct.SelectQuery.html
    ///
    pub fn select(rows: &[&'a str]) -> SelectQuery<'a, 'c> {
        SelectQuery {
            select: rows.to_vec(),
            from: "",
            whre: BTreeMap::new(),
            limit: None,
            order_by: None,
        }
    }

    /// Sets the table to select from to the value of `t`
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery;
    ///
    /// let mut q = SelectQuery::select(&["user"]).from("users");
    ///
    /// assert_eq!(q.as_string(), "SELECT user FROM users")
    /// ```
    pub fn from(mut self, t: &'a str) -> Self {
        self.from = t;
        self
    }
    /// Sets the limit value of the Query to the value of `l`
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery;
    ///
    /// let mut q = SelectQuery::select(&["user"]).from("users");
    /// q.limit(12);
    ///
    /// assert_eq!(q.as_string(), "SELECT user FROM users LIMIT 12")
    /// ```
    pub fn limit(&mut self, l: usize) {
        self.limit = Some(l);
    }

    /// Return whether or not the `SelectQuery` has a limit
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery:
    ///
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
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery;
    /// 
    /// let mut q = SelectQuery::select(&["user"]).from("users");
    /// assert_eq!(q.get_limit(), None);
    /// 
    /// q.limit(12);
    /// assert_eq!(q.get_limit(), Some(12));
    /// ```
    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }

    /// Removes the limit from the query
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery;
    ///
    /// let mut q = SelectQuery::select(&["user"]).from("users");
    /// 
    /// // set the limit
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

    /// Adds a ORDER BY clause to the query
    pub fn order_by(&mut self, ob: OrderBy<'c>) {
        self.order_by = Some(ob);
    }
    /// Creates the string representation of the query
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::SelectQuery;
    ///
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

        if let Some(ref ob) = self.order_by {
            res = format!("{} {}", res, ob);
        }

        res
    }
}


#[derive(Debug)]
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

#[allow(unused_assignments)]
impl<'a> InsertQuery<'a> {
    /// Creates a new `InsertQuery` that puts data into `table`.
    pub fn into(table: &'a str) -> InsertQuery<'a> {
        InsertQuery {
            into: table,
            values: BTreeMap::new(),
        }
    }

    /// Returns a `String` that represents the `InsertQuery` in a valid SQL statement
    /// ## Example
    /// ```no_run
    ///
    /// extern crate query_builder;
    /// use query_builder::{Value, InsertQuery};
    ///
    /// let mut q = InsertQuery::into("users");
    /// q.values.insert("name", Value::Varchar("greg"));
    ///
    /// assert_eq!(q.as_string(), "INSERT INTO users(name) VALUES('greg')")
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

#[derive(Debug)]
/// Struct representing a SQL Delete Statement
pub struct DeleteQuery<'a, 'c> {
    from: &'a str,
    pub whre: BTreeMap<&'a str, Value<'c>>,
    limit: Option<usize>,
    order_by: Option<OrderBy<'c>>,
}

impl<'a, 'c> Display for DeleteQuery<'a, 'c> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.as_string())
    }
}

#[allow(unused_assignments)]
impl<'a, 'c> DeleteQuery<'a, 'c> {
    /// Return a new `DeleteQuery` that deletes data from table `table`
    pub fn from(table: &'a str) -> DeleteQuery {
        DeleteQuery {
            from: table,
            whre: BTreeMap::new(),
            limit: None,
            order_by: None,
        }
    }

    /// Sets the limit of items to delete
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::{DeleteQuery, Value};
    /// 
    /// let mut query = DeleteQuery::from("users");
    /// // add values to delete
    /// query.whre.insert("name", Value::Varchar("gregory"));
    /// 
    /// // add the limit
    /// query.limit(1);
    /// 
    /// // make sure the query looks like expected
    /// assert_eq!(query.as_string(), "DELETE FROM users WHERE name = 'gregory' LIMIT 1");
    /// ```
    pub fn limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    /// Returns the limit of the `DeleteQuery`
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::{DeleteQuery, Value};
    /// 
    /// // create query
    /// let mut query = QueryBuilder::from("users");
    /// 
    /// // set the limit
    /// query.limit(12);
    /// 
    /// assert_eq!(query.get_limit(), Some(12));
    /// ```
    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }

    /// Removes the limit from the `DeleteQuery`
    pub fn clear_limit(&mut self) {
        self.limit = None;
    }

    /// Adds a ORDER BY clause to the query
    pub fn order_by(&mut self, ob: OrderBy<'b>) {
        self.order_by = Some(ob);
    }

    /// Removes the ORDER BY clause from the query
    pub fn clear_order_by(&mut self) {
        self.order_by = None;
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

#[derive(Debug)]
/// Struct representing an SQL Update statement
pub struct UpdateQuery<'a, 'c> {
    update: &'a str,
    pub set: BTreeMap<&'a str, Value<'c>>,
    pub whre: BTreeMap<&'a str, Value<'c>>,
    limit: Option<usize>,
}

impl<'a, 'c> Display for UpdateQuery<'a, 'c> {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.as_string())
    }
}

#[allow(unused_assignments)]
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
    /// ## Example
    /// ```no_run
    /// extern crate query_builder;
    /// use query_builder::{UpdateQuery, Value};
    /// 
    /// let mut query 
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

        if let Some(l) = self.limit {
            res = format!("{} LIMIT {}", res, l);
        }

        res
    }
}
