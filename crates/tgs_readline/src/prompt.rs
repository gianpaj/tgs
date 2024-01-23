//! Shell prompt


use tgs_services::{styled, styled_buf::StyledBuf};

use crate::line::LineCtx;

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt_left(&self, line_ctx: &LineCtx) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &LineCtx) -> StyledBuf;
}

/// Default implementation for [Prompt]
#[derive(Default)]
pub struct DefaultPrompt {}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, _line_ctx: &LineCtx) -> StyledBuf {
        styled!("> ")
    }

    fn prompt_right(&self, _line_ctx: &LineCtx) -> StyledBuf {
        styled!()
    }
}
