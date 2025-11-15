mod pricing;

use pricing::*;

fn main() {
    let base_price = 100.0;
    let tax_percent = 20.0;

    // 1) Базовый налог
    let with_tax = apply_tax(base_price, tax_percent);
    println!("Base: {base_price}, with tax: {with_tax}");

    // 2) Пример скидки
    let discounted = apply_discount(with_tax, 10.0);
    println!("With 10% discount: {discounted}");

    // 3) Базовая скидка по сумме
    let base_disc = compute_base_discount(with_tax);
    println!("Base discount percent for {with_tax}: {base_disc}%");

    // 4) Финальная цена в разных сценариях
    let regular = final_price(100.0, 20.0, false, false);
    let tax_exempt = final_price(100.0, 20.0, true, false);
    let vip_big_order = final_price(600.0, 20.0, false, true);

    println!("Regular final price: {regular}");
    println!("Tax exempt final price: {tax_exempt}");
    println!("VIP big order final price: {vip_big_order}");

    // 5) Heavy order примеры
    let h1 = is_heavy_order(5, 500.0);
    let h2 = is_heavy_order(12, 1200.0);
    let h3 = is_heavy_order(60, 200.0);

    println!("Heavy (5, 500.0)? {h1}");
    println!("Heavy (12, 1200.0)? {h2}");
    println!("Heavy (60, 200.0)? {h3}");
}







