pub mod enemy_circle_intersections;
pub mod player_circle_intersections;
pub mod distill_closest;
pub mod distill_target;
pub mod prelude;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use core::slice::Iter;
use std::{default, marker::*};

use ref_marks::*;
use ref_paths::*;

use self::player_circle_intersections::player_circle_intersections_sys;
use self::enemy_circle_intersections::enemy_circle_intersections_sys;
use self::distill_target::target_detection_distillation_sys;
use self::distill_closest::closest_detection_distillation_sys;

pub struct RTSUnitDetectorsPlugin;

impl Plugin for RTSUnitDetectorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            player_circle_intersections_sys,
            enemy_circle_intersections_sys,
            target_detection_distillation_sys,
            closest_detection_distillation_sys
        ));
    }
}

#[derive(Component)]
/// Signed waymark.
pub struct ToDetector<S: RefSignature>{
    waymark: Entity,
    signature: PhantomData<S>
}
impl<S: RefSignature> Waymark for ToDetector<S> {
    fn go(&self) -> Entity {
        return self.waymark
    }
}

#[derive(Component)]
pub struct TIntersectionsAggregate(pub Vec<Entity>);
impl Default for TIntersectionsAggregate {
    fn default() -> Self {
        Self(Vec::new())
    }
}
impl TIntersectionsAggregate {
    pub fn new() -> Self {
        return Self(Vec::new())
    }
    pub fn iter(&self) -> Iter<Entity> {
        return self.0.iter();
    }
}

pub trait ImmutableDetector{
    const FILTER: QueryFilter<'static>;
    fn shape(&self) -> Collider;
}

/* 
macro_rules! immutable_detector {(...) => {
    ...
};}
*/

pub trait DistillationColumn{
    /// Meant for general use.
    fn read_detection(&self) -> Option<Entity>;

    /// Meant for use with system, that edit a component type, traited with distillation column, by using their local aggregate.
    fn distiller_set(&mut self, v: Option<Entity>);
}

// Unsucessful automation
/* 
// use bevy::ecs::query::{QueryData, QueryFilter, QueryIter};
#[derive(SystemParam)]
/// Preset commands, for systems that distill the aggregate data into single results.
pub struct AggregateDistillation<'w, 's, Column, D, F: QueryFilter = ()>
where 
    Column: DistillationColumn + Component,
    D: QueryData,
{
    q: Query<'w, 's, (&'static mut Column, &'static TIntersectionsAggregate, D), F>,,
} 
impl<'w,'s, Column, D, F> AggregateDistillation<'w,'s, Column, D, F>
where 
    Column: DistillationColumn + Component,
    D: QueryData,
    F: QueryFilter
{
    pub fn iter_mut(&'static mut self) -> QueryIter<'w, 's, (&mut Column, &TIntersectionsAggregate, D), F> {
       return self.q.iter_mut()
    }

    pub fn distill(
        mut column: Mut<'w, Column>,
        aggregate: &TIntersectionsAggregate,
        mut distillation_logic: impl FnMut(Entity) -> Option<Entity> // distillation_logic(distilled, aggregate_data) -> new_distilled
    ) {
        for entity in aggregate.iter() {
            column.distiller_set(distillation_logic(*entity));
        }
    }
}
*/

/// Prefab function for distillation systems.
pub fn distill<'w, Column: DistillationColumn>(
    mut column: Mut<'w, Column>,
    aggregate: &TIntersectionsAggregate,
    mut distillation_logic: impl FnMut(Entity) -> Option<Entity>
) {
    column.distiller_set(None);
    for entity in aggregate.iter() {
        column.distiller_set(distillation_logic(*entity));
    }
}