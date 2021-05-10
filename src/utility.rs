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
                #[allow(dead_code)]
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
macro_rules! implement_deref {
    ($struct:ident, $param:ident, $type:tt) => {
        impl std::ops::Deref for $struct {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.$param
            }
        }

        impl std::ops::DerefMut for $struct {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$param
            }
        }
    };
}

#[macro_export]
macro_rules! create_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*)
     ) => {

        set_struct!($struct, $param, $type);

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*));

        implement_deref!($struct, $param, $type);

        /// While the underlying pointer might not allow concurrent access from different threads (and is consequently not "Sync"),
        /// it relies neither on thread-local storage nor thread-specific locks and is therefore safe to send.
        unsafe impl Send for $struct {}
    }
}
