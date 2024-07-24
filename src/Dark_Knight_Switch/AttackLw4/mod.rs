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

unsafe extern "C" fn effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT(
            agent,
            Hash40::new("sys_smash_flash"),
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
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_marth_sword1_dark"),
            Hash40::new("tex_marth_sword2"),
            10,
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
            Hash40::new("marth_sword_dark"),
            Hash40::new("haver"),
            0,
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
            Hash40::new("marth_sword_light"),
            Hash40::new("sword1"),
            0,
            0,
            12.6,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            -3,
            0,
            0,
            0,
            0,
            0,
            0.6,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_dark"), false, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
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
            Hash40::new("marth_sword_dark"),
            Hash40::new("haver"),
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("marth_sword_light"),
            Hash40::new("sword1"),
            0,
            0,
            11,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
            Hash40::new("top"),
            0,
            0,
            0,
            0,
            180,
            0,
            0.5,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_light"), false, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("marth_sword_dark"), false, true);
    }
}

unsafe extern "C" fn expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
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
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("weaponbladem"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("weapongripm"), false);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("sayam"), false);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    frame(agent.lua_state_agent, 4.0);
    execute(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("haver"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
        slope!(
            agent,
            *MA_MSC_CMD_SLOPE_SLOPE_INTP,
            *SLOPE_STATUS_TOP,
            2,
            true
        );
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            14,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashll"), 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 12);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_attacklw4_soulshift", effect_attacklw4, Default)
        .expression_acmd(
            "expression_attacklw4_soulshift",
            expression_attacklw4,
            Default,
        ) // Game acmd script
        .install();
}
