use std::collections::HashSet;
use ustr::UstrSet;
use crate::ast::evolution::Block;
use crate::ast::object::ObjectOp;
use crate::Evolutions;
use crate::merge::object::State;

pub fn migrate(state: &State, evolutions: &Evolutions) -> State {
    let mut seen = HashSet::new();
    for evolution in evolutions {
        for block in evolution.blocks {
            match block {
                Block::Obj(obj_op) => match obj_op {
                    ObjectOp::Add(add_op) => {
                        seen.insert()
                    }
                    ObjectOp::Change(change_op) => {}
                    ObjectOp::Delete(delete_op) => {}
                }
            }
        }
    }
    todo!()
}
