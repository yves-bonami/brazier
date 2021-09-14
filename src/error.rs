/// This error is returned whenever something goes wrong within the mediator itself.
#[derive(Debug, PartialEq)]
pub enum MediatorError {
    /// The handler is not registerd.
    /// Please register the handler before using it.
    HandlerNotRegisteredError,
}

impl std::error::Error for MediatorError {}

impl std::fmt::Display for MediatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediatorError::HandlerNotRegisteredError => write!(f, "Handler not registered"),
        }
    }
}
