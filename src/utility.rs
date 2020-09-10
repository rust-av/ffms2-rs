extern crate paste;

#[macro_export]
macro_rules! simple_enum {
    ($enum:ident, ($($field_name:ident),*$(,)*)) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $enum {
            $($field_name,)*
        }
    }
}

#[macro_export]
macro_rules! create_enum {
    ($enum:ident, $type:ident, $func_name:ident,
    ($($field_name:ident),*$(,)*)) => {

        simple_enum!($enum, ($($field_name),*));

        impl $enum {
            paste::item! {
                pub(crate) fn [<to_ $func_name>](&self) -> $type {
                    match self {
                        $(
                            $enum::$field_name => $type::[<FFMS_ $field_name>],
                        )*
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! from_i32 {
    ($enum:ident, $type:ident,
    ($($field_name:ident),*$(,)*)) => {
        impl $enum {
            paste::item! {
                pub(crate) fn from_i32(e: i32) -> Self {
                    match e {
                        $(
                            e if e == $type::[<FFMS_ $field_name>] as i32 => $enum::$field_name,
                        )*
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! display {
    ($enum:ident, ($($field_name:ident: $field_err:expr),*$(,)*)) => {
        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let v = match self {
                    $(
                        $enum::$field_name => $field_err,
                    )*
                };

                write!(f, "{}", v)
            }
        }
    }
}

#[macro_export]
macro_rules! errors {
    ($enum:ident, $type:ident,
    ($($field_name:ident: $field_err:expr),*$(,)*)) => {

        simple_enum!($enum, ($($field_name,)*));

        display!($enum, ($($field_name: $field_err,)*));

        from_i32!($enum, $type, ($($field_name,)*));

    }
}

#[macro_export]
macro_rules! set_struct {
    ($struct:ident, $param:ident, $type:tt) => {
        pub struct $struct {
            $param: $type,
        }
    };
}

#[macro_export]
macro_rules! default_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*))
     => {
      impl Default for $struct {
            fn default() -> Self {
                let $param = $type {
                    $($field_name: $field_default_expr,)*
                };
                $struct{ $param }
            }
        }
    }
}

#[macro_export]
macro_rules! create_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_type:ty),*$(,)*),
     ($($field_default_expr:expr),*$(,)*),
     ($($field_expr:expr),*$(,)*)
     ) => {

        set_struct!($struct, $param, $type);

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*));

        set_params!($struct, $param,
                   ($($field_name,)*),
                   ($($field_type,)*),
                   ($($field_expr,)*));
    }
}

#[macro_export]
macro_rules! set_params {
    ($struct:ident, $param:ident,
    ($($field_name:ident),*$(,)*),
    ($($field_type:ty),*$(,)*),
    ($($field_expr:expr),*$(,)*))
    => {
            impl $struct {
                paste::item! {
                    $(
                        pub fn [<set_ $field_name>](&mut self, $field_name: $field_type) {
                            self.$param.$field_name = paste::expr! { ($field_expr) }
                        }
                    )*
                }
            }
       }
}
