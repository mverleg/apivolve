use std::collections::HashSet;
use ustr::{Ustr, UstrSet};
use crate::ast::evolution::Block;
use crate::ast::object::ObjectOp;
use crate::Evolutions;
use crate::merge::object::State;

pub fn migrate(state: &State, evolutions: &Evolutions) -> State {
    //TODO @mark: maybe UstrSet?
    let mut seen = HashSet::new();
    for evolution in evolutions {
        for block in evolution.blocks {
            match block {
                Block::Obj(obj_ev) => match obj_ev.op {
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
                        seen.insert(Ustr::from(&format!("{}/{}", &obj_ev.identifier, &change_op.name)));
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
