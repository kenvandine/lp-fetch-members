use anyhow::{Context, Result, bail};
use clap::Parser;
use lpcli::{auth, client::LaunchpadClient, people};

/// List Launchpad team members by join date and count who joined before you.
/// Requires credentials from `lpcli login`.
#[derive(Parser)]
struct Args {
    /// Launchpad team name
    #[arg(short, long, default_value = "canonical")]
    team: String,

    /// Your Launchpad username (no leading ~)
    #[arg(short, long)]
    user: String,

    /// Print every member who joined before you
    #[arg(short, long)]
    verbose: bool,
}

fn name_from_link(link: &Option<String>) -> String {
    link.as_deref()
        .and_then(|l| l.rsplit('/').next())
        .unwrap_or("?")
        .trim_start_matches('~')
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let creds = auth::load_credentials()
        .context("no Launchpad credentials found; run `lpcli login` first")?;
    let lp = LaunchpadClient::new(Some(creds));

    let members = people::get_team_members(&lp, &args.team).await?;
    let mut active: Vec<_> = members
        .iter()
        .filter(|m| matches!(m.status.as_deref(), Some("Approved" | "Administrator")))
        .filter_map(|m| Some((m.date_joined?, name_from_link(&m.member_link))))
        .collect();
    active.sort();

    let Some(me) = active.iter().find(|(_, n)| *n == args.user) else {
        bail!("{} is not an active member of {}", args.user, args.team);
    };
    let my_date = me.0;

    let before: Vec<_> = active
        .iter()
        .filter(|(d, n)| *d < my_date && *n != args.user)
        .collect();

    if args.verbose {
        for (d, n) in &before {
            println!("{}  {}", d.format("%Y-%m-%d"), n);
        }
    }
    println!("Active members of ~{}: {}", args.team, active.len());
    println!("{} joined on {}", args.user, my_date.format("%Y-%m-%d"));
    println!("Still in the team and joined before you: {}", before.len());
    Ok(())
}
