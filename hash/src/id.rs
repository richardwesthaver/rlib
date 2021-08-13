#[allow(unused)]
macro_rules! id_u64 {
    ($($name:ident;)*) => {
        $(
            impl $name {
                /// Retrieves the time that the Id was created at.
                pub fn created_at(&self) -> DateTime<Utc> {
                    let offset = self.0 >> 22;
                    let secs = offset / 1000;
                    let millis = (offset % 1000) * 1_000_000; // 1 million nanoseconds in a millisecond

                    let tm = NaiveDateTime::from_timestamp(1_420_070_400 + secs as i64, millis as u32);
                    DateTime::from_utc(tm, Utc)
                }

                /// Immutably borrow inner Id.
                #[inline]
                pub fn as_u64(&self) -> &u64 {
                    &self.0
                }

                /// Mutably borrow inner Id.
                #[inline]
                pub fn as_mut_u64(&mut self) -> &mut u64 {
                    &mut self.0
                }
            }

            // This is a hack so functions can accept iterators that either:
            // 1. return the id itself (e.g: `MessageId`)
            // 2. return a reference to it (`&MessageId`).
            impl AsRef<$name> for $name {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            impl<'a> From<&'a $name> for $name {
                fn from(id: &'a $name) -> $name {
                    id.clone()
                }
            }

            impl From<u64> for $name {
                fn from(id_as_u64: u64) -> $name {
                    $name(id_as_u64)
                }
            }

            impl PartialEq<u64> for $name {
                fn eq(&self, u: &u64) -> bool {
                    self.0 == *u
                }
            }

            impl Display for $name {
                fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                    Display::fmt(&self.0, f)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
                    deserializer.deserialize_any(U64Visitor).map($name)
                }
            }

            impl From<$name> for u64 {
                fn from(id: $name) -> u64 {
                    id.0 as u64
                }
            }

            impl From<$name> for i64 {
                fn from(id: $name) -> i64 {
                    id.0 as i64
                }
            }
        )*
    }
}
