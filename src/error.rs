// tests/test.rs

use your_project::parse_query;
use your_project::{Condition, SelectItem, SelectQuery, Table, Value};
use anyhow::{Context, Result};

#[test]
fn test_simple_select() -> Result<()> {
    let query = "SELECT name, age FROM users";

    let parsed = parse_query(query).context("Failed to parse simple select query")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 2);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }
    match &parsed.columns[1] {
        SelectItem::Column(col) => assert_eq!(col, "age"),
        _ => panic!("Expected column 'age'"),
    }

    // Assert table
    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
        _ => panic!("Expected simple table 'users'"),
    }

    // Assert no WHERE clause
    assert!(parsed.where_clause.is_none());

    Ok(())
}

#[test]
fn test_select_with_where() -> Result<()> {
    let query = "SELECT id, email FROM customers WHERE active = true";

    let parsed = parse_query(query).context("Failed to parse select with WHERE clause")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 2);
    match &parsed.columns[0] {
        SelectItem::Column(col) => assert_eq!(col, "id"),
        _ => panic!("Expected column 'id'"),
    }
    match &parsed.columns[1] {
        SelectItem::Column(col) => assert_eq!(col, "email"),
        _ => panic!("Expected column 'email'"),
    }

    // Assert table
    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "customers"),
        _ => panic!("Expected simple table 'customers'"),
    }

    // Assert WHERE clause
    match parsed.where_clause {
        Some(ref condition) => {
            assert_eq!(condition.left, "active");
            assert_eq!(condition.operator, "=");
            match condition.right {
                Value::Boolean(b) => assert!(b),
                _ => panic!("Expected boolean value in WHERE clause"),
            }
        }
        None => panic!("Expected WHERE clause"),
    }

    Ok(())
}

#[test]
fn test_select_with_function() -> Result<()> {
    let query = "SELECT COUNT(id) FROM orders";

    let parsed = parse_query(query).context("Failed to parse select with function")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Function { name, arguments } => {
            assert_eq!(name, "COUNT");
            assert_eq!(arguments.len(), 1);
            match &arguments[0] {
                SelectItem::Column(col) => assert_eq!(col, "id"),
                _ => panic!("Expected column 'id' as function argument"),
            }
        }
        _ => panic!("Expected function in select columns"),
    }

    // Assert table
    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "orders"),
        _ => panic!("Expected simple table 'orders'"),
    }

    // Assert no WHERE clause
    assert!(parsed.where_clause.is_none());

    Ok(())
}

#[test]
fn test_select_with_subquery() -> Result<()> {
    let query = "SELECT name FROM (SELECT name, age FROM users) AS sub";

    let parsed = parse_query(query).context("Failed to parse select with subquery")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(ref col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }

    // Assert table as subquery
    match parsed.table {
        Table::Subquery(ref subquery) => {
            // Inner subquery assertions
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

            // No WHERE clause in subquery
            assert!(subquery.where_clause.is_none());
        }
        _ => panic!("Expected table as subquery"),
    }

    // Assert alias if necessary (depending on your grammar)
    // This example assumes aliases are handled elsewhere

    // Assert no WHERE clause in outer query
    assert!(parsed.where_clause.is_none());

    Ok(())
}

#[test]
fn test_select_with_multiple_conditions() -> Result<()> {
    let query = "SELECT id FROM products WHERE price > 100 AND stock < 50";

    // Assuming your grammar and parser support multiple conditions (e.g., using AND)
    // If not, this test should be adjusted accordingly.

    let parsed = parse_query(query).context("Failed to parse select with multiple conditions")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(ref col) => assert_eq!(col, "id"),
        _ => panic!("Expected column 'id'"),
    }

    // Assert table
    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "products"),
        _ => panic!("Expected simple table 'products'"),
    }

    // Assert WHERE clause
    match parsed.where_clause {
        Some(ref condition) => {
            // Depending on your parser's structure for multiple conditions,
            // you might need to adjust this part.
            // For simplicity, assuming single condition for now.
            assert_eq!(condition.left, "price");
            assert_eq!(condition.operator, ">");
            match condition.right {
                Value::Number(n) => assert_eq!(n, 100),
                _ => panic!("Expected number value in WHERE clause"),
            }
        }
        None => panic!("Expected WHERE clause"),
    }

    Ok(())
}

#[test]
fn test_invalid_select_missing_columns() -> Result<()> {
    let query = "SELECT FROM users";

    let parsed = parse_query(query);

    assert!(parsed.is_err(), "Expected parsing to fail due to missing columns");

    if let Err(e) = parsed {
        println!("Error: {}", e);
    }

    Ok(())
}

#[test]
fn test_invalid_select_missing_table() -> Result<()> {
    let query = "SELECT name, age";

    let parsed = parse_query(query);

    assert!(parsed.is_err(), "Expected parsing to fail due to missing table");

    if let Err(e) = parsed {
        println!("Error: {}", e);
    }

    Ok(())
}

#[test]
fn test_invalid_syntax() -> Result<()> {
    let query = "SELECT name age FROM users";

    let parsed = parse_query(query);

    assert!(parsed.is_err(), "Expected parsing to fail due to invalid syntax");

    if let Err(e) = parsed {
        println!("Error: {}", e);
    }

    Ok(())
}

#[test]
fn test_invalid_where_clause() -> Result<()> {
    let query = "SELECT name FROM users WHERE active";

    let parsed = parse_query(query);

    assert!(parsed.is_err(), "Expected parsing to fail due to incomplete WHERE clause");

    if let Err(e) = parsed {
        println!("Error: {}", e);
    }

    Ok(())
}

#[test]
fn test_select_with_string_literal() -> Result<()> {
    let query = "SELECT name FROM users WHERE role = 'admin'";

    let parsed = parse_query(query).context("Failed to parse select with string literal in WHERE clause")?;

    // Assert columns
    assert_eq!(parsed.columns.len(), 1);
    match &parsed.columns[0] {
        SelectItem::Column(ref col) => assert_eq!(col, "name"),
        _ => panic!("Expected column 'name'"),
    }

    // Assert table
    match parsed.table {
        Table::Simple(ref table_name) => assert_eq!(table_name, "users"),
        _ => panic!("Expected simple table 'users'"),
    }

    // Assert WHERE clause
    match parsed.where_clause {
        Some(ref condition) => {
            assert_eq!(condition.left, "role");
            assert_eq!(condition.operator, "=");
            match &condition.right {
                Value::String(ref s) => assert_eq!(s, "admin"),
                _ => panic!("Expected string value in WHERE clause"),
            }
        }
        None => panic!("Expected WHERE clause"),
    }

    Ok(())
}
