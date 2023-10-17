use clik::*;
use velocity::*;

mod u_auth;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    u_auth::register_commands(cli);
    let mut userinfo = u_userinfo();
    userinfo.add_subcommand(u_spec_userinfo());
    cli.add_command(userinfo);
}

#[clik_command(userinfo, "Provide information about the current user")]
async fn u_userinfo(state: &mut Velocity) {
    print!("{}", state.user_info(None).await?);

    Ok(())
}

#[clik_command(user, "Provide information about another user")]
#[clik_arg(uid, "The UID of the user to retrieve information of")]
async fn u_spec_userinfo(state: &mut Velocity, uid: UID) {
    print!("{}", state.user_info(Some(uid)).await?);

    Ok(())
}
