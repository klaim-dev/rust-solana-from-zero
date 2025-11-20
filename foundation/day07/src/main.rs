use day07::parser::*;

fn main() {
    println!("=== Order Parser Demo ===\n");

    // Demo 1: Parse a single order line
    println!("Demo 1: Parse a single order line");
    println!("Input: \"1;coffee;10;4.50\"");
    match parse_order_line("1;coffee;10;4.50") {
        Ok(order) => {
            println!("Success! Parsed order:");
            println!("  ID: {}", order.id);
            println!("  Name: {}", order.name);
            println!("  Quantity: {}", order.qty);
            println!("  Price: {:.2} EUR ({} cents)", 
                     order.price_cents as f32 / 100.0, 
                     order.price_cents);
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Demo 2: Parse multiple orders
    println!("Demo 2: Parse multiple orders");
    let orders_input = "\
1;coffee;9;5.43
2;milk;6;4.13
3;water;2;1.47
4;bread;1;2.99
";
    println!("Input:\n{}", orders_input);
    match parse_orders(orders_input) {
        Ok(orders) => {
            println!("Success! Parsed {} orders:", orders.len());
            for order in &orders {
                println!("  Order #{}: {} x {} ({:.2} EUR each)", 
                         order.id, 
                         order.qty,
                         order.name,
                         order.price_cents as f32 / 100.0);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Demo 3: Parse orders with line info (success case)
    println!("Demo 3: Parse orders with line info (success)");
    let orders_input2 = "\
5;tea;3;3.50
6;juice;2;2.75
";
    println!("Input:\n{}", orders_input2);
    match parse_orders_with_line_info(orders_input2) {
        Ok(orders) => {
            println!("Success! Parsed {} orders with line tracking:", orders.len());
            for order in &orders {
                println!("  Order #{}: {} x {} ({:.2} EUR)", 
                         order.id, 
                         order.qty,
                         order.name,
                         order.price_cents as f32 / 100.0);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Demo 4: Error handling - invalid order line
    println!("Demo 4: Error handling - invalid order line");
    println!("Input: \"abc;coffee;10;4.50\"");
    match parse_order_line("abc;coffee;10;4.50") {
        Ok(order) => println!("Unexpected success: {:?}", order),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 5: Error handling - empty line
    println!("Demo 5: Error handling - empty line");
    println!("Input: \"   \"");
    match parse_order_line("   ") {
        Ok(order) => println!("Unexpected success: {:?}", order),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 6: Error handling - invalid field count
    println!("Demo 6: Error handling - invalid field count");
    println!("Input: \"1;coffee;10\"");
    match parse_order_line("1;coffee;10") {
        Ok(order) => println!("Unexpected success: {:?}", order),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 7: Error handling - zero ID
    println!("Demo 7: Error handling - zero ID");
    println!("Input: \"0;coffee;10;4.50\"");
    match parse_order_line("0;coffee;10;4.50") {
        Ok(order) => println!("Unexpected success: {:?}", order),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 8: Error handling - invalid price format
    println!("Demo 8: Error handling - invalid price format");
    println!("Input: \"1;coffee;10;4.5\" (missing cents)");
    match parse_order_line("1;coffee;10;4.5") {
        Ok(order) => println!("Unexpected success: {:?}", order),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 9: Error handling - parse_orders with error
    println!("Demo 9: Error handling - parse_orders with error");
    let invalid_input = "\
1;coffee;9;5.43
2;milk;invalid;4.13
";
    println!("Input:\n{}", invalid_input);
    match parse_orders(invalid_input) {
        Ok(orders) => println!("Unexpected success: {:?} orders", orders.len()),
        Err(e) => println!("Expected error: {}", e),
    }
    println!();

    // Demo 10: Error handling - parse_orders_with_line_info with error
    println!("Demo 10: Error handling - parse_orders_with_line_info with error");
    let invalid_input2 = "\
1;coffee;9;5.43
2;milk;6;4.13
3;water;abc;1.47
";
    println!("Input:\n{}", invalid_input2);
    match parse_orders_with_line_info(invalid_input2) {
        Ok(orders) => println!("Unexpected success: {:?} orders", orders.len()),
        Err(e) => println!("Expected error with line number: {}", e),
    }
    println!();

    // Demo 11: Edge case - whitespace handling
    println!("Demo 11: Edge case - whitespace handling");
    println!("Input: \"  1  ;  coffee  ;  10  ;  4.50  \"");
    match parse_order_line("  1  ;  coffee  ;  10  ;  4.50  ") {
        Ok(order) => {
            println!("Success! Parsed order (trimmed whitespace):");
            println!("  ID: {}, Name: '{}', Qty: {}, Price: {:.2} EUR", 
                     order.id, order.name, order.qty, order.price_cents as f32 / 100.0);
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("=== Demo Complete ===");
}
