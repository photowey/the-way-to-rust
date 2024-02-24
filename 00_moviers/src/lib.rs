/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// ----------------------------------------------------------------

use std::error::Error;
use std::fmt::{self, Display};

use clap::{App, Arg, ArgMatches};

// ----------------------------------------------------------------

#[derive(Debug)]
pub struct ArgumentsError {
    message: String,
}

impl Display for ArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// ----------------------------------------------------------------

impl Error for ArgumentsError {}

// ----------------------------------------------------------------

pub struct Arguments {
    pub input: String,
    pub output: String,
}

impl Arguments {
    pub fn new(app: App) -> Result<Self, ArgumentsError> {
        let matches = app.get_matches();
        let input = parse(&matches, "input", "inout").unwrap();
        let output = parse(&matches, "output", "Output").unwrap();

        Ok(Arguments { input, output })
    }
}

// ----------------------------------------------------------------

pub fn run() -> Result<Arguments, ArgumentsError> {
    let app = app();

    Arguments::new(app)
}

// ----------------------------------------------------------------

fn app<'a>() -> App<'a> {
    let input_arg = Arg::with_name("input")
        .short('i')
        .long("input")
        .value_name("INPUT")
        .help("Sets the input file or directory")
        .required(true)
        .takes_value(true);

    let output_arg = Arg::with_name("output")
        .short('o')
        .long("output")
        .value_name("OUTPUT")
        .help("Sets the output file or directory")
        .takes_value(true);

    App::new("moviers")
        .version("1.0.0")
        .author("photowey")
        .about("A command line tool for extract audio from video.")
        .arg(input_arg)
        .arg(output_arg)
}

// ----------------------------------------------------------------

fn parse(matches: &ArgMatches, parameter: &str, target: &str) -> Result<String, ArgumentsError> {
    if let Some(input) = matches.value_of(parameter) {
        Ok(input.to_string())
    } else {
        Err(ArgumentsError {
            message: format!("{} not specified", target),
        })
    }
}
