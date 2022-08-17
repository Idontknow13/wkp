/*!
This module contains the `to_arg` macro, which automatically implements the needed
traits for converting a type into a clap argument.
*/

#[macro_export]
/// Converts a specified type into a valid clap argument (has `ToString` + `FromStr`)
///
/// ## Exmaple
///
/// ```
/// use wikipedia_cli::to_clap_arg;
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)] // `PartialEq` needed for testing
/// enum Foo {
///     Bar,
///     Baz
/// }
///
/// to_clap_arg! {
///     type: Foo,
///     variants:
///         "bar", "b" => Bar,
///         "baz", "bz" => Baz
/// }
///
/// # fn main() {
/// let b = Foo::Bar;
///
/// assert_eq!(String::from("bar"), b.to_string());
/// assert_eq!(Ok(Foo::Baz), Foo::from_str("bz"));
/// # }
/// ```
macro_rules! to_clap_arg {
    ( type: $t:ident, variants: $( $long:expr, $short:expr => $variant:ident ),+ ) => {
        impl ToString for $t {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Self::$variant => $long,
                    )+
                }.to_string()
            }
        }

        impl std::str::FromStr for $t {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $long | $short => Ok(Self::$variant),
                    )+
                    _ => Err("the provided variant doesn't exist")
                }
            }
        }
    };
}
