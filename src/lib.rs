#![doc = include_str!("../docs.md")]

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Main module that contains rules for parser
#[derive(Parser)]
#[grammar = "./sql.pest"]
pub struct SQLParser;

/// Main structure for storing a SQL select query.
/// Contains selected columns, the table, and an optional WHERE filter.
#[derive(Debug)]
pub struct SelectQuery {
    /// List of columns or functions to select.
    pub columns: Vec<SelectItem>,
    /// The table to select data from.
    pub table: Table,
    /// Filtering conditions in WHERE, if present.
    pub where_clause: Option<Condition>,
}

/// Possible values for SQL expressions.
#[derive(Debug)]
pub enum Value {
    /// A number.
    Number(i64),
    /// A string (text).
    String(String),
    /// A boolean (true or false).
    Boolean(bool),
}

/// A condition for where.
/// Contains the column name, comparison operator, and the value to compare.
#[derive(Debug)]
pub struct Condition {
    /// The left part of the condition, like a column name.
    pub left: String,
    /// Comparison operator, such as `=` or `>`.
    pub operator: String,
    /// The value to compare against.
    pub right: Value,
}

/// Types of items in select: a simple column, a function, or a star (*).
#[derive(Debug)]
pub enum SelectItem {
    /// A column with a name.
    Column(String),
    /// A function call, with a function name and arguments.
    Function {
        name: String,
        arguments: Vec<SelectItem>,
    },
}

/// The table for select, which can be a simple table or a subquery.
#[derive(Debug)]
pub enum Table {
    /// A table name.
    Simple(String),
    /// A subquery SELECT.
    Subquery(Box<SelectQuery>),
}

/// Possible errors when parsing an SQL query.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error("No query found")]
    NoQueryFound,

    #[error("Table not specified")]
    TableNotSpecified,

    #[error("Unexpected rule in select_list")]
    UnexpectedRuleInSelectList,

    #[error("Unexpected rule in select_item")]
    UnexpectedRuleInSelectItem,

    #[error("Missing select item")]
    MissingSelectItem,

    #[error("Missing table name")]
    MissingTableName,

    #[error("Unexpected rule in table")]
    UnexpectedRuleInTable,

    #[error("No condition found in WHERE clause")]
    NoConditionInWhereClause,

    #[error("Missing left operand in condition")]
    MissingLeftOperand,

    #[error("Missing operator in condition")]
    MissingOperator,

    #[error("Missing right operand in condition")]
    MissingRightOperand,

    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),

    #[error("Unexpected rule in value")]
    UnexpectedRuleInValue,

    #[error("Expected inner rule for value")]
    ExpectedInnerRuleForValue,

    #[error("Function name missing")]
    FunctionNameMissing,

    #[error("Unexpected rule in select_list")]
    UnexpectedRuleInSelectListOther,
}

/// Function to parse an SQL query.
/// Takes an SQL query as text and returns a SelectQuery or an error.
///
/// # Example
///
/// ```
/// let query = "select name from users where age > 20";
/// let result = parse_query(query);
/// assert!(result.is_ok());
/// ```
pub fn parse_query(input: &str) -> Result<SelectQuery, ParseError> {
    let mut pairs = SQLParser::parse(Rule::select_query, input)
        .map_err(|e| ParseError::ParsingError(e.to_string()))?;
    let pair = pairs.next().ok_or(ParseError::NoQueryFound)?;

    build_query_structure(pair)
}

fn build_query_structure(pair: Pair<Rule>) -> Result<SelectQuery, ParseError> {
    let mut columns: Vec<SelectItem> = Vec::new();
    let mut table: Option<Table> = None;
    let mut where_clause: Option<Condition> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::select_list => {
                columns = selected_rows_parser(inner)?;
            }
            Rule::table => {
                table = Some(table_parser(inner)?);
            }
            Rule::where_clause => {
                where_clause = Some(where_parser(inner)?);
            }
            _ => {}
        }
    }

    Ok(SelectQuery {
        columns,
        table: table.ok_or(ParseError::TableNotSpecified)?,
        where_clause,
    })
}

fn selected_rows_parser(pair: Pair<Rule>) -> Result<Vec<SelectItem>, ParseError> {
    let mut selected_rows = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::select_item => {
                let row = parse_select_item(inner)?;
                selected_rows.push(row);
            }
            _ => return Err(ParseError::UnexpectedRuleInSelectListOther),
        }
    }

    Ok(selected_rows)
}

fn parse_select_item(pair: Pair<Rule>) -> Result<SelectItem, ParseError> {
    let mut inner_pairs = pair.into_inner();

    if let Some(inner) = inner_pairs.next() {
        match inner.as_rule() {
            Rule::identifier => Ok(SelectItem::Column(inner.as_str().to_string())),
            Rule::function_call => {
                let mut parts = inner.into_inner();
                let function = parts
                    .next()
                    .ok_or(ParseError::FunctionNameMissing)?
                    .as_str()
                    .to_string();
                let arguments = parts
                    .map(parse_select_item)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(SelectItem::Function {
                    name: function,
                    arguments,
                })
            }
            Rule::star => Ok(SelectItem::Column("*".to_string())),
            _ => Err(ParseError::UnexpectedRuleInSelectItem),
        }
    } else {
        Err(ParseError::MissingSelectItem)
    }
}

fn table_parser(pair: Pair<Rule>) -> Result<Table, ParseError> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or(ParseError::MissingTableName)?;

    match inner.as_rule() {
        Rule::identifier => Ok(Table::Simple(inner.as_str().to_string())),
        Rule::select_query => {
            let subquery = build_query_structure(inner)?;
            Ok(Table::Subquery(Box::new(subquery)))
        }
        _ => Err(ParseError::UnexpectedRuleInTable),
    }
}

fn where_parser(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::WHERE => {}
            Rule::condition => {
                let condition = parse_condition(inner)?;
                return Ok(condition);
            }
            _ => {}
        }
    }

    Err(ParseError::NoConditionInWhereClause)
}

fn parse_condition(pair: Pair<Rule>) -> Result<Condition, ParseError> {
    let mut inner_rules = pair.into_inner();

    let left = inner_rules
        .next()
        .ok_or(ParseError::MissingLeftOperand)?
        .as_str()
        .to_string();

    let operator = inner_rules
        .next()
        .ok_or(ParseError::MissingOperator)?
        .as_str()
        .to_string();

    let right_pair = inner_rules.next().ok_or(ParseError::MissingRightOperand)?;
    let right = parse_value(right_pair)?;

    Ok(Condition {
        left,
        operator,
        right,
    })
}

fn parse_value(pair: Pair<Rule>) -> Result<Value, ParseError> {
    let inner_pair = pair
        .into_inner()
        .next()
        .ok_or(ParseError::ExpectedInnerRuleForValue)?;

    match inner_pair.as_rule() {
        Rule::number => {
            let num = inner_pair.as_str().parse::<i64>()?;
            Ok(Value::Number(num))
        }
        Rule::string => {
            let s = inner_pair.as_str();
            let s = &s[1..s.len() - 1];
            Ok(Value::String(s.to_string()))
        }
        Rule::boolean => {
            let b = inner_pair.as_str().eq_ignore_ascii_case("true");
            Ok(Value::Boolean(b))
        }
        _ => Err(ParseError::UnexpectedRuleInValue),
    }
}
