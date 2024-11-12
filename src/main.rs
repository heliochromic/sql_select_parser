use sql_select_parser::parse_query;

fn main() {
    let query = "select * from users";
    let parsed_query = parse_query(query);
    println!("{:#?}", parsed_query);
}
