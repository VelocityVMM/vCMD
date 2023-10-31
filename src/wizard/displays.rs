use rustyline::{error::ReadlineError, history::History, Editor, Helper};
use velocity::endpoints::v::DisplayConfig;

use super::{ReadlineExt, YesNo};

/// Prompt the user to configure displays for a virtual machine
/// # Arguments
/// * `readline` - The readline instance to use
pub fn wizard_displays<'a, H: Helper, I: History>(
    readline: &mut Editor<H, I>,
) -> Result<Vec<DisplayConfig>, ReadlineError> {
    let mut res = Vec::new();

    loop {
        match readline.readline_t::<YesNo>("Add another display? (y/n) > ")? {
            YesNo::NO => break,
            YesNo::YES => res.push(DisplayConfig {
                name: readline.readline("Display name > ")?,
                width: readline.readline_t("Display width > ")?,
                height: readline.readline_t("Display height > ")?,
                ppi: readline.readline_t("Display ppi > ")?,
            }),
        }
    }

    Ok(res)
}
