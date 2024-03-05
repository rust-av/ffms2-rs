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
