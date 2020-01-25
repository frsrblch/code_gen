use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum Visibility {
    Pub,
    PubCrate,
    Private,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Visibility::Private => Ok(()),
            Visibility::Pub => write!(f, "pub "),
            Visibility::PubCrate => write!(f, "pub (crate) "),
        }
    }
}
