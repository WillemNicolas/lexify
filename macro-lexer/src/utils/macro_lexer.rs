/*#[macro_export]
macro_rules! parse {
    ($id:ident,eof $eof:path, $($tail:tt)*) => {
        $id.eof($eof);
        parse!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|_| {$($tt)*});
        parse!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => || {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|_| {$($tt)*});
        parse!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => |$param:ident| {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|$param| {$($tt)*});
        parse!($id,$($tail)*);
    };
    ($id:ident,) => {

    };
}*/

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
    ($id:ident,rule $regex:literal => {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|_| {$($tt)*});
        lexer!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => || {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|_| {$($tt)*});
        lexer!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => |$param:ident| {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|$param| {$($tt)*});
        lexer!($id,$($tail)*);
    };
    ($id:ident,) => {};
}