#[macro_export]
macro_rules! lexer {
    (with $tokentype:ty, $($tail:tt)*) => {
        {
            let mut builder = LexerBuilder::<$tokentype>::new();
            lexer!(builder,$($tail)*);
            builder.build()
        }
    };
    ($id:ident,eof $eof:path, $($tail:tt)*) => {
        $id.eof($eof);
        lexer!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => $t:path ,$($tail:tt)*) => {
        $id.rule($regex, $t);
        lexer!($id,$($tail)*);
    };
    ($id:ident,) => {};
}