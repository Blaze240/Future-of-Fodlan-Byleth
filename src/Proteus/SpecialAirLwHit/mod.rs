use {
    crate::DARK_KNIGHT_EXIST,
    crate::PALADIN_EXIST,
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};

unsafe extern "C" fn effect_specialairlwhit(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 2);
    if macros::is_excute(agent) {
        if rand == 1 {
            if PALADIN_EXIST == false {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("elight_change_start"),
                    Hash40::new("top"),
                    0,
                    10,
                    0,
                    0,
                    0,
                    0,
                    1.3,
                    true,
                );
            }
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("dark_knight_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("paladin_armor"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("weaponbladem"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("weapongripm"),
                true,
            );
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("sayam"), true);
            PALADIN_EXIST = true;
            DARK_KNIGHT_EXIST = false;
        } else {
            if DARK_KNIGHT_EXIST == false {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("eflame_change_start"),
                    Hash40::new("top"),
                    0,
                    10,
                    0,
                    0,
                    0,
                    0,
                    1.3,
                    true,
                );
            }

            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("dark_knight_armor"),
                true,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("paladin_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("weaponbladem"),
                false,
            );
            ModelModule::set_mesh_visibility(
                agent.module_accessor,
                Hash40::new("weapongripm"),
                false,
            );
            ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("sayam"), false);
            DARK_KNIGHT_EXIST = true;
            PALADIN_EXIST = false;
        }
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_guard_mark"), true, true);
        macros::EFFECT(
            agent,
            Hash40::new("marth_counter_success"),
            Hash40::new("top"),
            0,
            14.8,
            -1,
            0,
            90,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        WorkModule::is_flag(
            agent.module_accessor,
            *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT,
        );
        if macros::is_excute(agent) {
            macros::EFFECT(
                agent,
                Hash40::new("sys_counter_flash"),
                Hash40::new("top"),
                0,
                14.8,
                -1,
                0,
                0,
                0,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                false,
            );
        }
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
        if rand == 1 {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("texmarthswoadtrace"),
                Hash40::new("texmarthswoadtraceadd"),
                10,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.2,
                true,
                Hash40::new("marth_sword_blue"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
                *EFFECT_AXIS_X,
                0,
                *TRAIL_BLEND_BLEND_SRC_ONE,
                1,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        } else {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("texmarthswoadtrace_dark"),
                Hash40::new("texmarthswoadtraceadd"),
                10,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.2,
                true,
                Hash40::new("marth_sword_dark"),
                Hash40::new("haver"),
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0,
                *EFFECT_AXIS_X,
                0,
                *TRAIL_BLEND_BLEND_SRC_ONE,
                1,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        }
        macros::FLASH(agent, 1, 1, 1, 0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd(
            "effect_specialairlwhit_soulshift",
            effect_specialairlwhit,
            Low,
        )
        .install();
}
