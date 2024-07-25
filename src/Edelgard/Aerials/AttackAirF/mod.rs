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

unsafe extern "C" fn effect_attackairf(agent: &mut L2CAgentBase) {
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
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(
            agent,
            Hash40::new("master_atk_air_lw_impact"),
            Hash40::new("top"),
            0,
            6.5,
            26,
            0,
            0,
            0,
            1.2,
            true,
        );
        macros::LAST_EFFECT_SET_ALPHA(agent, 1.2);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_spearflare"),
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
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(
                agent,
                Hash40::new("master_atk_air_f"),
                Hash40::new("top"),
                -1,
                9.5,
                4,
                -6.3,
                -33,
                190,
                1.4,
                true,
            );
            macros::LAST_EFFECT_SET_RATE(agent, 1.5);
        } else {
            if macros::is_excute(agent) {
                macros::EFFECT_FOLLOW(
                    agent,
                    Hash40::new("master_atk_air_f"),
                    Hash40::new("top"),
                    -1,
                    10.5,
                    4,
                    -9.5,
                    -33,
                    195,
                    1.4,
                    true,
                );
                macros::LAST_EFFECT_SET_RATE(agent, 1.5);
            }
        }
    }
}

pub fn install() {
    Agent::new("master")
        .effect_acmd("effect_attackairf_switchsb", effect_attackairf, Default)
        .install();
}
