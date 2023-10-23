use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut create = create();
    create.add_subcommand(create_user());
    create.add_subcommand(create_group());
    create.add_subcommand(create_media());
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

#[clik_command(media, "Allocate new media")]
#[clik_arg(mpid, "The media pool id to allocate in")]
#[clik_arg(gid, "The group id of the group the media should belong to")]
#[clik_arg(name, "A user-friendly name for the new media")]
#[clik_arg(
    ty,
    "A string describing the type of media this is - see the Velocity API documentation"
)]
#[clik_arg(size, "The size of the new media")]
async fn create_media(
    state: &mut Velocity,
    mpid: MPID,
    gid: GID,
    name: String,
    ty: String,
    size: u64,
) {
    let mid = state.media_allocate(mpid, gid, &name, &ty, size).await?.mid;

    println!("Created new media '{name}' in pool {mpid}. MID: {mid}");

    Ok(())
}

#[clik_command(create, "Create something")]
async fn create(state: &mut Velocity) {
    Ok(())
}
