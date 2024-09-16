macro_rules! driver_wrapper_impl {
    ($ident:ident,$driver: path) => {
        unsafe impl Send for $ident {}

        impl<D: $driver + 'static> From<D> for $ident {
            fn from(value: D) -> Self {
                Self(Box::new(value))
            }
        }

        impl std::ops::Deref for $ident {
            type Target = dyn $driver;
            fn deref(&self) -> &Self::Target {
                &*self.0
            }
        }

        impl std::ops::DerefMut for $ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut *self.0
            }
        }

        impl $ident {
            #[allow(unused)]
            pub fn as_driver(&self) -> &dyn $driver {
                &*self.0
            }
        }
    };
}

macro_rules! driver_wrapper {
    ([$doc:expr] $ident:ident [$driver: path]) => {
        #[doc = $doc]
        pub struct $ident(Box<dyn $driver>);

        crate::macros::driver_wrapper_impl!($ident, $driver);
    };
}

pub(crate) use driver_wrapper;
pub(crate) use driver_wrapper_impl;
