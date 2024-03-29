pub mod patch_strategy;
pub mod map;

use patch_strategy::{GenerateLuaCode, PatchCreatable, WriteAdditional, ProcessText, PatchGroup};

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs;
use std::str;

use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event, BytesEnd, BytesDecl};
use quick_xml::reader::Reader;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyModule, IntoPyDict};

/// Patcher performs all possible strategies for given file.

pub struct Patcher<'a> {
    /// path to file for patching
    path: Option<&'a PathBuf>,
    /// its string representation
    readable: String,
    /// list of creatable tags(keys), value marks this tag replaceable
    creatables: HashMap<String, bool>,
    /// [tag: patch object]
    creatable_patches: HashMap<String, &'a dyn PatchCreatable>,
    /// list of modidiable tags
    modifyables: Vec<String>,
    /// [tag: patch object]
    modifyable_patches: HashMap<String, &'a mut dyn PatchGroup>,
}

impl<'a> Patcher<'a> {
    pub fn new() -> Self {
        Patcher { 
            path: None, 
            readable: String::new(), 
            creatables: HashMap::new(), 
            creatable_patches: HashMap::new(),
            modifyables: vec![],
            modifyable_patches: HashMap::new(),
        }
    }
    /// sets main file for this patcher.
    pub fn with_root(&mut self, root_path: &'a PathBuf) -> Option<&mut Self> {
        self.path = Some(root_path);
        match fs::File::open(root_path) {
            Ok(mut f) => {
                f.read_to_string(&mut self.readable).unwrap();
                fs::remove_file(&root_path).unwrap();
                Some(self)
            }
            Err(e) => {
                println!("Error trying create patcher with root file {:?}: {}", root_path, e.to_string());
                None
            }
        }
    }

    /// adds creatable patch strategy
    pub fn with_creatable(&mut self, label: &str, patch: &'a dyn PatchCreatable, replaceable: bool) -> &mut Self {
        self.creatables.insert(label.to_string(), replaceable);
        self.creatable_patches.insert(label.to_string(), patch);
        self
    }

    /// adds modifiable patch strategy
    pub fn with_modifyables(&mut self, label: &str, group: &'a mut dyn PatchGroup) -> &mut Self {
        self.modifyables.push(label.to_string());
        self.modifyable_patches.insert(label.to_string(), group);
        self
    }

    /// executes patch process and writes patched info when it is done.
    pub fn run(&mut self) {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer::new(&mut output);
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None))).unwrap();
        self.process(&mut writer);
        let mut out_file = fs::File::create(self.path.unwrap()).unwrap();
        out_file.write_all(&output).unwrap();
    }

    /// reads all tags in given xml file.
    /// if finds tag that is creatable/modifiable then applies necessary patches to its content
    fn process(&mut self, writer: &mut Writer<&mut Vec<u8>>) {
        let mut buf = Vec::new();
        let mut reader = Reader::from_str(&self.readable);
        reader.trim_text(true);
        reader.expand_empty_elements(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    // gets actual name of tag
                    let actual_tag = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    if self.creatables.contains_key(&actual_tag) {
                        if *self.creatables.get(&actual_tag).unwrap() == true {
                            reader.read_to_end(e.to_end().name()).unwrap();
                        }
                        //println!("creatable tag found: {}", &actual_tag);
                        let actual_strategy = self.creatable_patches.get_mut(&actual_tag).unwrap();
                        actual_strategy.try_create(writer);
                    }
                    else if self.modifyables.contains(&actual_tag) {
                        //println!("modifyable tag found: {}", &actual_tag);
                        let end = e.to_end().into_owned();
                        let text = reader.read_text(end.name()).unwrap().to_string();
                        let strategy = self.modifyable_patches.get_mut(&actual_tag).unwrap();
                        strategy.run(&text, writer);
                    }
                    else {
                        let mut elem = BytesStart::new(str::from_utf8(e.name().0).unwrap());
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        writer.write_event(Event::Start(elem)).unwrap();
                    }
                }
                Ok(Event::Text(e)) => {
                    writer.write_event(Event::Text(e)).unwrap();
                },
                Ok(Event::End(e)) => {
                    let elem = BytesEnd::new(str::from_utf8(e.name().0).unwrap());
                    writer.write_event(Event::End(elem)).unwrap();
                },
                _ => ()
            }
            buf.clear();
        }
    }
}

/// Modifies text file of map.
/// This is needed cause of stupid encoding of these files so i'm using this python solution here(couldnt find right way to output utf16-le in Rust)
pub struct TextProcessor<'a> {
    path: Option<&'a PathBuf>,
    processors: Vec<&'a dyn ProcessText>
}

impl<'a> TextProcessor<'a> {
    pub fn new(path: &'a PathBuf) -> Self {
        TextProcessor { 
            path: Some(path), 
            processors: vec![] 
        }
    }

    /// adds new processor
    pub fn with(&mut self, processor: &'a dyn ProcessText) -> &mut Self {
        self.processors.push(processor);
        self
    }

    /// apply all processors to text
    pub fn run(&self) {
        let file = fs::File::open(self.path.unwrap()).unwrap();
        let mut text = utf16_reader::read_to_string(file);
        fs::remove_file(self.path.unwrap()).unwrap();
        for processor in &self.processors {
            text = processor.try_process(&mut text);
        }
        let code = 
        "def save_utf16file(*args, **kwargs):
            with open(kwargs['path'], encoding='utf16', mode='w') as out:
                out.write(kwargs['text'])";
        fs::File::create(self.path.unwrap()).unwrap();
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(py, &code, "", "")
                        .unwrap().getattr("save_utf16file").unwrap().into();
            let kwargs = vec![
                ("path", self.path.unwrap().to_str().unwrap()), 
                ("text", &text)
            ];
            fun.call(py, (), Some(kwargs.into_py_dict(py))).unwrap();
        });
    }
}

/// Generates additional lua files.
pub struct CodeGenerator<'a> {
    code_generators: Vec<&'a dyn GenerateLuaCode>
}

impl<'a> CodeGenerator<'a> {
    pub fn new() -> Self {
        CodeGenerator { 
            code_generators: vec![] 
        }
    }

    pub fn with(&mut self, gen: &'a dyn GenerateLuaCode) -> &mut Self {
        self.code_generators.push(gen);
        self
    }

    pub fn run(&mut self, base_path: &PathBuf) {
        for generator in &self.code_generators {
            generator.to_lua(base_path);
        }
    }
}

/// Puts additional files to map(from configs mostly)
pub struct FileWriter<'a> {
    file_writers: Vec<&'a dyn WriteAdditional>
}

impl<'a> FileWriter<'a> {
    pub fn new() -> Self {
        FileWriter { 
            file_writers: vec![]
        }
    }

    pub fn with(&mut self, writer: &'a dyn WriteAdditional) -> &mut Self {
        self.file_writers.push(writer);
        self
    }

    pub fn run(&mut self) {
        for writer in &self.file_writers {
            writer.try_write();
        }
    }
}