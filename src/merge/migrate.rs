use ::std::collections::HashSet;

use ::ustr::{Ustr, UstrSet};

use crate::ast::evolution::Block;
use crate::ast::object::{FieldOp, ObjectOp};
use crate::ast::term::Iden;
use crate::Evolutions;
use crate::merge::object::State;

pub fn migrate(state: &State, evolutions: &Evolutions) -> State {
    //TODO @mark: maybe UstrSet?
    let mut seen = HashSet::new();
    for evolution in evolutions {
        for block in evolution.blocks {
            match block {
                Block::Obj(obj_ev) => match obj_ev.operation {
                    ObjectOp::Add(add_op) => {
                        if seen.contains(&obj_ev.identifier) {
                            todo!();  //TODO @mark: resolve potential conflict
                        }
                        seen.insert(obj_ev.identifier);
                    }
                    ObjectOp::Change(change_op) => {
                        if seen.contains(&obj_ev.identifier) {
                            todo!();  //TODO @mark: resolve potential conflict
                        }
                        seen.insert(obj_ev.identifier);
                        for field_ev in change_op.operations {
                            match field_ev.operation {
                                FieldOp::Add(field_add_op) => {}
                                FieldOp::Change(field_change_op) => {}
                                FieldOp::Delete() => {}
                            }
                            seen.insert(Iden::new_span(format!("{}/{}", &obj_ev.identifier, &field_ev.name), field_ev.name.span));
                        }
                    }
                    ObjectOp::Delete(delete_op) => {
                        if seen.contains(&obj_ev.identifier) {
                            todo!();  //TODO @mark: resolve potential conflict
                        }
                        seen.insert(obj_ev.identifier);
                    }
                }
            }
        }
    }
    todo!()
}
