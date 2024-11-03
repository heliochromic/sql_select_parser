# Parser of SQL queries on Rust using Pest
## Description of the Project
This project implements a simple parser for SELECT SQL queries in the Rust programming language using the Pest library. The parser is capable of analyzing basic SELECT queries, extracting information about selected columns, table, filtering conditions (WHERE), and sorting conditions (ORDER BY).

## Functionality
* **SELECT Query Parsing**: Support for basic SELECT query syntax with column and table specification.
* **WHERE Conditions Support**: Ability to parse simple filter conditions using comparison operators.
* **ORDER BY Support**: Ability to parse sorting conditions to order the results based on specified columns.
* **Abstract Syntax Tree (AST) Construction**: Structuring parsed queries as a data structure for further processing.
* **Error Handling**: Output informative messages in case of parsing errors.

## Installation

Add following dependecies to your Rust project
```toml
[dependencies]
anyhow = "1.0.92"
clap = "4.5.20"
pest = "2.7.14"
pest_derive = "2.7.14"
thiserror = "1.0.66"
```

Clone project

```unix
git clone https://github.com/your_username/sql_parser.git
cd sql_parser
```

Build project

```unix
cargo build
```