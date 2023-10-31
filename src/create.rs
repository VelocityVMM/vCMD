use clik::*;
use rustyline::DefaultEditor;
use velocity::*;

use crate::wizard::wizard_corevm;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut create_vm = create_vm();
    create_vm.add_subcommand(create_vm_efi());

    let mut create = create();
    create.add_subcommand(create_vm);
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

#[clik_command(efi, "Create a new EFI virtual machine")]
#[clik_arg(gid, "The group if of the group the virtual machine belongs to")]
#[clik_arg(name, "The name for the virtual machine")]
async fn create_vm_efi(state: &mut Velocity, gid: GID, name: String) {
    let config = velocity::endpoints::v::v_vm_efi::EFIVMConfig {
        rosetta: true,

        core: wizard_corevm(
            &mut DefaultEditor::new().expect("Create wizard Editor"),
            gid,
            &name,
        )?,
    };

    let vmid = state.vm_efi_create(config).await?;

    println!("New VM: {vmid}");

    Ok(())
}

#[clik_command(vm, "Create a new virtual machine")]
async fn create_vm(state: &mut Velocity) {
    Ok(())
}

#[clik_command(create, "Create something")]
async fn create(state: &mut Velocity) {
    Ok(())
}
