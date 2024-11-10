use anyhow::{anyhow, Error};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./sql.pest"]
pub struct SQLParser;

#[derive(Debug)]
pub struct SelectQuery {
    pub columns: Vec<SelectItem>,
    pub table: Table,
    pub where_clause: Option<Condition>,
}

#[derive(Debug)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Condition {
    pub left: String,
    pub operator: String,
    pub right: Value,
}

#[derive(Debug)]
pub enum SelectItem {
    Column(String),
    Function { name: String, argument: String },
}

#[derive(Debug)]
pub enum Table {
    Simple(String),
    Subquery(Box<SelectQuery>),
}

pub fn parse_query(input: &str) -> Result<SelectQuery, Error> {
    let mut pairs =
        SQLParser::parse(Rule::select_query, input).map_err(|e| anyhow!("Parsing error: {}", e))?;
    let pair = pairs.next().ok_or_else(|| anyhow!("No query found"))?;

    build_query_structure(pair)
}

fn build_query_structure(pair: Pair<Rule>) -> Result<SelectQuery, Error> {
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
        table: table.ok_or_else(|| anyhow!("Table not specified"))?,
        where_clause,
    })
}

fn selected_rows_parser(pair: Pair<Rule>) -> Result<Vec<SelectItem>, Error> {
    let mut selected_rows = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::select_item => {
                let row = parse_select_item(inner)?;
                selected_rows.push(row);
            }
            _ => return Err(anyhow!("Unexpected rule in select_list")),
        }
    }

    Ok(selected_rows)
}

fn parse_select_item(pair: Pair<Rule>) -> Result<SelectItem, Error> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| anyhow!("Missing select item"))?;

    match inner.as_rule() {
        Rule::identifier => Ok(SelectItem::Column(inner.as_str().to_string())),
        Rule::function_call => {
            let mut parts = inner.into_inner();
            let function = parts
                .next()
                .ok_or_else(|| anyhow!("Function name missing"))?
                .as_str()
                .to_string();
            let arg = parts
                .next()
                .ok_or_else(|| anyhow!("Function argument missing"))?
                .as_str()
                .to_string();

            Ok(SelectItem::Function {
                name: function,
                argument: arg,
            })
        }
        _ => return Err(anyhow!("Unexpected rule in select_item")),
    }
}

fn table_parser(pair: Pair<Rule>) -> Result<Table, Error> {
    let inner = pair
        .into_inner()
        .next()
        .ok_or_else(|| anyhow!("Missing table name"))?;

    match inner.as_rule() {
        Rule::identifier => Ok(Table::Simple(inner.as_str().to_string())),
        Rule::select_query => {
            let subquery = build_query_structure(inner)?;
            Ok(Table::Subquery(Box::new(subquery)))
        }
        _ => return Err(anyhow!("Unexpected rule in table")),
    }
}

fn where_parser(pair: Pair<Rule>) -> Result<Condition, Error> {
    // let mut inner_rules = condition_pair.into_inner();

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

    Err(anyhow!("No condition found in WHERE clause"))

}

fn parse_condition(pair: Pair<Rule>) -> Result<Condition, Error> {
    let mut inner_rules = pair.into_inner();

    let left = inner_rules
        .next()
        .ok_or_else(|| anyhow!("Missing left operand in condition"))?
        .as_str()
        .to_string();

    let operator = inner_rules
        .next()
        .ok_or_else(|| anyhow!("Missing operator in condition"))?
        .as_str()
        .to_string();

    let right_pair = inner_rules
        .next()
        .ok_or_else(|| anyhow!("Missing right operand in condition"))?;
    let right = parse_value(right_pair)?;

    Ok(Condition { left, operator, right })
}

fn parse_value(pair: Pair<Rule>) -> Result<Value, Error> {
    let inner_pair = pair.into_inner().next().ok_or_else(|| anyhow!("Expected inner rule for value"))?;
    
    match inner_pair.as_rule() {
        Rule::number => {
            let num = inner_pair
                .as_str()
                .parse::<i64>()
                .map_err(|e| anyhow!("Invalid number: {}", e))?;
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
        _ => Err(anyhow!("Unexpected rule in value")),
    }
}
