use crate::engine::handler_function::HandlerFunction;
use crate::utils::cpp_adapter::CppString;

#[repr(C)]
pub struct OpSpec {
    pub _unknown1: i32,
    pub name: CppString,
    pub parameters: CppString,
    pub _unknown4: i32,
    pub sub_commands: i32,
    pub _unknown6: CppString,
    pub _unknown7: i32,
    pub parameter_names: CppString,
    pub help_type: i32,
    pub help_string: CppString,
    pub _unknown11: i32,
    pub handler_function: HandlerFunction,
}

impl OpSpec {
    pub fn new(
        name: &str,
        parameters: &str,
        unknown: &str,
        parameter_names: &str,
        help_type: i32,
        help_str: &str,
    ) -> Self {
        Self {
            _unknown1: -1,
            name: CppString::from(name),
            parameters: CppString::from(parameters),
            _unknown4: 0,
            sub_commands: 0,
            _unknown6: CppString::from(unknown),
            _unknown7: -1,
            parameter_names: CppString::from(parameter_names),
            help_type: help_type,
            help_string: CppString::from(help_str),
            _unknown11: -1,
            handler_function: HandlerFunction::empty(),
        }
    }

    pub fn empty() -> Self {
        Self {
            _unknown1: 0,
            name: CppString::empty(),
            parameters: CppString::empty(),
            _unknown4: 0,
            sub_commands: 0,
            _unknown6: CppString::empty(),
            _unknown7: -1,
            parameter_names: CppString::empty(),
            help_type: 0,
            help_string: CppString::empty(),
            _unknown11: -1,
            handler_function: HandlerFunction::empty(),
        }
    }
}
