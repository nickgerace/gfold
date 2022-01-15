#[derive(Debug, Clone, Copy)]
pub enum Status {
    Bare,
    Clean,
    Unclean,
    Unpushed,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bare => "bare",
            Self::Clean => "clean",
            Self::Unclean => "unclean",
            Self::Unpushed => "unpushed",
        }
    }
}
