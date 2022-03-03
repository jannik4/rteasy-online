// use crate::vhdl;
// use std::fmt::{Display, Formatter, Result};
//
// #[derive(Debug)]
// pub struct RenderLabel<'s>(pub &'s vhdl::Label<'s>);
//
// impl Display for RenderLabel<'_> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self.0 {
//             vhdl::Label::Named(named) => write!(f, "NAMED_{}", named.0),
//             vhdl::Label::Unnamed(idx) => write!(f, "UNNAMED_{}", idx),
//             vhdl::Label::End => write!(f, "TERMINATED"),
//         }
//     }
// }
