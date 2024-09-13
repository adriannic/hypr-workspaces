use std::env;

use hyprland::data::{Monitors, Workspace};
use hyprland::dispatch::*;
use hyprland::prelude::*;

fn main() -> hyprland::Result<()> {
    let selected_ws_id = env::args()
        .nth(1)
        .expect("Workspace identifier expected")
        .parse::<i32>()
        .expect("Workspace identifier expected");

    // Do nothing if the selected workspace is the current workspace
    if selected_ws_id == Workspace::get_active()?.id {
        return Ok(());
    }

    // Swap workspaces if the selected workspace is active in another monitor
    if let Some(mon) = Monitors::get()?
        .iter()
        .find(|mon| mon.active_workspace.id == selected_ws_id)
    {
        hyprland::dispatch!(
            SwapActiveWorkspaces,
            MonitorIdentifier::Current,
            MonitorIdentifier::Id(mon.id)
        )?
    }

    // If the workspace exists move it to the current monitor
    let _ = hyprland::dispatch!(
        MoveWorkspaceToMonitor,
        WorkspaceIdentifier::Id(selected_ws_id),
        MonitorIdentifier::Current
    );

    // Switch to the desired workspace
    hyprland::dispatch!(
        Workspace,
        WorkspaceIdentifierWithSpecial::Id(selected_ws_id)
    )?;

    // match Monitors::get()?
    //     .iter()
    //     .find(|mon| mon.active_workspace.id == selected_ws)
    // {
    //     Some(mon) => hyprland::dispatch!(
    //         SwapActiveWorkspaces,
    //         MonitorIdentifier::Current,
    //         MonitorIdentifier::Id(mon.id)
    //     )?,
    //     None => {
    //         hyprland::dispatch!(
    //             MoveWorkspaceToMonitor,
    //             WorkspaceIdentifier::Id(selected_ws),
    //             MonitorIdentifier::Current
    //         )?;
    //         hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(selected_ws))?;
    //     }
    // }
    Ok(())
}
