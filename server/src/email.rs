pub trait Email {
    fn subject(&self) -> String;
    fn body(&self) -> String;
}
