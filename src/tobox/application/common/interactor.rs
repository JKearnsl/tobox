use crate::application::common::exceptions::ApplicationError;

pub trait Interactor<I, O> {
    async fn execute(&self, data: I) -> Result<O, ApplicationError>;
}