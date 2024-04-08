use bevy::prelude::*;
use rts_unit_death::*;
use rts_unit_health::*;

// Ref signature is un-needed for most applications.
// So it is un-implemented here.

#[derive(Component)]
/// Data-destination, reference flag.
pub struct DeathIsLocal;

#[derive(Component)]
/// Data-source, reference flag.
pub struct HealthIsLocal;

#[derive(Component)]
/// Data transformation flag.
pub struct ZeroHealthMeansDeath;

/// Death = ZeroHealthMeansDeath + (DeathIsLocal + HealthIsLocal)
pub fn local_data_zero_health_means_death_sys(
    mut q: Query<(&mut DeathBang, &THealth), (With<ZeroHealthMeansDeath>, With<DeathIsLocal>, With<HealthIsLocal>)>
) {
    for (mut death, health) in q.iter_mut() {
        if health.0 <= 0.0 {
            death.bang();
        }
    }
}