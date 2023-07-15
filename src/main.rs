use std::env;

use hyprland::data::{Monitors, Workspace};
use hyprland::dispatch::*;
use hyprland::prelude::*;

fn main() -> hyprland::Result<()> {
    let selected_ws = match env::args().nth(1) {
        Some(value) => value.parse::<i32>().expect("Workspace identifier expected"),
        None => panic!("Workspace identifier expected"),
    };

    if selected_ws == Workspace::get_active()?.id {
        return Ok(());
    }

    match Monitors::get()?
        .filter(|mon| mon.active_workspace.id == selected_ws)
        .next()
    {
        Some(mon) => hyprland::dispatch!(
            SwapActiveWorkspaces,
            MonitorIdentifier::Current,
            MonitorIdentifier::Id(mon.id as u8)
        )?,
        None => {
            hyprland::dispatch!(
                MoveWorkspaceToMonitor,
                WorkspaceIdentifier::Id(selected_ws),
                MonitorIdentifier::Current
            )?;
            hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(selected_ws))?;
        }
    }
    Ok(())
}
