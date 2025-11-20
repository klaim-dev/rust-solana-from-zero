#[derive(Debug, PartialEq)]
pub struct Order {
    pub id: u32,
    pub name: String,
    pub qty: u32,
    pub price_cents: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum OrderParserError {
    #[error("line is empty")]
    EmptyLine,
    #[error("invalid field count: expected 4 fields, got {0}")]
    InvalidFieldCount(usize),
    #[error("not valid id: {0}")]
    InvalidId(String),
    #[error("name is empty")]
    EmptyName,
    #[error("invalid quantity: {0}")]
    InvalidQty(String),
    #[error("invalid price: {0}")]
    InvalidPrice(String),
}

#[derive(thiserror::Error, Debug)]
pub enum OrdersError {
    #[error("line {line}: {source}")]
    LineError {
        line: usize,
        #[source]
        source: OrderParserError,
    },
}

pub fn parse_order_line(line: &str) -> Result<Order, OrderParserError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(OrderParserError::EmptyLine);
    }

    let parts = trimmed.split(';').collect::<Vec<_>>();

    if parts.len() != 4 {
        return Err(OrderParserError::InvalidFieldCount(parts.len()));
    }

    let id_raw = parts[0];
    let id = id_raw
        .trim()
        .parse::<u32>()
        .map_err(|_| OrderParserError::InvalidId(id_raw.to_string()))?;

    if id == 0 {
        return Err(OrderParserError::InvalidId(id_raw.to_string()));
    }

    let name = parts[1].trim().to_string();
    if name.is_empty() {
        return Err(OrderParserError::EmptyName);
    }

    let qty_raw = parts[2];
    let qty = qty_raw
        .trim()
        .parse::<u32>()
        .map_err(|_| OrderParserError::InvalidQty(qty_raw.to_string()))?;
    let price_raw = parts[3];
    let (left, right) = price_raw
        .trim()
        .split_once('.')
        .ok_or_else(|| OrderParserError::InvalidPrice(price_raw.to_string()))?;
    if right.len() != 2 {
        return Err(OrderParserError::InvalidPrice(price_raw.to_string()));
    }

    let euros = left
        .trim()
        .parse::<u32>()
        .map_err(|_| OrderParserError::InvalidPrice(price_raw.to_string()))?;
    let cents = right
        .trim()
        .parse::<u32>()
        .map_err(|_| OrderParserError::InvalidPrice(price_raw.to_string()))?;

    let price_cents = euros * 100 + cents;

    Ok(Order {
        id,
        name,
        qty,
        price_cents,
    })
}

pub fn parse_orders(input: &str) -> Result<Vec<Order>, OrderParserError> {
    let mut vec_orders = Vec::new();
    for line in input.lines() {
        let order = parse_order_line(line)?;
        vec_orders.push(order);
    }
    Ok(vec_orders)
}

pub fn parse_orders_with_line_info(input: &str) -> Result<Vec<Order>, OrdersError> {
    let mut vec_orders = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let order = parse_order_line(line);
        match order {
            Ok(ord) => {
                vec_orders.push(ord);
            }
            Err(source) => {
                return Err(OrdersError::LineError {
                    line: idx + 1,
                    source,
                });
            }
        };
    }
    Ok(vec_orders)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_order_line_happy_path() {
        let line = "1;coffee;10;4.50";
        let result = parse_order_line(line).expect("Should be Order");
        assert_eq!(result.id, 1);
        assert_eq!(result.qty, 10);
        assert_eq!(result.name, "coffee");
        assert_eq!(result.price_cents, 450);
    }

    #[test]
    fn parse_order_line_empty_line() {
        let line = " ";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::EmptyLine)));
    }

    #[test]
    fn parse_order_line_invalid_field_count() {
        let line = "1;coffee;10";
        let result = parse_order_line(line);
        assert!(matches!(
            result,
            Err(OrderParserError::InvalidFieldCount(_))
        ));
    }

    #[test]
    fn parse_order_line_invalid_id() {
        let line = "abc;coffee;10;4.50";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidId(_))));
    }

    #[test]
    fn parse_order_line_empty_name() {
        let line = "1; ;10;4.50";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::EmptyName)));
    }

    #[test]
    fn parse_order_line_invalid_qty() {
        let line = "1;coffee;10abc;4.50";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidQty(_))));
    }

    #[test]
    fn parse_order_line_invalid_price() {
        let line = "1;coffee;10;abc";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_order_line_zero_id() {
        let line = "0;coffee;10;4.50";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidId(_))));
    }

    #[test]
    fn parse_order_line_price_wrong_decimal_places() {
        let line = "1;coffee;10;4.5";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_order_line_price_three_decimal_places() {
        let line = "1;coffee;10;4.500";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_order_line_whitespace_handling() {
        let line = "  1  ;  coffee  ;  10  ;  4.50  ";
        let result = parse_order_line(line).expect("Should parse with whitespace");
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "coffee");
        assert_eq!(result.qty, 10);
        assert_eq!(result.price_cents, 450);
    }

    #[test]
    fn parse_order_line_price_with_one_digit_euros() {
        let line = "1;coffee;10;0.50";
        let result = parse_order_line(line).expect("Should parse");
        assert_eq!(result.price_cents, 50);
    }

    #[test]
    fn parse_order_line_price_with_many_digits() {
        let line = "1;coffee;10;999.99";
        let result = parse_order_line(line).expect("Should parse");
        assert_eq!(result.price_cents, 99999);
    }

    #[test]
    fn parse_order_line_empty_price_left() {
        let line = "1;coffee;10;.50";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_order_line_no_decimal_point() {
        let line = "1;coffee;10;450";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_order_line_invalid_price_cents() {
        let line = "1;coffee;10;4.xy";
        let result = parse_order_line(line);
        assert!(matches!(result, Err(OrderParserError::InvalidPrice(_))));
    }

    #[test]
    fn parse_orders_happy_path() {
        let input = "\
1;coffee;9;5.43
2;milk;6;4.13
3;water;2;1.47
";

        let result = parse_orders(input).expect("Should be Vec<Order>");
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            Order {
                id: 1,
                name: "coffee".to_string(),
                qty: 9,
                price_cents: 543
            }
        );
        assert_eq!(
            result[1],
            Order {
                id: 2,
                name: "milk".to_string(),
                qty: 6,
                price_cents: 413
            }
        );
        assert_eq!(
            result[2],
            Order {
                id: 3,
                name: "water".to_string(),
                qty: 2,
                price_cents: 147
            }
        );
    }

    #[test]
    fn parse_orders_empty_input() {
        let input = "";
        let result = parse_orders(input).expect("Should return empty vec");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn parse_orders_single_order() {
        let input = "1;coffee;10;4.50";
        let result = parse_orders(input).expect("Should parse single order");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "coffee");
    }

    #[test]
    fn parse_orders_with_error() {
        let input = "\
1;coffee;9;5.43
2;milk;invalid;4.13
3;water;2;1.47
";
        let result = parse_orders(input);
        assert!(matches!(result, Err(OrderParserError::InvalidQty(_))));
    }

    #[test]
    fn parse_orders_with_empty_line() {
        let input = "\
1;coffee;9;5.43

2;milk;6;4.13
";
        let result = parse_orders(input);
        assert!(matches!(result, Err(OrderParserError::EmptyLine)));
    }

    #[test]
    fn parse_orders_with_line_info_happy_path() {
        let input = "\
1;coffee;9;5.43
2;milk;6;4.13
3;water;2;1.47
";

        let result = parse_orders_with_line_info(input).expect("Should parse");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[1].id, 2);
        assert_eq!(result[2].id, 3);
    }

    #[test]
    fn parse_orders_with_line_info_empty_input() {
        let input = "";
        let result = parse_orders_with_line_info(input).expect("Should return empty vec");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn parse_orders_with_line_info_error_first_line() {
        let input = "abc;coffee;10;4.50";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 1, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 1);
            assert!(matches!(source, OrderParserError::InvalidId(_)));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_second_line() {
        let input = "\
1;coffee;9;5.43
2;milk;invalid;4.13
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 2, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 2);
            assert!(matches!(source, OrderParserError::InvalidQty(_)));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_third_line() {
        let input = "\
1;coffee;9;5.43
2;milk;6;4.13
3;water;abc;1.47
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 3, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 3);
            assert!(matches!(source, OrderParserError::InvalidQty(_)));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_empty_line() {
        let input = "\
1;coffee;9;5.43

2;milk;6;4.13
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 2, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 2);
            assert!(matches!(source, OrderParserError::EmptyLine));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_invalid_price() {
        let input = "\
1;coffee;9;5.43
2;milk;6;4.5
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 2, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 2);
            assert!(matches!(source, OrderParserError::InvalidPrice(_)));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_empty_name() {
        let input = "\
1;coffee;9;5.43
2; ;6;4.13
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 2, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 2);
            assert!(matches!(source, OrderParserError::EmptyName));
        }
    }

    #[test]
    fn parse_orders_with_line_info_error_invalid_field_count() {
        let input = "\
1;coffee;9;5.43
2;milk;6
";
        let result = parse_orders_with_line_info(input);
        assert!(matches!(result, Err(OrdersError::LineError { line: 2, .. })));
        if let Err(OrdersError::LineError { line, source }) = result {
            assert_eq!(line, 2);
            assert!(matches!(source, OrderParserError::InvalidFieldCount(_)));
        }
    }

    #[test]
    fn parse_order_line_large_values() {
        let line = "4294967295;product;1000;999.99";
        let result = parse_order_line(line).expect("Should handle large values");
        assert_eq!(result.id, 4294967295);
        assert_eq!(result.qty, 1000);
        assert_eq!(result.price_cents, 99999);
    }

    #[test]
    fn parse_order_line_price_zero() {
        let line = "1;coffee;10;0.00";
        let result = parse_order_line(line).expect("Should allow zero price");
        assert_eq!(result.price_cents, 0);
    }

    #[test]
    fn parse_order_line_quantity_zero() {
        let line = "1;coffee;0;4.50";
        let result = parse_order_line(line).expect("Should allow zero quantity");
        assert_eq!(result.qty, 0);
    }

    #[test]
    fn parse_order_line_multiple_orders_with_trailing_newline() {
        let input = "1;coffee;10;4.50\n2;tea;5;3.25\n";
        let result = parse_orders(input).expect("Should handle trailing newline");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn parse_order_line_price_with_leading_zeros() {
        let line = "1;coffee;10;004.50";
        let result = parse_order_line(line).expect("Should handle leading zeros");
        assert_eq!(result.price_cents, 450);
    }
}
