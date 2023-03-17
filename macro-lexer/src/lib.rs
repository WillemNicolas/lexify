pub mod lexer ;
pub mod utils {
    pub mod tokenStream;
    pub mod lexerError;
}

#[cfg(test)]
mod tests {
    mod test;
}