#[macro_export]
macro_rules! parse {
    ($id:ident,eof $eof:path, $($tail:tt)*) => {
        $id.eof($eof);
        parse!($id,$($tail)*);
    };
    ($id:ident,rule $regex:literal => || {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|_| {$($tt)*})
    };
    ($id:ident,rule $regex:literal => |$param:ident| {$($tt:tt)*}, $($tail:tt)*) => {
        $id.rule($regex,|$param| {$($tt)*})
    };
    ($id:ident,) => {

    };
}
