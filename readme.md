# sql_select_parser
## Overview
The SQL Parser CLI is a command-line tool designed to parse and analyze SQL SELECT queries. Built with Rust, it leverages the pest parser generator to interpret SQL syntax and provides a structured Abstract Syntax Tree (AST) representation of the parsed queries. This tool is ideal for developers and database administrators who need to validate, analyze, or transform SQL queries programmatically.

## Features
* Parse SQL SELECT Queries: Supports parsing of standard SELECT statements, including various clauses and functions.
* Abstract Syntax Tree (AST) Generation: Transforms SQL queries into a structured AST for easy analysis and manipulation.
* Command-Line Interface: User-friendly CLI built with clap for seamless integration into development workflows.
* Automated Tasks with Makefile: Simplifies common tasks like building, testing, formatting, and linting through a comprehensive Makefile.
* Error Handling: Provides detailed error messages to help identify and resolve syntax issues in SQL queries.

## Description
* SelectQuery:

    * columns: A list of SelectItem representing the columns or functions selected.
    * table: Represents the source table, which can be a simple table or a subquery.
    * where_clause: An optional Condition for filtering results.
* SelectItem:

    * Column: Represents a column identifier.
    * Function: Represents a function call with a name and a list of arguments, which are themselves SelectItems.
* Table:

    * Simple: A simple table identifier.
    * Subquery: A nested SelectQuery acting as a table source.
* Condition:

    * left: The left operand (column name).
    * operator: The comparison operator.
    * right: The right operand (Value), which can be a number, string, or boolean.
* Value:

    * Number: An integer value.
    * String: A string literal.
    * Boolean: A boolean value (true or false).

## Installation
Clone repository and make sure that all dependecies installed properly 
```unix
git clone https://github.com/yourusername/sql_parser_cli.git
cd sql_parser_cli
```

```unix
anyhow = "1.0.92"
clap = { version = "4.5.20", features = ["derive"] }
pest = "2.7.14"
pest_derive = "2.7.14"
thiserror = "1.0.66"
```
Run demo
```unix
make example
```

## Example
Input
```sql
select id, name, email from users where active = true;
```
Output
```rust
SelectQuery {
    columns: [
        Column(
            "id",
        ),
        Column(
            "name",
        ),
        Column(
            "email",
        ),
    ],
    table: Simple(
        "users",
    ),
    where_clause: Some(
        Condition {
            left: "active",
            operator: "=",
            right: Boolean(
                true,
            ),
        },
    ),
}
```

## Grammar
```
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

SELECT = { "select" }
FROM = { "from" }
WHERE = { "where" }

identifier = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

number = @{ "-"? ~ ASCII_DIGIT+ }

string = @{ "'" ~ (!"'" ~ ANY)* ~ "'" }

boolean = @{ "true" | "false" }

operator = @{ "!=" | ">=" | "<=" | "<" | ">" | "=" }

function_name = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

table = { identifier | "(" ~ select_query ~ ")" }

select_query = { SELECT ~ select_list ~ FROM ~ table ~ where_clause? }

select_list = { select_item ~ ("," ~ select_item)* }

star = { "*" }

select_item = { function_call | identifier | star }

function_call = { function_name ~ "(" ~ ("*" | select_item ~ ("," ~ select_item)*)? ~ ")" }

where_clause = { WHERE ~ condition }

condition = { identifier ~ operator ~ value }

value = { string | number | boolean }
```