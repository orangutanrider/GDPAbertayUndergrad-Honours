use bevy_rapier2d::prelude::CollisionGroups;

use super::groups::*;

// membership indicates what groups the collider is part of.
// filter indicates what groups the collider can interact with.

pub const RTS_PHYSICS_CGROUP: CollisionGroups = CollisionGroups::new(
    RTS_PHYSICS, 
    RTS_PHYSICS
);

// Player team collsion groups
pub const P_CONTROL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SELECTABLE,
    P_SELECTABLE.union(RTS_DETECTION), 
);

pub const P_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SOUL,
    P_SOUL.union(RTS_DETECTION), 
);

// Prince
pub const PRINCE_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    P_SOUL.union(P_PRINCE),
    P_SOUL.union(P_PRINCE).union(RTS_DETECTION), 
);

// Enemy team collision groups
pub const E_SOUL_CGROUP: CollisionGroups = CollisionGroups::new(
    E_SOUL,
    E_SOUL.union(RTS_DETECTION), 
);