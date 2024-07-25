use {
    crate::DEFAULT_SWITCH,
    crate::HOPES_SWITCH,
    crate::PROMOTION_SWITCH,
    crate::TIMESKIP_SWITCH,
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

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 4) as u64;
        if rand == 1 {
            // changes to Timeskip version
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
                // sets Three Houses crown active
                agent.module_accessor,
                Hash40::new("crown_houses"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // sets Timeskip armor active
                agent.module_accessor,
                Hash40::new("timeskip_armor"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // hides default Byleth outfit
                agent.module_accessor,
                Hash40::new("byleth_outfit"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Emperor armor
                agent.module_accessor,
                Hash40::new("emperor_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes armor
                agent.module_accessor,
                Hash40::new("hopes_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes crown
                agent.module_accessor,
                Hash40::new("crown_hopes"),
                false,
            );
            TIMESKIP_SWITCH = true;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = false;
        } else if rand == 2 {
            // changes to Emperor version
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
                // sets Three Houses crown active
                agent.module_accessor,
                Hash40::new("crown_houses"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // sets Emperor armor active
                agent.module_accessor,
                Hash40::new("emperor_armor"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // hides default Byleth outfit
                agent.module_accessor,
                Hash40::new("byleth_outfit"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Timeskip armor
                agent.module_accessor,
                Hash40::new("timeskip_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes armor
                agent.module_accessor,
                Hash40::new("hopes_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes crown
                agent.module_accessor,
                Hash40::new("crown_hopes"),
                false,
            );
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = true;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = false;
        } else if rand == 3 {
            // changes to Three Hopes version
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
                // sets Three Hopes crown active
                agent.module_accessor,
                Hash40::new("crown_hopes"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // sets Three Hopes armor active
                agent.module_accessor,
                Hash40::new("hopes_armor"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // hides default Byleth outfit
                agent.module_accessor,
                Hash40::new("byleth_outfit"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Timeskip armor
                agent.module_accessor,
                Hash40::new("timeskip_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Emperor armor
                agent.module_accessor,
                Hash40::new("emperor_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Houses crown
                agent.module_accessor,
                Hash40::new("crown_houses"),
                false,
            );
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = true;
            DEFAULT_SWITCH = false;
        } else {
            // changes to default Byleth
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
                // hides Three Houses crown
                agent.module_accessor,
                Hash40::new("crown_houses"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Emperor armor
                agent.module_accessor,
                Hash40::new("emperor_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // sets default Byleth outfit active
                agent.module_accessor,
                Hash40::new("byleth_outfit"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // hides Timeskip armor
                agent.module_accessor,
                Hash40::new("timeskip_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes armor
                agent.module_accessor,
                Hash40::new("hopes_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes crown
                agent.module_accessor,
                Hash40::new("crown_hopes"),
                false,
            );
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = true;
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_turn_smoke"),
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
            true,
        );
        macros::AFTER_IMAGE4_ON_arg29(
            agent,
            Hash40::new("tex_master_sword1"),
            Hash40::new("tex_master_sword2"),
            6,
            Hash40::new("sword1"),
            1,
            0,
            0,
            Hash40::new("sword1"),
            17.5,
            0,
            0.15,
            true,
            Hash40::new("null"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            -90,
            1,
            0,
            *EFFECT_AXIS_X,
            0,
            *TRAIL_BLEND_ALPHA,
            101,
            *TRAIL_CULL_NONE,
            1.4,
            0.1,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_swordflare"),
            Hash40::new("sword1"),
            0,
            0,
            0,
            0,
            0,
            -90,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("master_swordflare"), false, true);
    }
}

pub fn install() {
    Agent::new("master")
        .effect_acmd("effect_attackhi3_switchsb", effect_attackhi3, Default)
        // Game acmd script
        .install();
}
