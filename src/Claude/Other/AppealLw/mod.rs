use {
    crate::TIMESKIP_SWITCH,
    crate::DEFAULT_SWITCH,
    crate::PROMOTION_SWITCH,
    crate::HOPES_SWITCH,
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

unsafe extern "C" fn effect_appeallw(agent: &mut L2CAgentBase) {
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
                // hides Barbarossa armor
                agent.module_accessor,
                Hash40::new("barbarossa_armor"),
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
            // changes to Barbarossa version
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
                // sets Barbarossa armor active
                agent.module_accessor,
                Hash40::new("barbarossa_armor"),
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
                // hides Barbarossa armor
                agent.module_accessor,
                Hash40::new("barbarossa_armor"),
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
                // hides Barbarossa armor
                agent.module_accessor,
                Hash40::new("barbarossa_armor"),
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
}

pub fn install() {
    Agent::new("master")
        .effect_acmd("effect_appeallwl_switchgd", effect_appeallw, Low)
        .effect_acmd("effect_appeallwr_switchgd", effect_appeallw, Low)
        .install();
}
