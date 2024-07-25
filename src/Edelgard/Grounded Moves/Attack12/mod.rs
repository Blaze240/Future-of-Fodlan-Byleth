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
unsafe extern "C" fn effect_attack12(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        DARK_KNIGHT_EXIST = true;
        PALADIN_EXIST = false;
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("null"),
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
            Hash40::new(
                "marth_sword_dark
        ",
            ),
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
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn expression_attack12(agent: &mut L2CAgentBase) {
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
            Hash40::new("haver"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_attack12_soulshift", effect_attack12, Default)
        .expression_acmd(
            "expression_attack12_soulshift",
            expression_attack12,
            Default,
        ) // Game acmd script
        .install();
}
