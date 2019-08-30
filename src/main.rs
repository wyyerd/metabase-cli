#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod commands;
mod model;
mod schema;
mod snapshot;

use structopt::StructOpt;

#[macro_export]
macro_rules! error {
    ($msg:literal $(, $args:expr)*) => {
        Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!($msg $(, $args)*)))
    }
}

#[derive(StructOpt)]
struct Arguments {
    #[structopt(long = "database-url", help = "The url to connect to metabase's database")]
    database_url: Option<String>,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "export", about = "Export the config for a collection, question or dashboard to stdout")]
    Export {
        // #[structopt(help = "One of: database, table, collection, dashboard, or question")]
        #[structopt(help = "One of: table or question")]
        datasource: String,

        #[structopt(help = "The id/name to be exported (e.g. '1')")]
        identifier: String,
    },

    #[structopt(name = "import", about = "Import the records in a metabase config file")]
    Import {
        #[allow(dead_code)]
        filename: String,
    },
}

fn main() {
    let args = Arguments::from_args();
    let database = args.database_url.or_else(|| dotenv::var("DATABASE_URL").ok());
    let database_url = match database {
        Some(url) => url,
        None => {
            eprintln!("The --database-url argument must be passed, or the DATABASE_URL environment variable must be set.");
            std::process::exit(1);
        }
    };
    let result = match args.command {
        Command::Export {
            datasource: source,
            identifier: id,
        } => commands::export(&database_url, &source, &id),
        Command::Import {
            filename,
        } => commands::import(&database_url, &filename),
    };
    match result {
        Ok(()) => (),
        Err(err) => {
            eprintln!("error: {}\n\n    {:?}\n", err, err);
            std::process::exit(1);
        }
    }
}
