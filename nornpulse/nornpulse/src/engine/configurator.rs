use crate::utils::cpp_adapter::CppString;
use callengine::{call_engine, CheckStructAlign};
use pest::Parser;

#[repr(C, packed)]
#[derive(Debug)]
#[derive(CheckStructAlign)]
pub struct Configurator {
  pub _unknown0: [u8; 20],
  #[check_align(20)] pub some_string: CppString,
  pub _unknown1: [u8; 4],
}

#[derive(Parser)]
#[grammar = "engine/configurator/cfg.pest"]
pub struct CfgParser;