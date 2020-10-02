use crate::utils::cpp_adapter::CppString;
use callengine::{call_engine, CheckStructAlign};
pub use injected_calls::inject_calls;
use pest::Parser;
use std::io::Write;

mod injected_calls;

#[derive(Parser)]
#[grammar = "engine/configurator/cfg.pest"]
pub struct CfgParser;

type StringMap = std::collections::HashMap<String, String>;

#[derive(Debug)]
pub struct Configurator {
    _vptr: [u8; 4],
    config: Box<StringMap>,
    file_name: Box<String>,
}

fn quote_write(s: &str, file: &mut impl Write) -> std::io::Result<()> {
    if s.contains(' ') {
        file.write_all(b"\"")?;
        file.write_all(s.as_bytes())?;
        file.write_all(b"\"")?;
    } else {
        file.write_all(s.as_bytes())?;
    }
    Ok(())
}

impl Configurator {
    pub fn bind_to_file(&mut self, file_name: &str) -> std::io::Result<()> {
        *self.file_name = file_name.to_string();
        self.load();
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            _vptr: Default::default(),
            config: Box::new(StringMap::new()),
            file_name: Box::new(String::new()),
        }
    }

    pub fn from(file_name: &str) -> Self {
        let mut me = Self {
            _vptr: Default::default(),
            config: Box::new(StringMap::new()),
            file_name: Box::new(file_name.to_string()),
        };
        me.load();
        me
    }

    pub fn exists(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let file = std::fs::File::create(&*self.file_name)?;
        let mut file = std::io::LineWriter::new(file);
        for (k, v) in &*self.config {
            quote_write(k, &mut file)?;
            file.write_all(b" ")?;
            quote_write(v, &mut file)?;
            file.write_all(b"\n")?;
        }
        file.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|k| k.as_str())
    }

    fn load(&mut self) -> bool {
        let unparsed_file = std::fs::read_to_string(&*self.file_name).unwrap();
        let parsed = CfgParser::parse(Rule::file, &unparsed_file).unwrap();
        for record in parsed {
            if let Rule::record = record.as_rule() {
                let mut iter = record.into_inner();
                let a = snailquote::unescape(iter.next().unwrap().as_str());
                let b = snailquote::unescape(iter.next().unwrap().as_str());

                if a.is_err() {
                    let a = a.unwrap_err();
                    log::debug!("{}", a.to_string());
                    continue;
                }

                if b.is_err() {
                    let b = b.unwrap_err();
                    log::debug!("{}", b.to_string());
                    continue;
                }

                self.config
                    .insert(a.unwrap().to_string(), b.unwrap().to_string());
            }
        }
        true
    }

    pub fn set(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }
}

impl Drop for Configurator {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
