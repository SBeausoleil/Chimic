use std::fmt;

#[derive(Debug, Clone)]
pub struct EmptyVec {
    pub empty_parameter: String,
}
impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The argument {} vector is empty!", self.empty_parameter)
    }
}
impl std::error::Error for EmptyVec {}

#[derive(Debug, Clone)]
pub struct IllegalArgument {
    pub msg: String,
}
impl fmt::Display for IllegalArgument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.msg.as_str())
    }
}
impl std::error::Error for IllegalArgument {}
