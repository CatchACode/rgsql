use core::panic;

#[derive(Debug, PartialEq, Default)]
pub enum SelectItemValue {
    Reference {
        value: String,
    },
    Integer {
        value: i64,
    },
    Real {
        value: f64,
    },
    Boolean {
        value: bool,
    },
    #[default]
    Invalid,
}

#[derive(Debug, PartialEq, Default)]
pub struct SelectItem {
    value: SelectItemValue,
    name: String,
}

#[derive(Debug, Default, PartialEq)]
pub struct FromClause {
    table: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct WhereClause {}

#[derive(Debug, PartialEq, Default)]
pub struct GroupByClause {}

#[derive(Debug, PartialEq, Default)]
pub struct JoinClause {}

#[derive(Debug, Default, PartialEq)]
pub struct SelectClause {
    items: Vec<SelectItem>,
    from: Option<FromClause>,
    group_by: Option<GroupByClause>,
    join: Option<JoinClause>,
}
#[derive(Debug, PartialEq)]
pub enum Statement {
    Select(SelectClause),
}

const KEYWORDS: &[&str] = &["select", "from", "as"];

fn tokenize(message: &String) -> Vec<&str> {
    println!("Tokenizing: \"{}\"", message);
    let is_delimiter = |c: char| c.is_whitespace() || c == '\0';
    let is_punctation = |c: char| matches!(c, ',' | '(' | ')' | ';');

    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in message.char_indices() {
        if is_delimiter(c) || is_punctation(c) {
            if start < i {
                tokens.push(&message[start..i]);
            }
            if is_punctation(c) {
                tokens.push(&message[i..i + c.len_utf8()]);
            }
            start = i + c.len_utf8();
        }
    }
    if start < message.len() {
        tokens.push(&message[start..]);
    }
    println!("Tokens are: {:?}", tokens);
    return tokens;
}

fn expect_select_item_value(tokens: &[&str], pos: usize) -> (SelectItemValue, usize) {
    match tokens[pos].to_lowercase().as_str() {
        "true" => return (SelectItemValue::Boolean { value: true }, pos + 1),
        "false" => return (SelectItemValue::Boolean { value: false }, pos + 1),
        _ => {
            if tokens[pos].parse::<i64>().is_ok() {
                return (
                    SelectItemValue::Integer {
                        value: tokens[pos].parse::<i64>().unwrap(),
                    },
                    pos + 1,
                );
            } else if tokens[pos].parse::<f64>().is_ok() {
                return (
                    SelectItemValue::Real {
                        value: tokens[pos].parse::<f64>().unwrap(),
                    },
                    pos + 1,
                );
            } else {
                return (
                    SelectItemValue::Reference {
                        value: tokens[pos].to_string(),
                    },
                    pos + 1,
                );
            }
        }
    }
}

fn expect_select_item(tokens: &[&str], pos: usize) -> (SelectItem, usize) {
    let (item, pos) = expect_select_item_value(tokens, pos);
    match tokens[pos].to_lowercase().as_str() {
        "from" => {
            return (
                SelectItem {
                    value: item,
                    name: tokens[pos - 1].to_string(),
                },
                pos,
            );
        }
        "," => {
            return (
                SelectItem {
                    value: item,
                    name: tokens[pos - 1].to_string(),
                },
                pos + 1,
            );
        }
        "as" => {
            return (
                SelectItem {
                    value: item,
                    name: tokens[pos + 2].to_string(),
                },
                pos + 3,
            );
        }
        ";" => {
            return (
                SelectItem {
                    value: item,
                    name: tokens[pos - 1].to_string(),
                },
                pos + 1,
            );
        }
        _ => {
            panic!("Cannot parse SelectItem: unexpected token {}!", tokens[pos]);
        }
    }
}

fn expect_from(tokens: &[&str], mut pos: usize) -> (FromClause, usize) {
    if tokens[pos].to_lowercase() != "from" {
        panic!(
            "Cannot parse from: expected \"from\" but got \"{}\"",
            tokens[pos]
        )
    }
    pos += 1;
    return (
        FromClause {
            table: tokens[pos].to_string(),
        },
        pos + 1,
    );
}

///
fn expect_select_items(tokens: &[&str], mut pos: usize) -> (Vec<SelectItem>, usize) {
    let mut items: Vec<SelectItem> = Vec::new();
    while (pos < tokens.len()) && (tokens[pos].to_lowercase() != "from") {
        let item;
        (item, pos) = expect_select_item(&tokens, pos);
        items.push(item);
    }

    return (items, pos);
}

fn expect_select(tokens: &[&str], mut pos: usize) -> (SelectClause, usize) {
    let mut select_items: Vec<SelectItem> = Vec::new();
    (select_items, pos) = expect_select_items(tokens, pos);
    let mut select_clause = SelectClause {
        items: select_items,
        from: None,
        group_by: None,
        join: None,
    };
    if (pos < tokens.len()) && (tokens[pos].to_lowercase().as_str() == "from") {
        let (from_clause, pos) = expect_from(tokens, pos);
        select_clause.from = Some(from_clause);
    }
    if (pos < tokens.len()) && (tokens[pos].to_lowercase().as_str() == "where") {
        panic!("Cannot parse select statement: where clause not implemented")
    }
    if (pos < tokens.len()) && (tokens[pos].to_lowercase().as_str() == "group") {
        panic!("Cannot parse select statement: group by clause not implemented")
    }
    return (select_clause, pos);
}

fn expect_root(tokens: &[&str], mut pos: usize) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();
    match tokens[pos].to_lowercase().as_str() {
        "select" => {
            let select_clause: SelectClause;
            (select_clause, pos) = expect_select(tokens, pos + 1);
            statements.push(Statement::Select(select_clause));
        }
        other => panic!("unexpected token: {other}"),
    }
    return statements;
}

pub fn parse(message: &String) -> Vec<Statement> {
    return expect_root(&tokenize(message), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_select() {
        let expected_statement_1: Statement = Statement::Select(SelectClause {
            items: vec![SelectItem {
                name: "3".to_string(),
                value: SelectItemValue::Integer { value: 3 },
            }],
            from: None,
            group_by: None,
            join: None,
        });

        let expected_statement_2: Statement = Statement::Select(SelectClause {
            items: vec![
                SelectItem {
                    name: "100".to_string(),
                    value: SelectItemValue::Integer { value: 100 },
                },
                SelectItem {
                    name: "TRUE".to_string(),
                    value: SelectItemValue::Boolean { value: true },
                },
                SelectItem {
                    name: "6".to_string(),
                    value: SelectItemValue::Integer { value: 6 },
                },
            ],
            from: None,
            group_by: None,
            join: None,
        });

        let expected_statement_3: Statement = Statement::Select(SelectClause {
            items: vec![],
            from: None,
            group_by: None,
            join: None,
        });
        let actual_statement_1: Statement =
            parse(&"SELECT 3;".to_string()).into_iter().nth(0).unwrap();
        let actual_statement_2: Statement = parse(&"SELECT 100, TRUE, 6;".to_string())
            .into_iter()
            .nth(0)
            .unwrap();
        assert_eq!(expected_statement_1, actual_statement_1);
        assert_eq!(expected_statement_2, actual_statement_2);
    }
}
