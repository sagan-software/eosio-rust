#[macro_export]
macro_rules! name_type {
    ($ident:ident) => {
        #[derive(
            Debug,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Default,
            Hash,
            PartialOrd,
            Ord,
            crate::bytes::Read,
            crate::bytes::Write,
            crate::bytes::NumBytes,
        )]
        #[eosio(crate_path = "crate::bytes")]
        pub struct $ident($crate::name::Name);

        impl $ident {
            #[must_use]
            pub const fn new(value: u64) -> Self {
                Self($crate::name::Name::new(value))
            }

            #[must_use]
            pub const fn as_u64(&self) -> u64 {
                self.0.as_u64()
            }

            #[must_use]
            pub const fn as_name(&self) -> $crate::name::Name {
                self.0
            }
        }

        impl core::ops::Deref for $ident {
            type Target = $crate::name::Name;

            #[must_use]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::convert::AsRef<$crate::name::Name> for $ident {
            #[must_use]
            fn as_ref(&self) -> &$crate::name::Name {
                &self.0
            }
        }

        impl core::convert::AsRef<$ident> for $ident {
            #[must_use]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl From<u64> for $ident {
            #[must_use]
            fn from(value: u64) -> Self {
                Self::new(value)
            }
        }

        impl From<$ident> for u64 {
            #[must_use]
            fn from(value: $ident) -> Self {
                value.as_u64()
            }
        }

        impl From<$crate::name::Name> for $ident {
            #[must_use]
            fn from(value: $crate::name::Name) -> Self {
                Self(value)
            }
        }

        impl From<$ident> for $crate::name::Name {
            #[must_use]
            fn from(value: $ident) -> Self {
                value.as_name()
            }
        }

        impl core::str::FromStr for $ident {
            type Err = $crate::name::ParseNameError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let name = $crate::name::Name::from_str(s)?;
                Ok(Self(name))
            }
        }
    };
}
