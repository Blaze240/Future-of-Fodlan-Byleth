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

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
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
        DARK_KNIGHT_EXIST = false;
        PALADIN_EXIST = true;
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_marth_sword1"),
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
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn expression_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ModelModule::set_mesh_visibility(
            agent.module_accessor,
            Hash40::new("dark_knight_armor"),
            false,
        );
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("paladin_armor"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("weaponbladem"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("weapongripm"), true);
        ModelModule::set_mesh_visibility(agent.module_accessor, Hash40::new("sayam"), true);
        AttackModule::set_attack_reference_joint_id(
            agent.module_accessor,
            Hash40::new("haver"),
            AttackDirectionAxis(*ATTACK_DIRECTION_Z),
            AttackDirectionAxis(*ATTACK_DIRECTION_Y),
            AttackDirectionAxis(*ATTACK_DIRECTION_X),
        );
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("marth")
        .effect_acmd("effect_attackairhi_soulshift", effect_attackairhi, Default)
        .expression_acmd(
            "expression_attackairhi_soulshift",
            expression_attackairhi,
            Default,
        )
        .install();
}
