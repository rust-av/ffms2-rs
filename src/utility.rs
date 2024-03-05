extern crate paste;

macro_rules! simple_enum {
    ($enum:ident, ($($field_name:ident),*$(,)*)) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $enum {
            $($field_name,)*
        }
    }
}

macro_rules! create_enum {
    ($enum:ident, $type:ty, $func_name:ident,
    ($($field_name:ident),*$(,)*)) => {

        simple_enum!($enum, ($($field_name),*));

        impl $enum {
            paste::item! {
                pub(crate) fn [<to_ $func_name>](self) -> $type {
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

macro_rules! set_struct {
    ($struct:ident, $param:ident, $type:ty) => {
        pub struct $struct {
            $param: $type,
        }
    };
}

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

macro_rules! create_struct {
    ($struct:ident, $param:ident, $type:tt,
          ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*)
     ) => {

        set_struct!($struct, $param, $type);

        /// While the underlying pointer might not allow concurrent access from
        /// different threads (and it is consequently not "Sync"),
        /// that relies neither on thread-local storage nor thread-specific
        /// locks and is therefore safe to send.
        unsafe impl Send for $struct {}

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*));


    }
}
