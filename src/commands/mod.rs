mod get;
mod set;
mod list;
mod forget;

pub use get::Get;
pub use set::Set;
pub use list::List;
pub use forget::Forget;

pub trait Command {
    type Args: Arguments;

    fn run_with_args(args: Self::Args);
    fn run(args: Vec<String>) {
        Self::run_with_args(Self::Args::parse(args));
    }
}

pub trait Arguments {
    fn parse(args: Vec<String>) -> Self;
}
