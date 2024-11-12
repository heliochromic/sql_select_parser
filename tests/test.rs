use anyhow::{Context, Result};
use sql_select_parser::parse_query;
use sql_select_parser::{SelectItem, Table, Value};

#[test]
fn test_whitespace_handling() -> Result<()> {
    let query = "select name, age from users where active = true  ";

    let parsed = parse_query(query).context("Failed to parse query with various whitespace")?;

    assert_eq!(parsed.columns.len(), 2);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }
    match &parsed.columns[1] {
        SelectItem::Column(col) => assert_eq!(col, "age"),
        _ => panic!("Expected column 'age'"),
    }

    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
        _ => panic!("Expected simple table 'users'"),
    }

    match parsed.where_clause {
        Some(ref condition) => {
            assert_eq!(condition.left, "active");
            assert_eq!(condition.operator, "=");
            match condition.right {
                Value::Boolean(b) => assert!(b),
                _ => panic!("Expected boolean value in WHERE clause"),
            }
        }
        None => panic!("Expected where clause"),
    }

    Ok(())
}

#[test]
fn test_select_keyword() -> Result<()> {
    let query = "select name from users";

    let parsed = parse_query(query).context("Failed to parse query with select keyword")?;

    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }

    Ok(())
}

#[test]
fn test_from_keyword() -> Result<()> {
    let query = "select name from users";

    let parsed = parse_query(query).context("Failed to parse query with from keyword")?;

    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
        _ => panic!("Expected simple table 'users'"),
    }

    Ok(())
}

#[test]
fn test_where_keyword() -> Result<()> {
    let query = "select name from users where active = true";

    let parsed = parse_query(query).context("Failed to parse query with WHERE keyword")?;

    assert!(parsed.where_clause.is_some());

    Ok(())
}

#[test]
fn test_identifier() -> Result<()> {
    let query = "select user_name, _email from users_table";

    let parsed = parse_query(query).context("Failed to parse query with identifiers")?;

    assert_eq!(parsed.columns.len(), 2);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "user_name"),
        _ => panic!("Expected column 'user_name'"),
    }
    match &parsed.columns[1] {
        SelectItem::Column(col) => assert_eq!(col, "_email"),
        _ => panic!("Expected column '_email'"),
    }

    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users_table"),
        _ => panic!("Expected simple table 'users_table'"),
    }

    Ok(())
}

#[test]
fn test_number_value() -> Result<()> {
    let query = "select id from orders where quantity = 100";

    let parsed = parse_query(query).context("Failed to parse query with numeric value")?;

    match parsed.where_clause {
        Some(ref condition) => {
            assert_eq!(condition.left, "quantity");
            assert_eq!(condition.operator, "=");
            match condition.right {
                Value::Number(n) => assert_eq!(n, 100),
                _ => panic!("Expected number value in where clause"),
            }
        }
        None => panic!("Expected WHERE clause"),
    }

    Ok(())
}

#[test]
fn test_function_name() -> Result<()> {
    let query = "select count(id) from orders";

    let parsed = parse_query(query).context("Failed to parse query with function call")?;

    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Function { name, arguments } => {
            assert_eq!(name, "count");
            assert_eq!(arguments.len(), 1);
            match &arguments[0] {
                SelectItem::Column(col) => assert_eq!(col, "id"),
                _ => panic!("Expected column 'id' as function argument"),
            }
        }
        _ => panic!("Expected function in select columns"),
    }

    Ok(())
}

#[test]
fn test_select_star() -> Result<()> {
    let query = "select * from users";

    let parsed = parse_query(query).context("Failed to parse select * query")?;

    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "*"),
        _ => panic!("Expected '*' in select columns"),
    }

    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
        _ => panic!("Expected simple table 'users'"),
    }

    Ok(())
}

#[test]
fn test_select_list() -> Result<()> {
    let query = "select id, name, email from users";

    let parsed = parse_query(query).context("Failed to parse select list with multiple items")?;

    assert_eq!(parsed.columns.len(), 3);
    let expected_columns = vec!["id", "name", "email"];
    for (i, col) in expected_columns.iter().enumerate() {
        match &parsed.columns[i] {
            SelectItem::Column(c) => assert_eq!(c, col),
            _ => panic!("Expected column '{}'", col),
        }
    }

    Ok(())
}

#[test]
fn test_table_subquery() -> Result<()> {
    let query = "select name from (select name, age from users)b";

    let parsed = parse_query(query).context("Failed to parse subquery in from clause")?;

    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(ref col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }
    match parsed.table {
        Table::Subquery(ref subquery) => {
            assert_eq!(subquery.columns.len(), 2);
            match &subquery.columns[0] {
                SelectItem::Column(col) => assert_eq!(col, "name"),
                _ => panic!("Expected column 'name' in subquery"),
            }
            match &subquery.columns[1] {
                SelectItem::Column(col) => assert_eq!(col, "age"),
                _ => panic!("Expected column 'age' in subquery"),
            }

            match subquery.table {
                Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
                _ => panic!("Expected simple table 'users' in subquery"),
            }

            assert!(subquery.where_clause.is_none());
        }
        _ => panic!("Expected table as subquery"),
    }

    Ok(())
}

#[test]
fn test_invalid_select_missing_columns() -> Result<()> {
    let query = "select from users";

    let parsed = parse_query(query);

    assert!(
        parsed.is_err(),
        "Expected parsing to fail due to missing columns"
    );

    if let Err(e) = parsed {
        println!("Error: {}", e);
    }

    Ok(())
}
