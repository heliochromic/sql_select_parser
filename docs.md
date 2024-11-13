# sql_select_parser

[Link to crates.io](https://crates.io/crates/sql_select_parser)

## Overview
`sql_select_parser` is a CLI tool for parsing SQL SELECT queries into a structured Abstract Syntax Tree (AST) using Rust and pest. Ideal for developers and DB admins, this tool helps validate and analyze SQL queries programmatically.

## Features
- **SQL Parsing**: Parses SELECT queries with various clauses and functions.
- **AST Generation**: Transforms SQL into an AST for analysis and manipulation.
- **CLI Interface**: Built with clap for easy command-line use.
- **Makefile**: Automates tasks like building, testing, and formatting.
- **Detailed Error Handling**: Identifies SQL syntax issues with clear error messages.

# Grammar Explanation

This grammar defines the syntax rules for a simple SQL parser that supports basic `SELECT` queries with columns, tables, conditions, and functions.

- **WHITESPACE**: Matches spaces, tabs, newlines, and carriage returns. Ignored during parsing.
  
- **Keywords**:
  - `SELECT`, `FROM`, `WHERE`: Matches SQL keywords to define different clauses.

- **identifier**: Matches valid SQL identifiers, which start with a letter or underscore and can contain alphanumeric characters or underscores.

- **number**: Matches integers, optionally with a negative sign.

- **string**: Matches single-quoted text strings.

- **boolean**: Matches boolean values (`true` or `false`).

- **operator**: Matches common comparison operators (`!=`, `>=`, `<=`, `<`, `>`, `=`).

- **function_name**: Matches function names, which start with a letter or underscore and can contain alphanumeric characters or underscores.

- **table**: Matches a table, which can be a simple identifier or a nested `SELECT` query as a subquery.

- **select_query**: Defines the structure of a `SELECT` query, which includes:
  - A `SELECT` keyword followed by a list of selected items (`select_list`).
  - A `FROM` clause specifying the table.
  - An optional `WHERE` clause with conditions.

- **select_list**: Matches a list of items (columns or functions) to be selected, separated by commas.

- **star**: Matches the `*` character to select all columns.

- **select_item**: Represents an item in the `SELECT` clause, which can be a function call, identifier (column), or `*`.

- **function_call**: Matches a function call, allowing functions with arguments.

- **where_clause**: Matches a `WHERE` clause that contains conditions.

- **condition**: Represents a condition in the `WHERE` clause, using an identifier, operator, and value for comparison.

- **value**: Represents the values used in conditions, which can be a string, number, or boolean.
