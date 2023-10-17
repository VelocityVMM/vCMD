use clik::*;
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut list = list();
    list.add_subcommand(users());
    list.add_subcommand(groups());
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

#[clik_command(list, "List something")]
fn list(state: &mut Velocity) {
    Ok(())
}
