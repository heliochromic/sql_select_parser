use anyhow::Error;
use sql_select_parser::parse_query;

fn main() -> Result<(), Error> {
    let queries = vec![
        "select id, name from users where active = true",
        "select * from products",
        "select sum(price) from sales where quantity >= 100",
        "select name from (select name from employees where department = 'HR') where active = false",
        "select id, name from where"
    ];

    for query in queries {
        println!("Parsing query: {}", query);
        match parse_query(query) {
            Ok(ast) => {
                println!("Parsed AST:\n{:#?}", ast);
            }
            Err(e) => {
                println!("Error: {}\n", e);
            }
        }
        println!("----------------------------------------\n");
    }

    Ok(())
}
