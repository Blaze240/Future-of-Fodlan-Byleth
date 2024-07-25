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

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if rand == 1 {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_swing"),
                Hash40::new("top"),
                0,
                12,
                -1,
                14,
                -30,
                37,
                1,
                true,
            );
        } else {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_swing_dark"),
                Hash40::new("top"),
                0,
                12,
                -1,
                14,
                -30,
                37,
                1,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        if rand == 1 {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1"),
                Hash40::new("tex_marth_sword2"),
                6,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
                true,
                Hash40::new("null"),
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
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_jump"),
                Hash40::new("top"),
                -0.0,
                0,
                -5,
                0,
                0,
                0,
                1,
                true,
            );
        } else {
            macros::AFTER_IMAGE4_ON_arg29(
                agent,
                Hash40::new("tex_marth_sword1_dark"),
                Hash40::new("tex_marth_sword2"),
                6,
                Hash40::new("sword1"),
                0,
                0,
                0.5,
                Hash40::new("sword1"),
                -0.0,
                -0.0,
                12.6,
                true,
                Hash40::new("null"),
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
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_jump_dark"),
                Hash40::new("top"),
                -0.0,
                0,
                -5,
                0,
                0,
                0,
                1,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_swing"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump_dark"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_swing_dark"), -1);
        if rand == 1 {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_shadow"),
                Hash40::new("top"),
                -0.0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_sword_blue"),
                Hash40::new("haver"),
                -0.0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        } else {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_dolphin_shadow_dark"),
                Hash40::new("top"),
                -0.0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("marth_sword_dark"),
                Hash40::new("haver"),
                -0.0,
                0,
                0,
                0,
                0,
                0,
                1,
                true,
            );
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
            Hash40::new("top"),
            -2,
            -10,
            15,
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
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_blue"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_dark"), false, true);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_specialairhi_soulshift", effect_specialairhi, Low)
        .install();
}
