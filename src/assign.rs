use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut assign = assign();
    assign.add_subcommand(assign_user());
    assign.add_subcommand(assign_pool());
    cli.add_command(assign);

    let mut revoke = revoke();
    revoke.add_subcommand(revoke_user());
    revoke.add_subcommand(revoke_pool());
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

#[clik_command(pool, "Assign a pool to a group")]
#[clik_arg(mpid, "The media pool id of the pool to assign")]
#[clik_arg(gid, "The group id of the group to assign to")]
#[clik_arg(
    quota,
    "The quota of available space the group should have in the pool"
)]
#[clik_arg(write, "If the group can write to media in the pool")]
#[clik_arg(manage, "If the group can create and remove media in the pool")]
async fn assign_pool(
    state: &mut Velocity,
    mpid: MPID,
    gid: GID,
    quota: u64,
    write: bool,
    manage: bool,
) {
    state.pool_assign(gid, mpid, quota, write, manage).await?;

    println!(
        "Assigned pool {} to group {}: write: {}, manage: {}",
        mpid, gid, write, manage
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

#[clik_command(pool, "Revoke a pool from a group")]
#[clik_arg(mpid, "The media pool id of the pool to revoke")]
#[clik_arg(gid, "The group id of the group to revoke the pool from")]
async fn revoke_pool(state: &mut Velocity, mpid: MPID, gid: GID) {
    state.pool_revoke(gid, mpid).await?;

    println!("Revoked pool {} from group {}", mpid, gid);

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
