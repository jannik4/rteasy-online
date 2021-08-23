mod case_values;
mod const_eval;
mod double_assign;
mod ordering;
mod register_array_read;
mod sim;

use crate::mir::*;
use crate::symbols::Symbols;
use crate::Error;

pub fn check(symbols: &Symbols<'_>, mir: &mut Mir<'_>) -> Result<(), Error> {
    // Errors
    let mut errors = Vec::new();
    let mut error_sink = |e| errors.push(e);

    // Check double assign
    double_assign::check(symbols, &*mir, &mut error_sink)?;

    // Check register array more than 2 reads
    register_array_read::check(symbols, &*mir, &mut error_sink)?;

    // Check case values
    case_values::check(&*mir, &mut error_sink)?;

    // Reorder unclocked
    ordering::check_and_order(mir, &mut error_sink)?;

    // Check errors
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Errors(errors))
    }
}
