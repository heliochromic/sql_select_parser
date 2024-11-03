fn test_single_join_query() {
    let input = "
    SELECT employees.name, departments.department_name
    FROM employees
    WHERE departments.location = 'Kyiv';
    ";

    let parse_result = SQLParser::parse(Rule::sql, input);
    assert!(parse_result.is_ok(), "Failed to parse: {}", input);
}
