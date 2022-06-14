use structopt::StructOpt;

/// Get or set string in KV database.
#[derive(StructOpt)]
enum Opt {
    /// Set value "value" with key "name"
    Set { name: String, value: String },
    /// Get value with key "name"
    Get { name: String },
}

fn program_name() -> String {
    std::env::args().nth(0).unwrap()
}

fn main() {
    match runtime() {
        Ok(code) => {
            std::process::exit(code);
        }
        Err(error) => {
            eprintln!("{}: {}", program_name(), error);
            std::process::exit(100);
        }
    }
}

fn runtime() -> anyhow::Result<i32> {
    let opt = Opt::from_args();

    let db: sled::Db = sled::open(dirs::data_local_dir().unwrap().join("ckv.db"))?;

    match opt {
        Opt::Set { name, value } => {
            db.insert(&name, value.into_bytes()).unwrap();
        }
        Opt::Get { name } => match db.get(&name)? {
            None => {
                eprintln!("{}: name not found", program_name());
                return Ok(1);
            }
            Some(value) => match String::from_utf8(value.to_vec()) {
                Ok(value) => println!("{}", value),
                Err(_) => {
                    eprintln!("{}: value not utf8", program_name());
                    return Ok(2);
                }
            },
        },
    }

    Ok(0)
}
