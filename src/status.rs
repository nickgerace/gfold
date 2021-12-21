#[derive(Debug, Clone)]
pub enum Status {
    Bare,
    Clean,
    Unclean,
    Unpushed,
}
