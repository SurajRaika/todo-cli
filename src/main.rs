// main.rs
mod cli;
mod test;
mod localStorage;
mod task;
mod migration;
fn main() {
    migration::init_db();
    cli::run_cli();
}
