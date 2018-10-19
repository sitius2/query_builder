
#[cfg(test)]
mod tests {
	extern crate query_builder;
	use self::query_builder::*;
	#[test]
    fn select_simple() {
        let q = SelectQuery::select(&["user"]).from(&["users"]);

        assert_eq!(q.as_string(), "SELECT user FROM users")
    }

    #[test]
    fn select_simple_where() {
        let mut q = SelectQuery::select(&["user, name"]).from(&["users"]);
        q.whre.push(WhereClause::new("name", Value::Varchar("ezio"), None));

        assert_eq!(
            q.as_string(),
            "SELECT user, name FROM users WHERE name = 'ezio'"
        );
    }

    #[test]
    fn select_simple_where_limt() {
        let mut q = SelectQuery::select(&["user"]).from(&["users"]);
        q.whre.push(WhereClause::new("name", Value::Varchar("connor"), None));
        q.limit(42);

        assert_eq!(
            q.as_string(),
            "SELECT user FROM users WHERE name = 'connor' LIMIT 42"
        );
    }

    #[test]
    fn select_simple_order_by() {
        let mut q  = SelectQuery::select(&["user"]).from(&["users"]);
        
        q.order_by(OrderBy::Row("name"));

        assert_eq!(q.as_string(), "SELECT user FROM users ORDER BY name")
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
        q.whre.push(WhereClause::new("name", Value::Varchar("george"), None));

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
        q.whre.push(WhereClause::new("name", Value::Varchar("steve"), None));

        assert_eq!(
            q.as_string(),
            "UPDATE users SET name = 'george' WHERE name = 'steve'"
        )
    }

    #[test]
    fn update_simple_where_limit() {
        let mut q = UpdateQuery::update("users");
        q.set.insert("name", Value::Varchar("george"));
        q.whre.push(WhereClause::new("name", Value::Varchar("steve"), None));
        q.limit(1);

        assert_eq!(
            q.as_string(),
            "UPDATE users SET name = 'george' WHERE name = 'steve' LIMIT 1"
        );
    }
}