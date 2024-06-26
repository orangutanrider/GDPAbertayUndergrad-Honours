// Defend

// Recent damage increases defend factor
// Many enemies in aggregate detector range increases defend factor

// High defend factor = {
//      more body speed,
//      move away from enemies,
//      more aggressive defence head,
//      more health regen,
//      higher attack speed and damage,
// }

// Low defend factor = {
//      weak long range attacks,
//      reclusive head
// }

// Defence targets = closest

// Defence nav = away from closest
// Defence nav = slow update-rate

use std::any::TypeId;

use bevy::prelude::*;

use rts_unit_detectors::*;
use rts_unit_health::*;
use rts_unit_team::PlayerTeam;

use crate::*;
use super::*;

#[derive(Component, Default)]
pub struct DefendHead;

#[derive(Component)]
pub struct ToDefend(Entity);
waymark!(ToDefend);

#[derive(Bundle, Default)]
pub struct BundDefend {
    pub to_mover: ToMover,
    pub to_hub: ToHub,
    pub flag: DefendHead,

    pub factor: DefendFrenzy,
    pub target: DefendTarget,

    pub mover_in: TMoveAggregator,
    pub mover_process: LocalTransformMovement,
    pub speed: MoveSpeed,
    
    pub attack: DirectAttackBang,
    pub damage: DirectAttackPower,
    pub attack_timer: AttackTimer,
    pub laser: LaserVisualsOnAttack,
}

// Defend factor data collection
#[derive(Component, Default)]
pub struct DefendFrenzy{
    damaged_factor: f32,
    proximity_factor: u32,

    previous_health: f32,
}
impl DefendFrenzy {
    pub fn read(&self) -> f32 {
        return (self.damaged_factor * DEFEND_PAIN_WEIGHT) + ((self.proximity_factor as f32) * DEFEND_SAFE_SPACE_WEIGHT) + 0.001
    }
}

pub fn defend_factor_sys(
    mut q: Query<(&mut DefendFrenzy, &ToHub), With<DefendHead>>,
    hub_q: Query<(&THealth, &TIntersectionsAggregate)>
) {
    for (mut defend, to_hub) in q.iter_mut() {
        let hub = to_hub.go();
        let Ok((health, detection)) = hub_q.get(hub) else {
            continue;
        };

        // Damage to defend factor
        let before_damage = defend.previous_health;
        if before_damage > health.0 {
            let damage = before_damage - health.0;
            defend.damaged_factor = defend.damaged_factor + damage;
            defend.previous_health = health.0;
        } else {
            defend.previous_health = health.0;
        }

        // Detection to defend factor
        let enemies = detection.0.len(); // num of enemies invading the safe space >:(
        defend.proximity_factor = enemies as u32;
    }
}

pub fn defend_factor_damaged_decay(
    mut q: Query<&mut DefendFrenzy, With<DefendHead>>,
    time: Res<Time>,
) {
    for mut defend in q.iter_mut() {
        defend.damaged_factor = (defend.damaged_factor -  time.delta_seconds() * (
            DEFEND_PAIN_DECAY + (DEFEND_PAIN_EXPONENT_DECAY * defend.damaged_factor)))
            .clamp(0.0, f32::MAX);
    }
}

// Defend target
#[derive(Component)]
pub struct DefendTarget{
    target: Entity,
    update_cooldown: f32,
}
impl Default for DefendTarget {
    fn default() -> Self {
        Self{
            target: Entity::PLACEHOLDER,
            update_cooldown: 0.0,
        }
    }
}
impl DefendTarget {
    pub fn read(&self) -> Entity {
        return self.target
    }
}

// Target = closest
pub fn defend_target_selection_sys(
    enemy_q: Query<(Entity, &GlobalTransform), (With<PlayerTeam>, With<THealth>)>,
    mut defend_q: Query<(&mut DefendTarget, &ToHub)>,
    hub_q: Query<&GlobalTransform>,
    time: Res<Time>
) {
    for (mut target, to_hub) in defend_q.iter_mut() {
        target.update_cooldown = target.update_cooldown + time.delta_seconds();
        if target.update_cooldown < DEFEND_TARGET_UPDATE_RATE { continue; }

        // Get
        let Ok(body) = hub_q.get(to_hub.go()) else { continue; }; 
        let body = body.translation().truncate();

        let mut closest = Entity::PLACEHOLDER;
        let mut closest_val = f32::MAX;
        for (enemy, enemy_position) in enemy_q.iter() {
            let enemy_position = enemy_position.translation().truncate();
            
            let distance = body.distance(enemy_position);
            if distance > closest_val { continue; }
            
            closest_val = distance;
            closest = enemy;
        }
        let closest = closest;

        // Set
        target.target = closest;
    }
}

// Head movement
// High defend factor = long neck + fast
// Low defend factor = short neck + avg
pub fn defend_head_movement_sys(
    mut head_q: Query<(&mut TMoveAggregator, &DefendFrenzy, &GlobalTransform, &ToHub, &DefendTarget), With<DefendHead>>,
    q: Query<&GlobalTransform>
) {
    for (mut mover, defend, head_location, to_hub, target) in head_q.iter_mut() {
        // Get
        let hub = to_hub.go();
        let Ok(body) = q.get(hub) else { continue; };
        let body = body.translation().truncate();

        let head = head_location.translation().truncate();

        let body_head_distance = body.distance(head);

        let target = target.read();
        let Ok(target) = q.get(target) else { continue; };
        let target = target.translation().truncate();

        // Calculate body authority
        let body_prevelance = (body_head_distance * DEFEND_BODY_PULL) / 1.0;

        // Calculate head autonomy
        let defend = defend.read();
        let defend_prevelance = (defend * DEFEND_NECK_GROWTH).clamp(DEFEND_NECK_MIN, DEFEND_NECK_MAX);

        // Calculate move vectors
        let to_body_move = (body - head).normalize_or_zero() * DEFEND_BODY_AUTHORITY;
        let to_target_move = (target - head).normalize_or_zero() * DEFEND_HEAD_AUTONOMY;

        // To mover
        use rts_unit_movers::Key as MoveKey;
        mover.inputs.insert(MoveKey::External(hub), (to_body_move, body_prevelance)); // Body
        let local = TypeId::of::<DefendHead>();
        mover.inputs.insert(MoveKey::Local(local), (to_target_move, defend_prevelance)); // Move
    }
}

// Attack
    // Targeted detection or distance check
    // Timer
    // Short range
    // Only attack held target

#[derive(Component, Default)]
pub struct AttackTimer(f32);
impl AttackTimer {
    fn decrement(&mut self, delta: f32) {
        if self.0 <= 0.0 { return; }
        self.0 = self.0 - delta;
    }

    fn increment(&mut self, delta: f32) {
        self.0 = self.0 + delta;
    }

    fn reset(&mut self) {
        self.0 = 0.0;
    }

    fn read(&self) -> f32 {
        return self.0
    }
}

pub fn defend_attack_sys(
    mut q: Query<(&mut DirectAttackBang, &mut AttackTimer, &DefendTarget, &GlobalTransform, &DefendFrenzy), With<DefendHead>>,
    target_q: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (mut attack, mut timer, target, head_position, defend) in q.iter_mut() {
        let target = target.target;
        let Ok(target_at) = target_q.get(target) else { continue; };
        let target_at = target_at.translation().truncate();
        
        let head_position = head_position.translation().truncate();

        let distance = head_position.distance(target_at);

        let defend = defend.read();

        let range_equation = { DEFEND_ATTACK_RANGE *
            ((1.0 / (defend * DEFEND_FRENZY_RANGE_DECREASE))
            .clamp(DEFEND_MIN_ATTACK_RANGE, DEFEND_MAX_ATTACK_RANGE))
        };

        if distance > range_equation { 
            timer.decrement(time.delta_seconds());
            continue; 
        }

        timer.increment(time.delta_seconds());
        if timer.read() < (
            ((DEFEND_ATTACK_SPEED / defend * DEFEND_FRENZY_ATTACK_SPEED)
            .clamp(DEFEND_MAX_ATTACK_SPEED, DEFEND_MIN_ATTACK_SPEED)
        )) { continue; }

        attack.bang(target);
        timer.reset();
    }
}

pub fn defend_frenzy_to_colour(
    mut q: Query<(&mut Sprite, &DefendFrenzy)>
) {
    let min = DEFEND_FRENZY_TO_COLOUR_MIN_MAX.x;
    let max = DEFEND_FRENZY_TO_COLOUR_MIN_MAX.y;
    for (mut sprite, frenzy) in q.iter_mut() {
        let current = frenzy.read();
        
        let t = (current + min) / max;
        let colour_min: Vec3 = DEFEND_COLOUR.hsl_to_vec3();
        let colour_max = DEFEND_FRENZY_COLOUR.hsl_to_vec3();
        let colour = Vec3::lerp(colour_min, colour_max, t);
        
        sprite.color = Color::hsl(colour.x, colour.y, colour.z);
    }
}

#[derive(Component)]
pub struct DefendNeck{
    pub hub: Entity,
    pub defend: Entity,
}

pub fn defend_neck_sys(
    mut q: Query<(&mut Transform, &DefendNeck)>,
    transform_q: Query<&GlobalTransform>,
) {
    for (mut transform, neck) in q.iter_mut() {
        let origin = neck.hub;
        let Ok(origin) = transform_q.get(origin) else {continue;};
        let origin = origin.translation().truncate();

        let target = neck.defend;
        let Ok(target) = transform_q.get(target) else {continue;};
        let target = target.translation().truncate();

        let distance = origin.distance(target);
        let diff = target - origin;
        let direction = diff.normalize();
    
        let translation = (origin + (direction * distance * 0.5)).extend(-0.5);
        let rotation = Quat::from_rotation_z(direction.to_angle());
    
        let scale = Vec3::new(distance, NECK_WIDTH, 0.1);

        transform.scale = scale;
        transform.translation = translation;
        transform.rotation = rotation;
    }
}

pub fn defend_to_body_movement_sys(
    defend_q: Query<(&ToMover, &DefendTarget, &DefendFrenzy, &GlobalTransform, Entity), With<DefendHead>>,
    target_q: Query<&GlobalTransform>,
    mut root_q: Query<&mut TMoveDecider>,
) {
    for (to_mover, target, defend, head, defend_entity) in defend_q.iter() {
        // Get
        let head = head.translation().truncate();

        let target = target.read();
        let Ok(target) = target_q.get(target) else { continue; };
        let target = target.translation().truncate();

        let defend = defend.read();

        let defend_move = (target - head).normalize_or_zero();
        let defend_move = defend_move * ((defend * DEFEND_HEAD_PULL) + DEFEND_BODY_MOVE_BASE_SPEED);
        let defend_move = defend_move.clamp_length(0.0, DEFEND_MOVE_LIMIT);

        let defend_prevelance = (defend * DEFEND_FRENZY_DOMINANCE) + DEFEND_BASE_DOMINANCE; // Move decision prevelance

        // Set
        let hub = to_mover.go();
        let Ok(mut body) = root_q.get_mut(hub) else { continue; };

        use rts_unit_movers::Key as MoveKey;
        body.inputs.insert(MoveKey::External(defend_entity), (defend_move, defend_prevelance));
    }
}