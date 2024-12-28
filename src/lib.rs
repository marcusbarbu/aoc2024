use core::fmt;
use std::env;
use std::path::PathBuf;

use tracing::{error, info, Level};

pub mod counter;
pub mod graph;
pub mod map_vec_extend;

#[derive(Debug, Clone)]
pub enum RequestedAocInputType {
    Real,
    Test,
    CustomTest { fname: String },
}

pub struct AocHelper {
    _day: u32,
    test_inputs: Vec<PathBuf>,
    real_input: PathBuf,
}

#[derive(Debug, Clone)]
pub enum AocHelperError {
    FileReadError,
    TimeoutError,
}

pub type AocResult<T> = std::result::Result<T, AocHelperError>;

impl fmt::Display for AocHelperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AocHelperError::FileReadError => {
                write!(f, "File read error! Could not read file!")
            }
            AocHelperError::TimeoutError => {
                write!(f, "Timed out!!")
            }
        }
    }
}

impl AocHelper {
    fn setup_logging() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // .with_max_level(Level::ERROR)
            // .with_max_level(Level::INFO)
            .with_max_level(Level::DEBUG)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_ansi(false)
            .with_ansi(true)
            // .pretty()
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    pub fn new(day: u32, addl_test_inputs: Option<Vec<String>>) -> Self {
        AocHelper::setup_logging();
        let _ = dotenvy::dotenv();
        let base_path: String = env::var("STATIC_BASE_PATH").unwrap();
        let mut day_input: PathBuf = PathBuf::new();
        day_input.push(base_path);
        day_input.push(format!("day{}", day));

        let mut real_input: PathBuf = day_input.clone();
        real_input.push("real_input");

        let mut test_inputs: Vec<PathBuf> = Vec::new();
        let mut default_test_input: PathBuf = day_input.clone();
        default_test_input.push("test_input");
        test_inputs.push(default_test_input);

        match addl_test_inputs {
            Some(input_fnames) => {
                input_fnames.into_iter().for_each(|fname| {
                    let mut new_test_path: PathBuf = day_input.clone();
                    new_test_path.push(fname);
                    test_inputs.push(new_test_path);
                });
            }
            None => {}
        };

        AocHelper {
            _day: day,
            test_inputs: test_inputs,
            real_input: real_input,
        }
    }

    pub fn get_real_input_path(&self) -> PathBuf {
        self.real_input.clone()
    }

    pub fn get_test_input_path(&self, fname: Option<&str>) -> Option<PathBuf> {
        if let Some(target) = fname {
            return self
                .test_inputs
                .iter()
                .find(|buf| buf.ends_with(target))
                .cloned();
        }
        return self.test_inputs.iter().next().cloned();
    }

    fn get_fname_as_string(fname: &PathBuf) -> AocResult<String> {
        let data = std::fs::read_to_string(&fname);
        match data {
            Ok(d) => return Ok(d),
            Err(e) => {
                error!(
                    "Failed to read file {}, err: {}",
                    fname.to_str().unwrap_or("BAD_FNAME"),
                    e
                );
                return Err(AocHelperError::FileReadError);
            }
        }
    }

    pub fn get_input_as_string(&self, input_type: RequestedAocInputType) -> AocResult<String> {
        match input_type {
            RequestedAocInputType::Real => {
                return AocHelper::get_fname_as_string(&self.real_input);
            }
            RequestedAocInputType::Test => {
                return AocHelper::get_fname_as_string(&self.test_inputs.iter().next().unwrap());
            }
            RequestedAocInputType::CustomTest { fname } => {
                let fname = self.get_test_input_path(Some(fname.as_str()));
                info!("Working on fname: {:?}", fname);
                if let Some(pb) = fname {
                    return AocHelper::get_fname_as_string(&pb);
                }
            }
        }

        Err(AocHelperError::FileReadError)
    }
}
