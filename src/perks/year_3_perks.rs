use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, StatHashes},
    enemies::EnemyType,
};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_3_perks() {
    add_mmr(
        Perks::ClownCartridge,
        |_: ModifierResponseInput| -> MagazineModifierResponse {
            MagazineModifierResponse {
                magazine_add: 0.0,
                magazine_scale: 1.5,
                magazine_stat_add: 0,
            }
        },
    );

    add_sbr(
        Perks::ElementalCapacitor,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let ev = if input.is_enhanced { 5 } else { 0 };
            if input.value == 1 {
                stats.insert(StatHashes::STABILITY.into(), 20 + ev);
            } else if input.value == 2 {
                stats.insert(StatHashes::RELOAD.into(), 50 + ev);
            } else if input.value == 3 {
                stats.insert(StatHashes::HANDLING.into(), 50 + ev);
            } else if input.value == 4 {
                stats.insert(StatHashes::RECOIL_DIR.into(), 20 + ev);
            } else if input.value == 5 {
                stats.insert(StatHashes::AIRBORNE.into(), 20 + ev);
            };
            stats
        },
    );

    add_hmr(
        Perks::ElementalCapacitor,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling = 0;
            if input.value == 3 {
                handling = if input.is_enhanced { 55 } else { 50 };
            };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::ElementalCapacitor,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if input.value == 2 {
                reload = if input.is_enhanced { 55 } else { 50 };
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::KillingWind,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::HANDLING.into(), 40);
                stats.insert(StatHashes::RANGE.into(), 20);
            };
            stats
        },
    );

    add_rmr(
        Perks::KillingWind,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            if input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: 20,
                    range_all_scale: 1.05,
                    range_zoom_scale: 1.0,
                    range_hip_scale: 1.0,
                }
            } else {
                RangeModifierResponse {
                    range_stat_add: 0,
                    range_all_scale: 1.0,
                    range_zoom_scale: 1.0,
                    range_hip_scale: 1.0,
                }
            }
        },
    );

    add_hmr(
        Perks::KillingWind,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: 40,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::LastingImpression,
        |_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.0,
                explosive_dmg_scale: 1.25,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::Vorpal,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut buff = 1.0;
            if (*input.calc_data.enemy_type == EnemyType::BOSS
                || *input.calc_data.enemy_type == EnemyType::MINIBOSS
                || *input.calc_data.enemy_type == EnemyType::CHAMPION
                || *input.calc_data.enemy_type == EnemyType::VEHICLE)
                && !input.pvp
            {
                buff = match *input.calc_data.ammo_type {
                    AmmoType::HEAVY => 1.1,
                    AmmoType::SPECIAL => 1.15,
                    AmmoType::PRIMARY => 1.2,
                    AmmoType::UNKNOWN => 0.0, //this should make someone point out a bug? whats error handling lol
                };
            }
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::TrenchBarrel,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut buffer: HashMap<u32, i32> = HashMap::new();
            let bump = if input.is_enhanced { 35 } else { 30 };
            if input.value > 0 {
                buffer.insert(StatHashes::HANDLING.into(), bump);
                //reload unknown
                buffer.insert(StatHashes::RELOAD.into(), bump);
            }
            buffer
        },
    );

    add_hmr(
        Perks::TrenchBarrel,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value == 0 {
                return HandlingModifierResponse::default();
            }
            HandlingModifierResponse {
                stat_add: if input.is_enhanced { 35 } else { 30 },
                ..Default::default()
            }
        },
    );

    //ready for when someone finds the reload information
    add_rsmr(
        Perks::TrenchBarrel,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: if input.is_enhanced { 35 } else { 30 },
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::TrenchBarrel,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.5,
                    explosive_dmg_scale: 1.5,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        },
    );
}
