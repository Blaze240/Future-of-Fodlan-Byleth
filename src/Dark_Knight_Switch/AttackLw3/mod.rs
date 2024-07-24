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
unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
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
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_turn_smoke"),
            Hash40::new("top"),
            3,
            0,
            0,
            0,
            0,
            0,
            1.2,
            0,
            0,
            0,
            0,
            0,
            0,
            false,
        );
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
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
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("sword1"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 12);
    }
}

pub fn install() {
    Agent::new("marth")
    .effect_acmd("effect_attacklw3_soulshift", effect_attacklw3, Default)
        .expression_acmd(
            "expression_attacklw3_soulshift",
            expression_attacklw3,
            Default,
        ) // Game acmd script
        .install();
}
