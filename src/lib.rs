use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar/sql.pest"]

pub struct SQLParser;   