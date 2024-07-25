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
unsafe extern "C" fn expression_entry(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
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

        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_MASTER_INSTANCE_WORK_ID_FLAG_SWORD_OFF_EFFECT_AURA,
        );
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), false);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x24772eddef), true);
    }
}

pub fn install() {
    Agent::new("master")
        .expression_acmd("expression_entryl_switchbe", expression_entry, Default)
        .expression_acmd("expression_entryr_switchbe", expression_entry, Default)
        .install();
}
