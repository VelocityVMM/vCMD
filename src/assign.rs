use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut assign = assign();
    assign.add_subcommand(assign_user());
    cli.add_command(assign);

    let mut revoke = revoke();
    revoke.add_subcommand(revoke_user());
    cli.add_command(revoke);
}

#[clik_command(user, "Assign a user to a group")]
#[clik_arg(gid, "The GID of the group to assign to")]
#[clik_arg(uid, "The UID of the user to assign")]
#[clik_arg(permission, "The permission to assign")]
async fn assign_user(state: &mut Velocity, gid: GID, uid: UID, permission: String) {
    state.user_add_permission(gid, uid, &permission).await?;

    println!(
        "Added permission '{}' to user {} on group {}",
        permission, uid, gid
    );

    Ok(())
}

#[clik_command(user, "Revoke a users permission on a group")]
#[clik_arg(gid, "The GID of the group to revoke from")]
#[clik_arg(uid, "The UID of the user to revoke from")]
#[clik_arg(permission, "The permission to revoke")]
async fn revoke_user(state: &mut Velocity, gid: GID, uid: UID, permission: String) {
    state.user_revoke_permission(gid, uid, &permission).await?;

    println!(
        "Revoked permission '{}' from user {} on group {}",
        permission, uid, gid
    );

    Ok(())
}

#[clik_command(assign, "Assign something")]
fn assign(state: &mut Velocity) {
    Ok(())
}

#[clik_command(revoke, "Revoke something")]
fn revoke(state: &mut Velocity) {
    Ok(())
}
