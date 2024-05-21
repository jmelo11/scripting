use rustatlas::prelude::*;

use crate::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    reference_date: Date,
    expr: ExprTree,
}

impl Event {
    pub fn new(reference_date: Date, expr: ExprTree) -> Event {
        Event {
            reference_date,
            expr,
        }
    }

    pub fn reference_date(&self) -> Date {
        self.reference_date
    }

    pub fn expr(&self) -> &ExprTree {
        &self.expr
    }
}
