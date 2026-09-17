#[macro_export]
macro_rules! option_match {
    ($e:ident $($l:literal => $id:expr_2021),*) => {
        match $e{
            $($l => Some($id)),*,
            _ => None
        }
    };

    ($e:ident $($l:literal => $id:expr_2021),*; $none:expr_2021) => {
        match $e{
            $($l => Some($id)),*,
            _ => $none
        }
    };
}

pub enum Peek<'a> {
    Symbol(&'a str),
    Whitespace,
    Semicolon,
    IndexOutBound,
}
