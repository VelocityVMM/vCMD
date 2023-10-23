use clik::*;
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use velocity::*;

pub fn register_commands(cli: &mut CLI<Velocity>) {
    let mut upload = upload();
    upload.add_subcommand(upload_media());
    cli.add_command(upload);
}

#[clik_command(media, "Upload new media")]
#[clik_arg(mpid, "The media pool id to upload to")]
#[clik_arg(gid, "The group id of the group the media should belong to")]
#[clik_arg(name, "A user-friendly name for the new media")]
#[clik_arg(
    ty,
    "A string describing the type of media this is - see the Velocity API documentation"
)]
#[clik_arg(path, "The path to the file to upload")]
#[clik_arg(readonly, "If this media should be read-only")]
async fn upload_media(
    state: &mut Velocity,
    mpid: MPID,
    gid: GID,
    name: String,
    ty: String,
    path: String,
    readonly: bool,
) {
    let file = tokio::fs::File::open(path).await?;
    let bar = ProgressBar::new(0);

    let style_e = ProgressStyle::with_template(
        "{prefix:<60} [{elapsed_precise}] [{msg:>23.yellow}] [{percent:>3.green}%] [{wide_bar}] ",
    );
    let style = style_e.unwrap();
    bar.set_length(file.metadata().await.unwrap().len());
    bar.set_style(style);
    bar.set_prefix(format!("Uploading {} to mediapool {}", name, mpid));

    let res = state
        .media_upload(mpid, gid, &name, &ty, readonly, file, move |total, done| {
            bar.set_position(done);
            bar.set_message(
                HumanBytes(done as u64).to_string()
                    + " / "
                    + HumanBytes(total as u64).to_string().as_str(),
            );
        })
        .await?;

    println!(
        "Uploaded new media '{name}' in pool {mpid}. MID: {}, size: {} bytes",
        res.mid, res.size
    );

    Ok(())
}

#[clik_command(upload, "Upload something")]
async fn upload(state: &mut Velocity) {
    Ok(())
}
