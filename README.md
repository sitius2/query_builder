--- Under construction ---

# Overview
## What is this?
This is a Library written in pure Rust intended to be easy to use for creating SQL Queries.
This is also a project I mainly started to improve my Rust skills. 

# Usage
In order to use this library add the line `query_builder = "*"` to the `[dependencies]` section of your `Cargo.toml`.
Then in your code you can use `extern crate query_builder;` and access it with `use query_builder::query_builder` (yes, that is currently very poor written and I will change this as soon as possible).

Creating a basic query that selects data from a table you can use the following code: 
```
extern crate query_builder;
use query_builder::query_builder::*;

fn main() {
    let mut query = SelectQuery::select(&["user"]).from("users");

    query.whre.insert("name", Value::Varchar("greg"));
    query.limit(1);

    assert_eq!(query.as_string(), "SELECT user FROM users WHERE name = 'greg' LIMIT 1");
}
```

