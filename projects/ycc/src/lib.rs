pub enum IResult<I, O> {
    Success(I, O),
    Failure(Failure),
}

pub enum Failure {
    Fail(ErrorMessage),
    Fatal(ErrorMessage),
    Incomplete(Needed),
}

pub struct ErrorMessage {}

pub struct Needed {}
