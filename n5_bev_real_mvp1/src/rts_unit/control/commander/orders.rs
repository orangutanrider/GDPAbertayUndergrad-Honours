/// Data structures for commandable units

use bevy::prelude::*;

pub const ORDER_COMPLETE_DISTANCE: f32 = 1.0;

#[derive(std::fmt::Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
pub enum OrderType {
    PureMovement(PureMovementOrder),
    AttackMove(AttackMoveOrder),
    AttackTarget(AttackTargetOrder),
    Empty,
}

#[derive(Clone, Copy)]
pub struct RTSUnitOrder {
    pub order_type: OrderType,
}
impl Default for RTSUnitOrder {
    fn default() -> Self {
        Self { 
            order_type: OrderType::Empty, 
        }
    }
}
impl RTSUnitOrder {
    pub fn new(order_type: OrderType) -> Self {
        return Self{
            order_type
        }
    }
}

// ATTACK TARGET
#[derive(Clone, Copy)]
pub struct AttackTargetOrder {
    pub invalidated: bool,
    pub target_unit: Entity,
}
impl Default for AttackTargetOrder {
    fn default() -> Self {
        Self {
            invalidated: false, 
            target_unit: Entity::PLACEHOLDER, 
        }
    }
}

// ATTACK MOVE
#[derive(Clone, Copy)]
pub struct AttackMoveOrder {
    pub waypoint: Vec2,
}
impl Default for AttackMoveOrder {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl AttackMoveOrder {
    pub fn check_for_order_complete(&self, position: Vec2) -> bool {
        if self.waypoint.distance(position) <= ORDER_COMPLETE_DISTANCE {
            return true;
        }
        return false;
    }
}

// PURE MOVEMENT
#[derive(Clone, Copy)]
pub struct PureMovementOrder {
    pub waypoint: Vec2,
}
impl Default for PureMovementOrder {
    fn default() -> Self {
        Self { 
            waypoint: Vec2::ZERO,
        }
    }
}
impl PureMovementOrder {
    pub fn check_for_order_complete(&self, position: Vec2) -> bool {
        if self.waypoint.distance(position) <= ORDER_COMPLETE_DISTANCE {
            return true;
        }
        return false;
    }
}