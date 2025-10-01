use crate::types::Decimal;

#[derive(Debug, Clone)]
pub struct Pricing {
    wholesale_cost: Decimal,
    list_price: Decimal,
    sales_price: Decimal,
    quantity: i32,
    ext_discount_amount: Decimal,
    ext_sales_price: Decimal,
    ext_wholesale_cost: Decimal,
    ext_list_price: Decimal,
    tax_percent: Decimal,
    ext_tax: Decimal,
    coupon_amount: Decimal,
    ship_cost: Decimal,
    ext_ship_cost: Decimal,
    net_paid: Decimal,
    net_paid_including_tax: Decimal,
    net_paid_including_shipping: Decimal,
    net_paid_including_shipping_and_tax: Decimal,
    net_profit: Decimal,
    refunded_cash: Decimal,
    reversed_charge: Decimal,
    store_credit: Decimal,
    fee: Decimal,
    net_loss: Decimal,
}

impl Pricing {
    pub const QUANTITY_MIN: i32 = 1;

    // Predefined markup and discount minimums
    pub fn markup_min() -> Decimal {
        Decimal::new(0, 2).unwrap()
    }

    pub fn discount_min() -> Decimal {
        Decimal::new(0, 2).unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        wholesale_cost: Decimal,
        list_price: Decimal,
        sales_price: Decimal,
        quantity: i32,
        ext_discount_amount: Decimal,
        ext_sales_price: Decimal,
        ext_wholesale_cost: Decimal,
        ext_list_price: Decimal,
        tax_percent: Decimal,
        ext_tax: Decimal,
        coupon_amount: Decimal,
        ship_cost: Decimal,
        ext_ship_cost: Decimal,
        net_paid: Decimal,
        net_paid_including_tax: Decimal,
        net_paid_including_shipping: Decimal,
        net_paid_including_shipping_and_tax: Decimal,
        net_profit: Decimal,
        refunded_cash: Decimal,
        reversed_charge: Decimal,
        store_credit: Decimal,
        fee: Decimal,
        net_loss: Decimal,
    ) -> Self {
        Pricing {
            wholesale_cost,
            list_price,
            sales_price,
            quantity,
            ext_discount_amount,
            ext_sales_price,
            ext_wholesale_cost,
            ext_list_price,
            tax_percent,
            ext_tax,
            coupon_amount,
            ship_cost,
            ext_ship_cost,
            net_paid,
            net_paid_including_tax,
            net_paid_including_shipping,
            net_paid_including_shipping_and_tax,
            net_profit,
            refunded_cash,
            reversed_charge,
            store_credit,
            fee,
            net_loss,
        }
    }

    // Accessor methods
    pub fn get_wholesale_cost(&self) -> Decimal {
        self.wholesale_cost
    }

    pub fn get_list_price(&self) -> Decimal {
        self.list_price
    }

    pub fn get_sales_price(&self) -> Decimal {
        self.sales_price
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn get_ext_discount_amount(&self) -> Decimal {
        self.ext_discount_amount
    }

    pub fn get_ext_sales_price(&self) -> Decimal {
        self.ext_sales_price
    }

    pub fn get_ext_wholesale_cost(&self) -> Decimal {
        self.ext_wholesale_cost
    }

    pub fn get_ext_list_price(&self) -> Decimal {
        self.ext_list_price
    }

    pub fn get_tax_percent(&self) -> Decimal {
        self.tax_percent
    }

    pub fn get_ext_tax(&self) -> Decimal {
        self.ext_tax
    }

    pub fn get_coupon_amount(&self) -> Decimal {
        self.coupon_amount
    }

    pub fn get_ship_cost(&self) -> Decimal {
        self.ship_cost
    }

    pub fn get_ext_ship_cost(&self) -> Decimal {
        self.ext_ship_cost
    }

    pub fn get_net_paid(&self) -> Decimal {
        self.net_paid
    }

    pub fn get_net_paid_including_tax(&self) -> Decimal {
        self.net_paid_including_tax
    }

    pub fn get_net_paid_including_shipping(&self) -> Decimal {
        self.net_paid_including_shipping
    }

    pub fn get_net_paid_including_shipping_and_tax(&self) -> Decimal {
        self.net_paid_including_shipping_and_tax
    }

    pub fn get_net_profit(&self) -> Decimal {
        self.net_profit
    }

    pub fn get_refunded_cash(&self) -> Decimal {
        self.refunded_cash
    }

    pub fn get_reversed_charge(&self) -> Decimal {
        self.reversed_charge
    }

    pub fn get_store_credit(&self) -> Decimal {
        self.store_credit
    }

    pub fn get_fee(&self) -> Decimal {
        self.fee
    }

    pub fn get_net_loss(&self) -> Decimal {
        self.net_loss
    }
}

// Limits structure for different pricing scenarios
#[derive(Debug, Clone)]
pub struct PricingLimits {
    max_quantity_sold: i32,
    max_markup: Decimal,
    max_discount: Decimal,
    max_wholesale_cost: Decimal,
}

impl PricingLimits {
    pub fn new(
        max_quantity_sold: i32,
        max_markup: Decimal,
        max_discount: Decimal,
        max_wholesale_cost: Decimal,
    ) -> Self {
        PricingLimits {
            max_quantity_sold,
            max_markup,
            max_discount,
            max_wholesale_cost,
        }
    }

    pub fn get_max_quantity_sold(&self) -> i32 {
        self.max_quantity_sold
    }

    pub fn get_max_markup(&self) -> Decimal {
        self.max_markup
    }

    pub fn get_max_discount(&self) -> Decimal {
        self.max_discount
    }

    pub fn get_max_wholesale_cost(&self) -> Decimal {
        self.max_wholesale_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_creation() {
        let pricing = Pricing::new(
            Decimal::new(1000, 2).unwrap(), // wholesale_cost: 10.00
            Decimal::new(1500, 2).unwrap(), // list_price: 15.00
            Decimal::new(1200, 2).unwrap(), // sales_price: 12.00
            5,                              // quantity
            Decimal::new(300, 2).unwrap(),  // ext_discount_amount: 3.00
            Decimal::new(6000, 2).unwrap(), // ext_sales_price: 60.00
            Decimal::new(5000, 2).unwrap(), // ext_wholesale_cost: 50.00
            Decimal::new(7500, 2).unwrap(), // ext_list_price: 75.00
            Decimal::new(8, 2).unwrap(),    // tax_percent: 0.08
            Decimal::new(480, 2).unwrap(),  // ext_tax: 4.80
            Decimal::new(100, 2).unwrap(),  // coupon_amount: 1.00
            Decimal::new(200, 2).unwrap(),  // ship_cost: 2.00
            Decimal::new(1000, 2).unwrap(), // ext_ship_cost: 10.00
            Decimal::new(5900, 2).unwrap(), // net_paid: 59.00
            Decimal::new(6380, 2).unwrap(), // net_paid_including_tax: 63.80
            Decimal::new(6900, 2).unwrap(), // net_paid_including_shipping: 69.00
            Decimal::new(7380, 2).unwrap(), // net_paid_including_shipping_and_tax: 73.80
            Decimal::new(900, 2).unwrap(),  // net_profit: 9.00
            Decimal::ZERO,                  // refunded_cash
            Decimal::ZERO,                  // reversed_charge
            Decimal::ZERO,                  // store_credit
            Decimal::ZERO,                  // fee
            Decimal::ZERO,                  // net_loss
        );

        assert_eq!(pricing.get_quantity(), 5);
        assert_eq!(pricing.get_wholesale_cost().get_number(), 1000);
        assert_eq!(pricing.get_list_price().get_number(), 1500);
    }

    #[test]
    fn test_pricing_limits() {
        let limits = PricingLimits::new(100, Decimal::ONE, Decimal::ONE, Decimal::ONE_HUNDRED);

        assert_eq!(limits.get_max_quantity_sold(), 100);
        assert_eq!(limits.get_max_markup(), Decimal::ONE);
    }

    #[test]
    fn test_constants() {
        assert_eq!(Pricing::QUANTITY_MIN, 1);
        assert_eq!(Pricing::markup_min().get_number(), 0);
        assert_eq!(Pricing::discount_min().get_number(), 0);
    }
}
