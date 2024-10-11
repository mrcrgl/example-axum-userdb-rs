pub struct ValidationError {
    pub(crate) message: String,
}

pub trait Validate {
    fn validate(&self) -> Option<Vec<ValidationError>>;
}
