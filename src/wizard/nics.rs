use rustyline::{error::ReadlineError, history::History, Editor, Helper};
use velocity::{
    endpoints::v::{NICConfig, NICType},
    NICID,
};

use super::{ReadlineExt, YesNo};

/// Prompt the user to configure NICs for a virtual machine
/// # Arguments
/// * `readline` - The readline instance to use
pub fn wizard_nics<'a, H: Helper, I: History>(
    readline: &mut Editor<H, I>,
) -> Result<Vec<NICConfig>, ReadlineError> {
    let mut res = Vec::new();

    loop {
        match readline.readline_t::<YesNo>("Add another NIC? (y/n) > ")? {
            YesNo::NO => break,
            YesNo::YES => {
                let ty: NICType = readline.readline_t("NIC type (NAT/BRIDGE) > ")?;
                match ty {
                    NICType::NAT => res.push(NICConfig { ty: ty, host: None }),
                    NICType::BRIDGE => {
                        let host: NICID = readline.readline_t("Host NIC id > ")?;
                        res.push(NICConfig {
                            ty,
                            host: Some(host),
                        })
                    }
                }
            }
        }
    }

    Ok(res)
}
