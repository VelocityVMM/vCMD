use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut list = list();
    list.add_subcommand(users());
    list.add_subcommand(groups());
    list.add_subcommand(pools());
    list.add_subcommand(media());
    cli.add_command(list);
}

#[clik_command(users, "List all registered users")]
async fn users(state: &mut Velocity) {
    let users = state.user_list().await?;

    println!("Registered users:");
    for user in users {
        println!(" - '{}' ({})", user.name, user.uid);
    }

    Ok(())
}

#[clik_command(groups, "List all available groups for the current user")]
async fn groups(state: &mut Velocity) {
    let groups = state.group_list().await?;

    println!("Available groups:");
    for group in groups {
        println!(" - '{}' ({})", group.name, group.gid);
    }

    Ok(())
}

#[clik_command(pools, "List all available pools for a group")]
#[clik_arg(gid, "The group id of the group to list available pools of")]
async fn pools(state: &mut Velocity, gid: GID) {
    let pools = state.pool_list(gid).await?;

    println!("Pools available to group {}:", gid);
    for pool in pools {
        println!(
            " - [{:>2}] '{}' (write: {}, manage: {})",
            pool.mpid, pool.name, pool.write, pool.manage
        );
    }

    Ok(())
}

#[clik_command(media, "List all available media for a group")]
async fn media(state: &mut Velocity, gid: GID) {
    let media = state.media_list(gid).await?;

    println!("Media available to group {}:", gid);
    for media in media {
        println!(
            " - {} in pool [{:>2}] (readonly: {:>5}) => '{}'",
            media.mid, media.mpid, media.readonly, media.name
        );
    }

    Ok(())
}

#[clik_command(list, "List something")]
fn list(state: &mut Velocity) {
    Ok(())
}
