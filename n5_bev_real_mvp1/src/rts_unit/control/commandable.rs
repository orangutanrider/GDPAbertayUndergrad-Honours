pub mod commander;
pub mod orders;

use bevy::prelude::*;
use orders::*;

#[derive(Component)]
pub struct Commandable {
    orders: Vec<RTSUnitOrder>,
}
impl Default for Commandable {
    fn default() -> Self {
        return Self { orders: Vec::new() }
    }
}
impl Commandable {
    pub fn new() -> Self{
        return Self { orders: Vec::new() }
    }
}

/// Set methods
impl Commandable {
    pub fn clear_orders(&mut self) {      
        self.orders.clear();
    }

    pub fn give_order(&mut self, order: OrderType) {
        self.orders.push(RTSUnitOrder{order_type: order});
    }

    pub fn complete_current_order(&mut self) {
        self.orders.pop();
    }
}

/// Get methods
impl Commandable {
    pub fn current_order(&self) -> Option<RTSUnitOrder> {
        let index = self.orders.len().wrapping_sub(1);
        return self.orders.get(index).copied()
    }

    pub fn number_of_orders(&self) -> usize {
        return self.orders.len()
    }

    pub fn orders_iter(&self) -> core::slice::Iter<'_, RTSUnitOrder> {
        return self.orders.iter()
    }
}