use crate::middleware::error::error;

pub fn middleware(args: Vec<String>) {
  error(args);
}