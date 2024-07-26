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

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        let rand = smash::app::sv_math::rand(hash40("agent"), 4) as u64;
        if rand == 1 {
            // changes to Timeskip version
            if TIMESKIP_SWITCH == false {
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
                // sets Three Houses eyepatch active
                agent.module_accessor,
                Hash40::new("eyepatch"),
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
                // hides Great Lord armor
                agent.module_accessor,
                Hash40::new("greatlord_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Hopes armor
                agent.module_accessor,
                Hash40::new("hopes_armor"),
                false,
            );
            TIMESKIP_SWITCH = true;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = false;
        } else if rand == 2 {
            // changes to Great Lord version
            if PROMOTION_SWITCH == false {
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
                // sets Three Houses eyepatch active
                agent.module_accessor,
                Hash40::new("eyepatch"),
                true,
            );
            ModelModule::set_mesh_visibility(
                // sets Great Lord armor active
                agent.module_accessor,
                Hash40::new("greatlord_armor"),
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
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = true;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = false;
        } else if rand == 3 {
            // changes to Three Hopes version
            if HOPES_SWITCH == false {
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
                // hides Great Lord armor
                agent.module_accessor,
                Hash40::new("greatlord_armor"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Three Houses eyepatch
                agent.module_accessor,
                Hash40::new("eyepatch"),
                false,
            );
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = true;
            DEFAULT_SWITCH = false;
        } else {
            // changes to default Byleth
            if DEFAULT_SWITCH == false {
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
                // hides Three Houses eyepatch
                agent.module_accessor,
                Hash40::new("eyepatch"),
                false,
            );
            ModelModule::set_mesh_visibility(
                // hides Great Lord armor
                agent.module_accessor,
                Hash40::new("greatlord_armor"),
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
            TIMESKIP_SWITCH = false;
            PROMOTION_SWITCH = false;
            HOPES_SWITCH = false;
            DEFAULT_SWITCH = true;
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(
            agent,
            Hash40::new("sys_action_smoke_v"),
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
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_hold"),
            Hash40::new("haver"),
            0,
            13,
            0.6,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold"), false, true);
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_hold2"),
            Hash40::new("haver"),
            0,
            13,
            0.6,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_hold_end"),
            Hash40::new("haver"),
            0,
            13,
            0.6,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(
            agent,
            Hash40::new("sys_atk_smoke"),
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
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("master_axe_hold2"), false, true);
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_slash_reverb"),
            Hash40::new("top"),
            0,
            5,
            0,
            0,
            0,
            0,
            1,
            true,
        );
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_slash"),
            Hash40::new("top"),
            0,
            5,
            0,
            0,
            0,
            0,
            1,
            false,
        );
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(
            agent,
            Hash40::new("master_axe_slash_particle"),
            Hash40::new("top"),
            0,
            7,
            1,
            0,
            0,
            0,
            1,
            true,
        );
    }
    frame(agent.lua_state_agent, 68.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("master_axe_slash_reverb"), -1);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("master_axe_slash"), false, true);
    }
}
pub fn install() {
    Agent::new("master")
        .effect_acmd("effect_speciallw_switchbl", effect_speciallw, Low)
        .install();
}
