use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use flate2::bufread::MultiGzDecoder;

use crate::sequence;