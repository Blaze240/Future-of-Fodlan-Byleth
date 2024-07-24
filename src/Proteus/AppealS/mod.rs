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

unsafe extern "C" fn effect_appeals(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 2) as u64;
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
            7,
            0,
            0,
            0,
            0.9,
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
        .effect_acmd("effect_appealsl_soulshift", effect_appeals, Low)
        .effect_acmd("effect_appealsr_soulshift", effect_appeals, Low)
        .install();
}
