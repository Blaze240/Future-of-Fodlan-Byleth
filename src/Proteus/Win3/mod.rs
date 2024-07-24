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

unsafe extern "C" fn effect_win3(agent: &mut L2CAgentBase) {
    let rand = smash::app::sv_math::rand(hash40("agent"), 2) as u64;
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
                Hash40::new("eflame_change_start"),
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
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        if rand == 1 {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1"),
                Hash40::new("tex_marth_sword2"),
                4,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
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
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        } else {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1_dark"),
                Hash40::new("tex_marth_sword2"),
                8,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
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
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_landing_smoke"),
            Hash40::new("top"),
            6,
            0,
            -3,
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
            true,
        );
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        if rand == 1 {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1"),
                Hash40::new("tex_marth_sword2"),
                8,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
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
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        } else {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1_dark"),
                Hash40::new("tex_marth_sword2"),
                8,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
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
                *TRAIL_BLEND_ALPHA,
                101,
                *TRAIL_CULL_NONE,
                1.4,
                0.2,
            );
        }
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_dash_smoke"),
            Hash40::new("top"),
            6.5,
            0,
            -2,
            0,
            -40,
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
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_win3_soulshift", effect_win3, Low)
        .install();
}
