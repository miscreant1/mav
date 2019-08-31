use super::commander::Commander;

mod install;
mod start;

pub fn get_commander<'a>(version: &'a str, author: &'a str) -> Commander<'a, str> {
    Commander::new(
        version,
        author,
        vec![install::get_command(), start::get_command()],
    )
}
