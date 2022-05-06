use pollster;
// Local crates.
mod inputs;
mod markdown;
mod windows;
use windows::run;
fn main() {
    pollster::block_on(run());
}
