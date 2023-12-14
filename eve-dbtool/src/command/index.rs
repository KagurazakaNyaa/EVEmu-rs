use clap::{arg, Parser, Subcommand};

use crate::internal::config::Config;

use super::{dungeon, install, seed};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Installs the base database and migrates to the most recent version available.
    Install {
        /// Limit the number of migrations (0 = unlimited).
        #[arg(short, long, default_value_t = 0)]
        limit: u64,
        /// Don't apply migrations, just print them.
        #[arg(long)]
        dryrun: bool,
    },
    /// Seeds EVEmu with default market data.
    Seed {
        /// Don't apply migrations, just print them.
        #[arg(long)]
        dryrun: bool,
    },
    /// Manage EVEmu dungeons.
    Dungeon {
        #[command(subcommand)]
        command: DungeonSubcommands,
    },
}

#[derive(Subcommand)]
pub enum DungeonSubcommands {
    /// Lists all dungeons in the database.
    List {},
    /// Apply all dungeons from the dungeon directory.
    Apply {
        /// Overwrite existing dungeon if a match is found.
        #[arg(long)]
        overwrite: bool,
    },
    /// Import a dungeon from file.
    Import {
        /// File to import.
        #[arg(short, long)]
        file: String,
        /// Overwrite existing dungeon if a match is found.
        #[arg(long)]
        overwrite: bool,
    },
    /// Exports a specific dungeon from the database to a file.
    Export {
        /// ID of the dungeon to export.
        #[arg(short, long)]
        dungeon: u64,
        /// Filename to write the dungeon data to.
        #[arg(short, long)]
        file: String,
        /// Don't apply migrations, just print them.
        #[arg(long)]
        dryrun: bool,
    },
    /// Creates a new blank dungeon.
    New {
        /// Name of the new dungeon.
        #[arg(short, long)]
        name: String,
        /// Status (1=Release, 2=Testing, 3=Working Copy).
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..3))]
        status: u8,
        /// Faction ID (run 'eve-dbtool dungeon list-factions' to list available)
        #[arg(short, long)]
        faction: u64,
        /// Archetype ID (run 'eve-dbtool dungeon list-archetypes' to list available)
        #[arg(short, long)]
        archetype: u64,
        /// Don't apply migrations, just print them.
        #[arg(long)]
        dryrun: bool,
    },
    /// Delete a dungeon from the database.
    Delete {
        /// ID of the dungeon to delete.
        #[arg(short, long)]
        dungeon: u64,
    },
    /// Add a new room to a dungeon.
    AddRoom {
        /// ID of the dungeon for which to add the room.
        #[arg(short, long)]
        dungeon: u64,
        /// Name of the room to add.
        #[arg(short, long)]
        name: String,
        /// Don't apply change, just print the JSON output.
        #[arg(long)]
        dryrun: bool,
    },
    /// Remove a room from a dungeon.
    RemoveRoom {
        /// ID of the dungeon for which to remove the room.
        #[arg(short, long)]
        dungeon: u64,
        /// ID of the room to remove.
        #[arg(short, long)]
        room: u64,
        /// Don't apply change, just print the JSON output.
        #[arg(long)]
        dryrun: bool,
    },
    /// Lists all rooms in the specified dungeon.
    ListRooms {
        /// ID of the dungeon for which to add the room.
        #[arg(short, long)]
        dungeon: u64,
    },
    /// Lists all Archetype IDs along with their names.
    ListArchetypes {},
    /// Lists all faction IDs along with their names.
    ListFactions {},
}

impl Cli {
    pub fn run(&self, config: &Config) {
        match &self.command {
            Commands::Install { limit, dryrun } => install::run(limit, dryrun, config),
            Commands::Seed { dryrun } => seed::run(dryrun, config),
            Commands::Dungeon { command } => dungeon::run(command, config),
        }
    }
}
