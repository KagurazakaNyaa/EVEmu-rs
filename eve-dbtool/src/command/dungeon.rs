use crate::internal::config::Config;

use super::index::DungeonSubcommands;

fn list() {
    todo!()
}
fn apply(overwrite: &bool) {
    todo!()
}
fn import(file: &String, overwrite: &bool) {
    todo!()
}
fn export(dungeon: &u64, file: &String, dryrun: &bool) {
    todo!()
}
fn new(name: &String, status: &u8, faction: &u64, archetype: &u64, dryrun: &bool) {
    todo!()
}
fn delete(dungeon: &u64) {
    todo!()
}
fn addRoom(dungeon: &u64, name: &String, dryrun: &bool) {
    todo!()
}
fn removeRoom(dungeon: &u64, room: &u64, dryrun: &bool) {
    todo!()
}
fn listRooms(dungeon: &u64) {
    todo!()
}
fn listArchetypes() {
    todo!()
}
fn listFactions() {
    todo!()
}

pub fn run(command: &DungeonSubcommands, config: &Config) {
    match command {
        DungeonSubcommands::List {} => list(),
        DungeonSubcommands::Apply { overwrite } => apply(overwrite),
        DungeonSubcommands::Import { file, overwrite } => import(file, overwrite),
        DungeonSubcommands::Export {
            dungeon,
            file,
            dryrun,
        } => export(dungeon, file, dryrun),
        DungeonSubcommands::New {
            name,
            status,
            faction,
            archetype,
            dryrun,
        } => new(name, status, faction, archetype, dryrun),
        DungeonSubcommands::Delete { dungeon } => delete(dungeon),
        DungeonSubcommands::AddRoom {
            dungeon,
            name,
            dryrun,
        } => addRoom(dungeon, name, dryrun),
        DungeonSubcommands::RemoveRoom {
            dungeon,
            room,
            dryrun,
        } => removeRoom(dungeon, room, dryrun),
        DungeonSubcommands::ListRooms { dungeon } => listRooms(dungeon),
        DungeonSubcommands::ListArchetypes {} => listArchetypes(),
        DungeonSubcommands::ListFactions {} => listFactions(),
    }
}
