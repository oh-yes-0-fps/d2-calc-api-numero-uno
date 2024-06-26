//This also includes intrinsic perks, not just exotic
use std::collections::HashMap;

use serde::__private::de;

use crate::{d2_enums::StatHashes, enemies::EnemyType, weapons::Stat};

use super::{
    add_dmr, add_edr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rr, add_rsmr, add_sbr,
    add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn exotic_perks() {
    add_dmr(
        Perks::ParacausalShot,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let bufflist_pve = vec![1.0, 3.92, 4.0, 4.4, 5.25, 7.67, 11.71, 18.36];
            let bufflist_pvp = vec![1.0, 1.01, 1.03, 1.13, 1.41, 1.96, 3.0, 4.73];
            let mut damage_buff = 1.0;
            if input.calc_data.curr_mag == 1.0 {
                let num_of_crits = clamp(input.calc_data.shots_fired_this_mag as i32, 0, 7);
                let bufflist = if input.pvp {
                    &bufflist_pvp
                } else {
                    &bufflist_pve
                };
                damage_buff = bufflist[num_of_crits as usize];
            };
            if input.calc_data.time_this_mag < 0.0 {
                let num_of_crits = clamp(input.value as i32, 0, 7);
                let bufflist = if input.pvp {
                    &bufflist_pvp
                } else {
                    &bufflist_pve
                };
                damage_buff = bufflist[num_of_crits as usize];
            }
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::HuntersTrance,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            let inter_val = *input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            out.insert(StatHashes::RELOAD.into(), buff_val);
            out.insert(StatHashes::RANGE.into(), buff_val);
            out.insert(StatHashes::HANDLING.into(), buff_val);
            out
        },
    );

    add_rsmr(
        Perks::HuntersTrance,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let inter_val = *input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            ReloadModifierResponse {
                reload_stat_add: buff_val,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::HuntersTrance,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let inter_val = *input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            RangeModifierResponse {
                range_stat_add: buff_val,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::HuntersTrance,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let inter_val = *input.calc_data.perk_value_map.get(&213689231).unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            HandlingModifierResponse {
                stat_add: buff_val,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::HuntersTrace,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_ads_scale = if input.value > 0 { 4.5 / 1.7 } else { 1.0 };
            RangeModifierResponse {
                range_zoom_scale: range_ads_scale,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::MementoMori,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if input.value > 0 && input.calc_data.total_shots_fired < 7.0 {
                damage_buff = if input.pvp { 1.285 } else { 1.5 };
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                crit_scale: 1.0,
            }
        },
    );

    add_rmr(
        Perks::MementoMori,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_all_scale = if input.value > 0 && input.calc_data.total_shots_fired < 7.0 {
                0.85
            } else {
                1.0
            };
            RangeModifierResponse {
                range_all_scale,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::Roadborn,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 20);
                out.insert(StatHashes::RELOAD.into(), 40);
            };
            out
        },
    );

    add_dmr(
        Perks::Roadborn,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut crit_mult = 1.0;
            if input.value > 0 {
                crit_mult = 1.17;
            };
            DamageModifierResponse {
                crit_scale: crit_mult,
                explosive_dmg_scale: 1.0,
                impact_dmg_scale: 1.0,
            }
        },
    );

    add_fmr(
        Perks::Roadborn,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if input.value > 0 {
                delay_mult = 0.583;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        },
    );

    add_rmr(
        Perks::Roadborn,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range_scale = 1.05;
            if input.value > 0 {
                range_scale = 1.15; //roughly
            };
            RangeModifierResponse {
                range_stat_add: 0,
                range_all_scale: range_scale,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        },
    );

    add_rsmr(
        Perks::Roadborn,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if input.value > 0 {
                reload = 40;
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: 1.0,
            }
        },
    );

    add_fmr(
        Perks::ReignHavoc,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if input.calc_data.shots_fired_this_mag >= input.calc_data.base_mag * 0.2 {
                delay_mult = 0.75;
            };
            if input.calc_data.shots_fired_this_mag >= input.calc_data.base_mag * 0.4 {
                delay_mult = 0.625;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        },
    );

    add_edr(
        Perks::ReignHavoc,
        |input: ModifierResponseInput| -> ExtraDamageResponse {
            let dmg = if input.pvp { 65.0 } else { 65.0 * 1.3 };
            ExtraDamageResponse {
                additive_damage: dmg,
                increment_total_time: false,
                times_to_hit: 1,
                time_for_additive_damage: 0.0,
                hit_at_same_time: true,
                is_dot: false,
                weapon_scale: true,
                crit_scale: false,
                combatant_scale: true,
            }
        },
    );

    add_dmr(
        Perks::WormsHunger,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(input.value, 0, 20);
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + (val as f64) * 0.1,
                explosive_dmg_scale: 1.0 + (val as f64) * 0.1,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::LagragianSight,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if input.value > 0 && input.calc_data.time_total < 30.0 {
                damage_buff = 1.4;
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::ToM,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if input.calc_data.curr_mag == 1.0 {
                damage_buff = if input.pvp { 2.0 } else { 2.4 };
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                crit_scale: 1.0,
            }
        },
    );

    add_rr(
        Perks::ToM,
        |input: ModifierResponseInput| -> RefundResponse {
            RefundResponse {
                refund_mag: if input.calc_data.curr_mag == 0.0 {
                    1
                } else {
                    0
                },
                refund_reserves: 0,
                crit: false,
                requirement: 1,
            }
        },
    );

    add_edr(
        Perks::RocketTracers,
        |input: ModifierResponseInput| -> ExtraDamageResponse {
            let dmg = if input.pvp { 24.0 } else { 105.0 };
            ExtraDamageResponse {
                additive_damage: dmg,
                times_to_hit: 1,
                increment_total_time: false,
                time_for_additive_damage: 0.0,
                hit_at_same_time: true,
                is_dot: false,
                weapon_scale: true,
                crit_scale: false,
                combatant_scale: true,
            }
        },
    );

    add_fmr(
        Perks::HakkeHeavyBurst,
        |_: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_size_add: -2.0,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::HakkeHeavyBurst,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let crit_scale = (1.5 + 5.0 / 51.0) / input.calc_data.base_crit_mult;
            DamageModifierResponse {
                explosive_dmg_scale: 1.48,
                impact_dmg_scale: 1.48,
                crit_scale,
            }
        },
    );

    add_dmr(
        Perks::SwoopingTalons,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg_mult = 1.0;
            if input.value > 0 {
                dmg_mult = 1.4;
            }
            dmg_mult += input.calc_data.total_shots_fired * 0.04;
            dmg_mult = clamp(dmg_mult, 1.0, 1.4);
            DamageModifierResponse {
                impact_dmg_scale: dmg_mult,
                explosive_dmg_scale: dmg_mult,
                crit_scale: 1.0,
            }
        },
    );
    add_dmr(
        Perks::IgnitionTrigger,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg_mult = 1.0;
            if input.value > 0 || input.calc_data.total_shots_fired > 20.0 {
                dmg_mult = if input.pvp { 1.55 } else { 1.99 };
            }
            DamageModifierResponse {
                impact_dmg_scale: dmg_mult,
                explosive_dmg_scale: dmg_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::CalculatedBalance,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if input.value > 0 { 0.2 } else { 0.0 };
            let duration = 5.0;
            if input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_fmr(
        Perks::RavenousBeast,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value > 0 {
                FiringModifierResponse {
                    burst_delay_scale: 0.8,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::RavenousBeast,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if input.value > 0 {
                damage_mult = if input.pvp { 2.2 } else { 2.87 };
                crit_mult = if input.pvp {
                    1.0 / (1.5 + -3.0 / 51.0)
                } else {
                    1.99 / 2.87
                };
            }
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: crit_mult,
            }
        },
    );

    add_sbr(
        Perks::ReleaseTheWolves,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let has_cat = input.calc_data.perk_value_map.contains_key(&431220296);
            let mut out = HashMap::new();
            if has_cat {
                if input.value == 0 {
                    out.insert(StatHashes::STABILITY.into(), 40);
                } else if input.value == 1 {
                    out.insert(StatHashes::RELOAD.into(), 100);
                }
            }
            out
        },
    );

    add_rsmr(
        Perks::ReleaseTheWolves,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let has_cat = input.calc_data.perk_value_map.contains_key(&431220296);
            if input.value == 1 && has_cat {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 0.85,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_fmr(
        Perks::ReleaseTheWolves,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value > 0 {
                FiringModifierResponse {
                    burst_delay_scale: 0.4,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::ReleaseTheWolves,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if input.value > 0 { 1.4 } else { 1.0 };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::Fundamentals,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value == 1 {
                stats.insert(StatHashes::STABILITY.into(), 20);
                stats.insert(StatHashes::AIM_ASSIST.into(), 10);
            } else if input.value == 2 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
                stats.insert(StatHashes::RELOAD.into(), 35);
            } else if input.value == 3 {
                stats.insert(StatHashes::RANGE.into(), 5);
                stats.insert(StatHashes::HANDLING.into(), 25);
            };
            stats
        },
    );

    add_hmr(
        Perks::Fundamentals,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling = 0;
            if input.value == 3 {
                handling = 25;
            }
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::Fundamentals,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if input.value == 2 {
                reload = 35;
            }
            ReloadModifierResponse {
                reload_stat_add: reload,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::Fundamentals,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range = 0;
            if input.value == 3 {
                range = 5;
            }
            RangeModifierResponse {
                range_stat_add: range,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::ThinTheHerd,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::RELOAD.into(), 70);
            }
            out
        },
    );

    add_rsmr(
        Perks::ThinTheHerd,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 70,
                    ..Default::default()
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_hmr(
        Perks::Chimera,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: 100,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::Chimera,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::RELOAD.into(), 100);
            }
            out
        },
    );

    add_dmr(
        Perks::FirstGlance,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if input.value > 0 {
                if input.calc_data.total_shots_fired == 0.0 {
                    damage_mult = 1.33;
                } else {
                    crit_mult = 1.33;
                };
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: crit_mult,
            }
        },
    );

    add_dmr(
        Perks::FateOfAllFools,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if input.value as f64 > input.calc_data.total_shots_fired {
                let cc = input.calc_data.base_crit_mult;
                damage_mult = cc;
                crit_mult = 1.0 / cc;
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: crit_mult,
            }
        },
    );

    add_dmr(
        Perks::HonedEdge,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let has_cat = input.calc_data.perk_value_map.contains_key(&529188544);
            if input.value == 2 {
                damage_mult = if input.pvp { 1.183 } else { 2.0 };
            } else if input.value == 3 {
                damage_mult = if input.pvp { 1.412 } else { 3.0 };
            } else if input.value == 4 && has_cat {
                damage_mult = if input.pvp { 1.504 * 1.2 } else { 4.0 * 1.2 };
            } else if input.value == 4 {
                damage_mult = if input.pvp { 1.504 } else { 4.0 };
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::TakenPredator,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            if input.value == 1 || input.value == 2 {
                damage_mult = 1.25;
            } else if input.value == 3 {
                damage_mult = 1.25 * 1.25;
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::MarkovChain,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(input.value, 0, 5);
            let damage_mult = (1.0 / 15.0) * val as f64 * if input.pvp { 1.0 } else { 2.0 };
            DamageModifierResponse {
                explosive_dmg_scale: 1.0 + damage_mult,
                impact_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::StringofCurses,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(input.value, 0, 5);
            let mut damage_mult = 0.2 * val as f64;
            if input.pvp {
                damage_mult = ((damage_mult * 100.0 / 2.0) / 4.0).ceil() * 0.04;
            }
            let duration = 3.5;
            if input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::StormAndStress,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }

            let damage_mult = if input.pvp { 1.8 } else { 3.62 };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::DualSpeedReceiver,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            if input.value == 0 {
                return RangeModifierResponse::default();
            }
            RangeModifierResponse {
                range_stat_add: 30,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::DualSpeedReceiver,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::ZOOM.into(), 3);
                out.insert(StatHashes::RANGE.into(), 30);
            }
            out
        },
    );

    add_dmr(
        Perks::FullStop,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                explosive_dmg_scale: 1.0,
                impact_dmg_scale: 1.0,
                crit_scale: if !input.pvp { 2.9 } else { 1.0 },
            }
        },
    );

    add_fmr(
        Perks::RatPack,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value == 0 {
                return FiringModifierResponse::default();
            }
            let val = clamp(input.value - 1, 0, 4);

            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.625 / 30.0),
                ..Default::default()
            }
        },
    );

    add_mmr(
        Perks::RatPack,
        |input: ModifierResponseInput| -> MagazineModifierResponse {
            let val = clamp(input.value - 1, 0, 4);
            MagazineModifierResponse {
                magazine_add: val as f64 * if val == 4 { 2.25 } else { 2.0 },
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::RideTheBull,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let extra_value = input.calc_data.shots_fired_this_mag / 10.0;
            let val = clamp(input.value + extra_value as u32, 0, 2);
            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.25 / 30.0),
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::SpinningUp,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let extra_value = input.calc_data.shots_fired_this_mag / 12.0;
            let val = clamp(input.value + extra_value as u32, 0, 2);
            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.5 / 30.0),
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::CranialSpike,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            let val = clamp(input.value, 0, 5) as i32;
            out.insert(StatHashes::RANGE.into(), 8 * val);
            out.insert(StatHashes::AIM_ASSIST.into(), 4 * val);
            out
        },
    );

    add_rsmr(
        Perks::CranialSpike,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(input.value, 0, 5) as i32;
            let rel = 0.97_f64.powi(val);
            ReloadModifierResponse {
                reload_time_scale: rel,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::CranialSpike,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let val = clamp(input.value, 0, 5) as i32;
            RangeModifierResponse {
                range_stat_add: 8 * val,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::DarkForgedTrigger,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value == 0 {
                return FiringModifierResponse::default();
            }
            if input
                .calc_data
                .perk_value_map
                .get(&1319823571)
                .unwrap_or(&0)
                > &4
            {
                FiringModifierResponse {
                    burst_delay_add: -5.0 / 30.0,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse {
                    burst_delay_add: -1.0 / 30.0,
                    ..Default::default()
                }
            }
        },
    );

    add_dmr(
        Perks::HarmonicLaser,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (input.value, input.pvp) {
                (0, _) => 1.0,
                (1, true) => 1.03,
                (1, false) => 1.323,
                (2.., true) => 1.0625,
                (2.., false) => 1.687,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::AgersScepterCatalyst,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.8,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        },
    );

    add_mmr(
        Perks::AgersScepterCatalyst,
        |input: ModifierResponseInput| -> MagazineModifierResponse {
            let mag_buff = if input.value > 0 && input.calc_data.total_shots_fired == 0.0 {
                2.0
            } else {
                1.0
            };
            MagazineModifierResponse {
                magazine_scale: mag_buff,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::ColdFusion,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = 0.0195 * clamp(input.calc_data.total_shots_hit, 0.0, 41.0);
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + buff,
                ..Default::default()
            }
        },
    );

    //Queenbreaker's sights
    add_dmr(
        Perks::MarksmanSights,
        |_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.38,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::MarksmanSights,
        |_: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: (1800.0 / (60000.0 / 333.0)), // 300 + 333 = 633 ,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Broadside,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match input.value {
                0 => 1.0,
                1 => 1.18,
                2 => 1.39,
                3 => 1.59,
                4.. => 1.81,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::TemporalUnlimiter,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value > 0 {
                return FiringModifierResponse {
                    burst_delay_add: 0.366,
                    ..Default::default()
                };
            }
            FiringModifierResponse::default()
        },
    );

    add_dmr(
        Perks::TemporalUnlimiter,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            let mut buff = if input.pvp { 7.545 } else { 14.0 };
            //season 23
            //https://www.bungie.net/7/en/News/Article/season-23-weapons-preview
            if *input.calc_data.enemy_type == EnemyType::CHAMPION {
                buff *= 2.0;
            }
            DamageModifierResponse {
                impact_dmg_scale: buff,
                crit_scale: 1.875,
                ..Default::default()
            }
        },
    );

    add_mmr(
        Perks::FourthHorsemanCatalyst,
        |_: ModifierResponseInput| -> MagazineModifierResponse {
            MagazineModifierResponse {
                magazine_add: 1.0,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::BlackHole,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = if input.calc_data.total_shots_hit % 2.0 == 1.0 {
                1.35
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Impetus,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.5,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        },
    );

    add_fmr(
        Perks::MarksmanSights,
        |_: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: 0.333, // 300 + 333 = 633 ,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Broadhead,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let broadhead_damage = if input.pvp { 30.0 } else { 60.0 };
            let impact_damage = input.calc_data.curr_firing_data.damage;
            let crit_mult = input.calc_data.curr_firing_data.crit_mult;

            let impact_dmg_scale = (broadhead_damage + impact_damage) / impact_damage;

            let crit_scale = (impact_damage * crit_mult + broadhead_damage)
                / (impact_damage * impact_dmg_scale * crit_mult);

            DamageModifierResponse {
                impact_dmg_scale,
                crit_scale,
                ..Default::default()
            }
        },
    );
    add_fmr(
        Perks::Desperation,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let duration = 7.0;
            if input.value == 0 || input.calc_data.time_total > duration {
                return FiringModifierResponse::default();
            }
            FiringModifierResponse {
                burst_delay_scale: 0.8,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::IonicReturn,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            let current_crit = input.calc_data.curr_firing_data.crit_mult;
            let crit_scale = (current_crit + (34.0 / 51.0)) / current_crit;
            DamageModifierResponse {
                impact_dmg_scale: 1.15,
                crit_scale,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::Unrepentant,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 || input.pvp {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 3.0,
                ..Default::default()
            }
        },
    );
    add_fmr(
        Perks::Unrepentant,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let shots_in_super_burst: f64 = 6.0;
            if input.calc_data.total_shots_hit >= shots_in_super_burst || input.value == 0 {
                return FiringModifierResponse::default();
            }
            FiringModifierResponse {
                burst_size_add: 3.0,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::ArcConductor,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.1,
                explosive_dmg_scale: 1.1,
                ..Default::default()
            }
        },
    );
    add_hmr(
        Perks::ArcConductor,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value == 0 {
                return HandlingModifierResponse::default();
            }
            HandlingModifierResponse {
                stat_add: 100,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::ArcConductor,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value == 1 {
                stats.insert(StatHashes::HANDLING.into(), 100);
            }
            stats
        },
    );
    add_dmr(
        Perks::VoidLeech,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 || input.pvp {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.2,
                explosive_dmg_scale: 1.2,
                ..Default::default()
            }
        },
    );
}
