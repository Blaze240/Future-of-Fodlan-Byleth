use {
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

unsafe extern "C" fn effect_appealhi(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 2);
    if macros::is_excute(agent) {
        if rand == 1 {
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
        } else {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("edge_aura"),
                Hash40::new("top"),
                -2,
                -2,
                0,
                80,
                90,
                0,
                1,
                true,
            );
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
            frame(agent.lua_state_agent, 19.0);
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
            frame(agent.lua_state_agent, 24.0);
            if macros::is_excute(agent) {
                macros::EFFECT(
                    agent,
                    Hash40::new("sys_smash_flash"),
                    Hash40::new("sword1"),
                    -0.0,
                    -0.0,
                    9,
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
        }
    }
    frame(agent.lua_state_agent, 19.0);
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
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("sword1"),
            -0.0,
            -0.0,
            9,
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
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_appealhil_soulshift", effect_appealhi, Low)
        .effect_acmd("effect_appealhir_soulshift", effect_appealhi, Low)
        .install();
}
