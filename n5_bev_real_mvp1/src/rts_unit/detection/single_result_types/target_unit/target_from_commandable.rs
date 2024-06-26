use bevy::prelude::*;

use crate::rts_unit::{
    *,
    soul::RTSUnitSoul,
    control::{
        parts::*,
        RootToControl,
} };

use super::TargetUnitDetection;

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            target_from_commandable,
        ));
    }
}

#[derive(Component)]
pub struct TargetFromCommandable;
impl ReferenceFlag<(RootToControl, Commandable), Commandable> for TargetFromCommandable {
    fn e (p: (RootToControl, Commandable), o: Commandable) {
        p.0.
    }
}

fn target_from_commandable(
    mut detector_q: Query<(&mut TargetUnitDetection, &ToRoot), With<TargetFromCommandable>>,
    root_q: Query<&RootToControl>,
    control_q: Query<&Commandable>,
) {
    for (mut detection, to_root) in detector_q.iter_mut() {
        let root = to_root.entity();
        let to_control = root_q.get(root);
        let to_control = to_control.unwrap();
        let control = to_control.entity();
        let commandable = control_q.get(control);
        let commandable = commandable.unwrap();

        let mut target: Option<RTSUnitSoul> = None;
        
        let order = commandable.current_order();
        if order.is_none() {
            detection.set_target(target);
            return
        }
        let order = order.unwrap();

        match order.order_type {
            OrderType::PureMovement(_) => {},
            OrderType::AttackMove(_) => {},
            OrderType::Empty => {},
            OrderType::AttackTarget(attack_target) => {
                target = attack_target.target;
            },
        }

        detection.set_target(target)
    }
}