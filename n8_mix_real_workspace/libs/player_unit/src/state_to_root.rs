use super::*;

#[derive(Component)]
/// (0-3)
/// Idle, Move, Chase, Attack;
/// Mutually exclusive.
pub struct TUnitIMCAMapper(pub u8);
impl Default for TUnitIMCAMapper {
    fn default() -> Self {
        Self(0)
    }
}

pub fn imca_mapper_sys(
    mut q: Query<(&mut TState, &mut TUnitIMCAMapper), Changed<TUnitIMCAMapper>>,
) {
    for (mut state, mapper) in q.iter_mut() {
        match mapper.0 {
            0 => { // Idle
                state.insert(Key::LocalComponent(TypeId::of::<TUnitIMCAMapper>()), IDLE);
            }
            1 => { // Move
                state.insert(Key::LocalComponent(TypeId::of::<TUnitIMCAMapper>()), MOVE);
            },
            2 => { // Chase
                state.insert(Key::LocalComponent(TypeId::of::<TUnitIMCAMapper>()), CHASE);
            },
            3 => { // Attack
                state.insert(Key::LocalComponent(TypeId::of::<TUnitIMCAMapper>()), ATTACK);
            },
            _ => {
                let mut mapper = mapper;
                mapper.0 = 0;
            }
        }
    }
}

#[derive(Component)]
pub struct AggroDetectorClosest(pub Option<Entity>);
impl Default for AggroDetectorClosest {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
pub struct AttackDetectorClosest(pub Option<Entity>);
impl Default for AttackDetectorClosest {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
pub struct AttackDetectorTargeted(pub Option<Entity>);
impl Default for AttackDetectorTargeted {
    fn default() -> Self {
        Self(None)
    }
}

pub fn aggro_to_tree_root_sys(
    aggro_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AggroDetection>>,
    mut root_q: Query<&mut AggroDetectorClosest>,
) {
    for (closest, to_root) in aggro_q.iter() {
        aggro_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn aggro_to_tree_root(
    root_q: &mut Query<&mut AggroDetectorClosest>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot, 
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

pub fn attack_closest_to_tree_root_sys(
    attack_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AttackDetection>>,
    mut root_q: Query<&mut AttackDetectorClosest>,
) {
    for (closest, to_root) in attack_q.iter() {
        attack_closest_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn attack_closest_to_tree_root(
    root_q: &mut Query<&mut AttackDetectorClosest>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot,
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

pub fn attack_target_to_tree_root_sys(
    aggro_q: Query<(&DistillationForClosest, &ToBehaviourRoot), With<AttackDetection>>,
    mut root_q: Query<&mut AttackDetectorTargeted>,
) {
    for (closest, to_root) in aggro_q.iter() {
        attack_target_to_tree_root(&mut root_q, closest, to_root)
    }
}
fn attack_target_to_tree_root(
    root_q: &mut Query<&mut AttackDetectorTargeted>,
    closest: &DistillationForClosest,
    to_root: &ToBehaviourRoot,
) {
    ref_caravan!(to_root::root_q(mut terminal));
    terminal.0 = closest.read_detection();
}

pub trait GenericStateBox {
    const STATE: TreeState;
}

#[derive(Component, Default)]
/// Local transfer.
pub struct ControlOrdersToState;

pub fn control_orders_to_state_sys<OrderTerminalType: 'static, StateBox: GenericStateBox>(
    mut q: Query<(&mut TState, &TActiveOrderType), With<ControlOrdersToState>>,
) {
    for (mut terminal, orders) in q.iter_mut() {
        let Some(order) = orders.current() else {
            terminal.insert(Key::LocalComponent(TypeId::of::<ControlOrdersToState>()), IDLE);
            continue;
        };

        // Validate active terminal
        if order != TypeId::of::<OrderTerminalType>() {
            continue;
        }

        terminal.insert(Key::LocalComponent(TypeId::of::<ControlOrdersToState>()), StateBox::STATE);
    }
}

struct AttackMoveStateBox;
impl GenericStateBox for AttackMoveStateBox {
    const STATE: TreeState = ATTACK_MOVE;
}
struct PureMoveStateBox;
impl GenericStateBox for PureMoveStateBox {
    const STATE: TreeState = PURE_MOVE;
}
struct AttackTargetStateBox;
impl GenericStateBox for AttackTargetStateBox {
    const STATE: TreeState = ATTACK_TARGET;
}

pub struct ControlOrdersToStatePlugin;
impl Plugin for ControlOrdersToStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            control_orders_to_state_sys::<TPureMoveOrders, PureMoveStateBox>,
            control_orders_to_state_sys::<TAttackMoveOrders, AttackMoveStateBox>,
            control_orders_to_state_sys::<TAttackTargetOrders, AttackTargetStateBox>,
        ));
    }
}

#[derive(Component, Default)]
pub struct DetectionToState;

pub fn detection_to_state_sys(
    mut q: Query<(&mut TState, &AggroDetectorClosest, &AttackDetectorClosest, &AttackDetectorTargeted), With<DetectionToState>>,
) {
    for (mut state, aggro_close, attack_close, attack_targeted) in q.iter_mut() {
        let held: TreeState = state.state();
 
        let type_id = TypeId::of::<DetectionToState>();

        if held.contains(ATTACK_TARGET){
            attack_target_detection_to_state(state, attack_targeted, type_id);
            continue;
        }
        else if attack_close.0.is_some() {
            state.insert(Key::LocalComponent(type_id), IN_ATTACK_RANGE);
            continue;
        }
        else if aggro_close.0.is_some() {
            state.insert(Key::LocalComponent(type_id), IN_AGGRO);
            continue;
        }
        else {
            state.insert(Key::LocalComponent(type_id), TreeState::empty());
        }
    }
}

fn attack_target_detection_to_state(
    mut state: Mut<TState>,
    attack_targeted: &AttackDetectorTargeted, // if the target is in the attack range
    type_id: TypeId,
) {
    if attack_targeted.0.is_none() {
        state.insert(Key::LocalComponent(type_id), IN_AGGRO);
        return;
    }

    state.insert(Key::LocalComponent(type_id), IN_ATTACK_RANGE);
}