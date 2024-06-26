pub use crate::{
    validate_active_terminal_c,
    validate_active_terminal_r,
    ControlPlugin,
    ControlBundle,
    ControlCoreBundle,
    Control,
    ToControl,
};

pub use crate::commandable::{
    Commandable,
    TActiveOrderType,
    ClearOrdersBang,
    OrderProcessedAgar,
    orders::{
        TUnitOrder,
        t_unit_order_clear_sys,
        pure_move::{
            PureMoveOrder,
            TPureMoveOrders,
            processing::PMProximityProcessor
        },
        attack_move::{
            AttackMoveOrder,
            TAttackMoveOrders,
            processing::AMProximityProcessor
        },
        attack_target::{
            AttackTargetOrder,
            TAttackTargetOrders,
            TCurrentTarget,
            processing::{
                //AbortCurrentTargetBang,
                UntilTargetGoneProcessor,
                TargetedBy,
            }
        }
    }
};

pub use crate::commander::SpiralCommander;

pub use crate::selectable::{
    Selected,
    Selectable,
    select,
    un_select_all,
};
