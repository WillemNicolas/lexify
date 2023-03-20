pub mod lexer ;
pub mod utils {
    pub mod token;
    pub mod lexer_error;

    pub mod macro_lexer;
}

#[cfg(test)]
mod tests {
    mod test;
}