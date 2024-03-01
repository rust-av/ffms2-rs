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

macro_rules! from_i32 {
    ($enum:ident, $type:ty,
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

macro_rules! display {
    ($enum:ident, ($($field_name:ident: $field_err:expr),*$(,)*)) => {
        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let v = match self {
                    $(
                        $enum::$field_name => $field_err,
                    )*
                };
                v.fmt(f)
            }
        }
    }
}

macro_rules! errors {
    ($enum:ident, $type:ident,
    ($($field_name:ident: $field_err:expr),*$(,)*)) => {

        simple_enum!($enum, ($($field_name,)*));

        display!($enum, ($($field_name: $field_err,)*));

        from_i32!($enum, $type, ($($field_name,)*));

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
