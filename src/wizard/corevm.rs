use rustyline::{error::ReadlineError, history::History, Editor, Helper};
use velocity::{endpoints::v::CoreVM, GID};

use crate::wizard::{
    disks::wizard_disks, displays::wizard_displays, nics::wizard_nics, ReadlineExt, YesNo,
};

/// Prompt the user to configure common properties for a virtual machine
/// # Arguments
/// * `readline` - The readline instance to use
/// * `gid` - The group id the virtual machine should be part of
/// * `name` - The name for the virtual machine
pub fn wizard_corevm<'a, H: Helper, I: History>(
    readline: &mut Editor<H, I>,
    gid: GID,
    name: &'a str,
) -> Result<CoreVM<'a>, ReadlineError> {
    let cpus = readline.readline_t("CPU count > ")?;
    let memory_mib = readline.readline_t("Memory in MiB > ")?;

    let displays = wizard_displays(readline)?;
    let disks = wizard_disks(readline)?;
    let nics = wizard_nics(readline)?;

    let autostart: YesNo = readline.readline_t("Autostart (y/n) > ")?;

    let vm = CoreVM {
        name,
        gid,
        cpus,
        memory_mib,
        displays,
        disks,
        nics,
        autostart: autostart.into(),
    };

    println!("VM configuration: {:#?}", vm);
    let ok: YesNo = readline.readline_t("Confirm config (y/n) > ")?;

    match ok {
        YesNo::YES => Ok(vm),
        YesNo::NO => Err(ReadlineError::Interrupted),
    }
}
