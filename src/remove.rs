use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut remove = remove();
    remove.add_subcommand(user());
    remove.add_subcommand(group());
    remove.add_subcommand(media());
    cli.add_command(remove);
}

#[clik_command(user, "Remove a user")]
#[clik_arg(uid, "The UID for the user to remove")]
async fn user(state: &mut Velocity, uid: UID) {
    state.user_remove(uid).await?;

    println!("Removed user with UID = {uid}");

    Ok(())
}

#[clik_command(group, "Remove a group")]
#[clik_arg(gid, "The GID for the user to remove")]
async fn group(state: &mut Velocity, gid: GID) {
    state.group_remove(gid).await?;

    println!("Removed group with GID = {gid}");

    Ok(())
}

#[clik_command(media, "Remove a piece of media")]
#[clik_arg(mid, "The media id of the piece of media to remove")]
async fn media(state: &mut Velocity, mid: MID) {
    state.media_remove(mid.clone()).await?;

    println!("Removed media with MID = {mid}");

    Ok(())
}

#[clik_command(remove, "Remove something")]
async fn remove(state: &mut Velocity) {
    Ok(())
}
