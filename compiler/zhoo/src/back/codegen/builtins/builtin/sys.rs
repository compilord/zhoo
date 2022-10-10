use crate::back::codegen::builtins::builtin::{Builtin, Proto};
use crate::front::parser::tree::ty::Ty;

pub fn sys_builtins() -> Vec<Builtin> {
  vec![
    Builtin::new(String::from("exit"), Proto(vec![Ty::INT], Ty::VOID)),
    // FIXME: `Undefined symbols for architecture x86_64: _create", referenced from: _main in main.o`
    Builtin::new(
      String::from("create"),
      Proto(vec![Ty::STR, Ty::STR], Ty::VOID),
    ),
    // FIXME: `dyld: BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB has segment 1 which is not writable`
    Builtin::new(String::from("open"), Proto(vec![Ty::STR], Ty::STR)),
  ]
}
