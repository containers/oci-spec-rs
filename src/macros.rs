#[cfg(not(feature = "builder"))]
macro_rules! make_pub {
    {
        $(#[$outer:meta])*
        struct $name:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $field:ident: $t:ty,
            )*
        }
    } => {
        $(#[$outer])*
        pub struct $name {
            $(
                $(#[$inner $($args)*])*
                pub $field: $t,
            )*
        }
    }
}

#[cfg(feature = "builder")]
macro_rules! make_pub {
    {
        $(#[$outer:meta])*
        struct $name:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $field:ident: $t:ty,
            )*
        }
    } => {
        $(#[$outer])*
        pub struct $name {
            $(
                $(#[$inner $($args)*])*
                $field: $t,
            )*
        }
    }
}
