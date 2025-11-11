pub fn apply_tax(base_price: f64, tax_percent: f64) -> f64 {
    let tax = base_price * tax_percent / 100.0;
    base_price + tax
}

pub fn apply_discount(price: f64, discount_percent: f64) -> f64 {
    let discount = price * discount_percent / 100.0;
    let final_price = price - discount;
    if final_price < 0.0 {
        0.0
    } else {
        final_price
    }
}

pub fn compute_base_discount(total: f64) -> f64 {
    if total < 100.0 {
        0.0
    } else if total <= 500.0 {
        5.0
    } else {
        10.0
    }
}

pub fn final_price(
    base_price: f64,
    tax_percent: f64,
    is_tax_exempt: bool,
    is_vip: bool,
) -> f64 {
    if base_price < 0.0 || tax_percent < 0.0 {
        return 0.0;
    }

    let subtotal = if is_tax_exempt {
        base_price
    } else {
        apply_tax(base_price, tax_percent)
    };

    if !is_vip {
        return subtotal;
    }

    let base_discount_percent = compute_base_discount(subtotal);
    let total_discount_percent = base_discount_percent + 5.0;

    apply_discount(subtotal, total_discount_percent)
}

pub fn is_heavy_order(total_items: u32, total_amount: f64) -> bool {
    (total_items > 10 && total_amount > 1000.0)
        || total_items > 50
        || total_amount > 5000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_tax_and_discount() {
        assert_eq!(apply_tax(100.0, 20.0), 120.0);
        assert_eq!(apply_discount(200.0, 10.0), 180.0);
        assert_eq!(apply_discount(50.0, 200.0), 0.0);
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute_base_discount(50.0), 0.0);
        assert_eq!(compute_base_discount(100.0), 5.0);
        assert_eq!(compute_base_discount(500.0), 5.0);
        assert_eq!(compute_base_discount(600.0), 10.0);
    }

    #[test]
    fn final_price_regular() {
        let price = final_price(100.0, 20.0, false, false);
        assert_eq!(price, 100.0_f64 * 1.20);
    }

    #[test]
    fn final_price_tax_exempt() {
        let price = final_price(100.0, 20.0, true, false);
        assert_eq!(price, 100.0);
    }

    #[test]
    fn final_price_vip_big_order() {
        let vip = final_price(600.0, 20.0, false, true);
        let regular = final_price(600.0, 20.0, false, false);
        assert!(vip < regular);
    }

    #[test]
    fn final_price_negative_inputs() {
        let price = final_price(-10.0, 20.0, false, false);
        assert_eq!(price, 0.0);
    }

    #[test]
    fn heavy_order_basic() {
        assert_eq!(is_heavy_order(5, 500.0), false);
        assert_eq!(is_heavy_order(12, 1200.0), true);
        assert_eq!(is_heavy_order(60, 200.0), true);
        assert_eq!(is_heavy_order(1, 6000.0), true);
    }
}

