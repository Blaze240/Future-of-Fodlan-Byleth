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

unsafe extern "C" fn effect_specialairlw(agent: &mut L2CAgentBase) {
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
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if rand == 1 {
            macros::EFFECT(
                agent,
                Hash40::new("marth_counter_flash"),
                Hash40::new("top"),
                0,
                11,
                3,
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
                true,
            );
        } else {
            macros::EFFECT(
                agent,
                Hash40::new("marth_counter_flash_dark"),
                Hash40::new("top"),
                0,
                11,
                3,
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
                true,
            );
        }
        macros::FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(agent.lua_state_agent, 1.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            macros::COL_NORMAL(agent);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}
pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_specialairlw_soulshift", effect_specialairlw, Low)
        .install();
}
