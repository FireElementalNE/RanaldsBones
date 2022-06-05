pub mod args {
    use std::str::FromStr;
    use anyhow::Result;
    use clap::{Arg, ArgMatches, Command};
    pub const TOME_ARG: &str = "tomes";
    pub const GRIM_ARG: &str = "grimoires";
    pub const BONUS_ARG: &str = "bonus";
    fn get_args() -> ArgMatches {
        let app = Command::new("Ranald's Bones")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Play Ranald's Bones!")
            .arg(Arg::new(TOME_ARG)
                .required(true)
                .help("Number of tomes (MAX 3d")
                .possible_values(["0", "1", "2", "3"])

            )
            .arg(Arg::new(GRIM_ARG)
                .required(true)
                .help("Number of grimoires (MAX 2)")
                .possible_values(["0", "1", "2"])
            )
            .arg(Arg::new(BONUS_ARG)
                .required(true)
                .help("Number of bonus dice (MAX 2)")
                .possible_values(["0", "1", "2"])
            );
        app.get_matches()
    }
    pub struct AppArgs {
        am: ArgMatches
    }
    impl AppArgs {
        pub fn new() -> Self {
            Self {
                am: get_args(),
            }
        }
        pub fn get_argument<T>(&self, arg_name: &str) -> Result<T, T::Err> where T: FromStr {
            let arg = self.am
                .value_of(arg_name)
                .expect(format!("Argument {arg_name} not present").as_str())
                .parse::<T>()?;
            Ok(arg)
        }
    }
}
pub mod log_utils {
    use env_logger::fmt::Color;
    use log::Level;
    use std::path::Path;
    use std::io::Write;
    use anyhow::Result;

    pub fn init_log() -> Result<()> {
        env_logger::builder()
            .format(|buf, record| {
                let mut level_style = buf.style();
                match record.level() {
                    Level::Error => level_style.set_color(Color::Red).set_bold(true),
                    Level::Warn => level_style.set_color(Color::Yellow).set_bold(true),
                    Level::Info => level_style.set_color(Color::Green).set_bold(true),
                    Level::Debug => level_style.set_color(Color::Blue).set_bold(true),
                    Level::Trace => level_style.set_color(Color::White).set_bold(true),
                };
                let fname = Path::new(record.file().unwrap()).file_name();
                match fname {
                    Some(fname) => {
                        // TODO: this is wrong need the and_then or ?
                        let fname_str = fname.to_str().unwrap();
                        writeln!(
                            buf,
                            "[{}][{}][{}:{}]: {}",
                            level_style.value(record.level()),
                            buf.timestamp(),
                            fname_str,
                            record.line().unwrap(),
                            record.args()
                        )?;
                    }
                    None => {
                        writeln!(
                            buf,
                            "[{}][{}]: {}",
                            level_style.value(record.level()),
                            buf.timestamp(),
                            record.args(),
                        )?;
                    }
                }
                Ok(())
            }).init();
        Ok(())
    }
}