use std::process;

use jigolo::model::ExitOutcome;
use jigolo::run;

fn main() {
    match run() {
        ExitOutcome::Success => {}
        ExitOutcome::AllPathsFailed => process::exit(1),
    }
}
