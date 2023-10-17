use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut create = create();
    create.add_subcommand(create_user());
    create.add_subcommand(create_group());
    cli.add_command(create);
}

#[clik_command(user, "Create a new user")]
#[clik_arg(username, "The username for the new user")]
async fn create_user(state: &mut Velocity, username: String) {
    let password = rpassword::prompt_password(format!("Password for new user '{username}': "))?;

    let uid = state.user_create(&username, &password).await?;

    println!("Created new user '{username}': UID = {uid}");

    Ok(())
}

#[clik_command(group, "Create a new group")]
#[clik_arg(name, "The name for the new group")]
#[clik_arg(parent_gid, "The group id the new group shold be a part of")]
async fn create_group(state: &mut Velocity, name: String, parent_gid: GID) {
    let group = state.group_create(parent_gid, &name).await?;

    println!("Created new group '{name}': GID = {}", group.gid);

    Ok(())
}

#[clik_command(create, "Create something")]
async fn create(state: &mut Velocity) {
    Ok(())
}
