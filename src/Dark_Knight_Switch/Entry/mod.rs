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
unsafe extern "C" fn expression_entryr(agent: &mut L2CAgentBase) {
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
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn expression_entryl(agent: &mut L2CAgentBase) {
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
    }
    frame(agent.lua_state_agent, 87.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    Agent::new("marth")
        .expression_acmd(
            "expression_entryl_soulshift",
            expression_entryl,
            Default,
        )
        .expression_acmd(
            "expression_entryr_soulshift",
            expression_entryr,
            Default,
        )
        .install();
}
