use std::io::Write;

use clik::*;
use velocity::Velocity;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    cli.add_command(auth());
    cli.add_command(reauth());
    cli.add_command(deauth());
}

#[clik_command(auth, "Authenticate this client")]
#[clik_arg(username, "The username for the user to authenticate as")]
pub async fn auth(state: &mut Velocity, username: String) {
    print!("Password for {}: ", username);
    std::io::stdout().flush()?;
    let password = rpassword::read_password()?;

    state.authenticate(&username, &password).await?;

    println!("Authenticated as '{}'", username);

    Ok(())
}

#[clik_command(reauth, "Reauthenticate this client")]
pub async fn reauth(state: &mut Velocity) {
    state.reauthenticate().await?;

    println!("Reauthenticated!");

    Ok(())
}

#[clik_command(deauth, "Deauthenticate this client")]
async fn deauth(state: &mut Velocity) {
    state.deauthenticate().await?;

    println!("Deauthenticated!");

    Ok(())
}
