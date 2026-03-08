//! The foundation library for building user interfaces.

pub mod composer;
pub mod element;
pub mod hook;
pub mod sequence;
pub mod store;
pub mod view;

#[macro_export]
macro_rules! elm {
    {$($e:expr),+} => {
        {
            $crate::element::Element::Packed {
                inner: vec![
                    $(
                        Box::new($e),
                    )+
                ],
            }
        }
    };
}
