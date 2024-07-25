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

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 2) as u64;
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
            if DARK_KNIGHT_EXIST == true {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("edge_aura"),
                    Hash40::new("top"),
                    4,
                    10,
                    0,
                    0,
                    0,
                    0,
                    1.3,
                    true,
                );
            }
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
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            10,
            0,
            0,
            0,
            1.5,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash_s"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            10,
            0,
            0,
            0,
            0.8,
            0,
            0,
            0,
            0,
            0,
            0,
            true,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_appeallwl_soulshift", effect_appeallw, Low)
        .effect_acmd("effect_appeallwr_soulshift", effect_appeallw, Low)
        .install();
}
