use rustyline::{error::ReadlineError, history::History, Editor, Helper};
use velocity::endpoints::v::DiskConfig;

use super::{ReadlineExt, YesNo};

/// Prompt the user to configure disks for a virtual machine
/// # Arguments
/// * `readline` - The readline instance to use
pub fn wizard_disks<'a, H: Helper, I: History>(
    readline: &mut Editor<H, I>,
) -> Result<Vec<DiskConfig>, ReadlineError> {
    let mut res = Vec::new();

    loop {
        match readline.readline_t::<YesNo>("Add another disk? (y/n) > ")? {
            YesNo::NO => break,
            YesNo::YES => res.push(DiskConfig {
                mid: readline.readline_t("MID > ")?,
                mode: readline.readline_t("Disk mode (USB/BLOCK/VIRTIO) > ")?,
                readonly: readline.readline_t::<YesNo>("Readonly (y/n) > ")?.into(),
            }),
        }
    }

    Ok(res)
}
