use std::io::Write;
use std::{env::VarError, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::IDENT;

const LUAJIT_FFI_GEN_DIR_ENV: &str = "LUAJIT_FFI_GEN_DIR";
const LUAJIT_FFI_GEN_DIR: &str = "../phx/script/ffi_gen";

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TypeDecl {
    #[default]
    NoDecl,
    Opaque,
    Transparent(String),
}

impl TypeDecl {
    fn id(&self) -> u8 {
        match self {
            TypeDecl::NoDecl => 0,
            TypeDecl::Opaque => 1,
            TypeDecl::Transparent(_) => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FfiGenerator {
    module_name: String,
    type_decl: TypeDecl,
    c_definitions: Vec<String>,
    global_symbol_table: Vec<String>,
    is_mt_clone: bool,
    to_string_method: Option<String>,
    metatype: Vec<String>,
}

/// Initialization
impl FfiGenerator {
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.into(),
            type_decl: Default::default(),
            c_definitions: Default::default(),
            global_symbol_table: Default::default(),
            is_mt_clone: Default::default(),
            to_string_method: Default::default(),
            metatype: Default::default(),
        }
    }

    pub fn set_type_decl_opaque(&mut self) {
        self.type_decl = TypeDecl::Opaque;
    }

    pub fn set_type_decl_transparent(&mut self, ty: impl Into<String>) {
        self.type_decl = TypeDecl::Transparent(ty.into());
    }

    pub fn has_type_decl(&self) -> bool {
        !matches!(self.type_decl, TypeDecl::NoDecl)
    }

    pub fn add_c_definition(&mut self, value: impl Into<String>) {
        self.c_definitions.push(value.into());
    }

    pub fn has_c_definitions(&self) -> bool {
        !self.c_definitions.is_empty()
    }

    pub fn add_global_symbol(&mut self, value: impl Into<String>) {
        self.global_symbol_table.push(value.into());
    }

    pub fn has_global_symbols(&self) -> bool {
        !self.global_symbol_table.is_empty()
    }

    pub fn set_mt_clone(&mut self) {
        self.is_mt_clone = true;
    }

    pub fn set_to_string_method(&mut self, method: &str) {
        self.to_string_method = Some(method.into());
    }

    pub fn add_metatype(&mut self, value: impl Into<String>) {
        self.metatype.push(value.into());
    }
}

/// Serialization and deserialization
impl FfiGenerator {
    pub fn load(module_name: &str) -> Self {
        let target_ffi_file = Self::ffi_file(module_name);

        if !target_ffi_file.exists() {
            return Self::new(module_name);
        }

        let data = std::fs::read_to_string(&target_ffi_file)
            .expect(&format!("Cannot load {target_ffi_file:?} FFI data file"));

        let res: Self = serde_json::from_str(&data)
            .expect(&format!("Cannot parse {target_ffi_file:?} FFI data file"));

        std::fs::remove_file(&target_ffi_file)
            .expect(&format!("Cannot remove {target_ffi_file:?} FFI data file"));

        assert_eq!(res.module_name, module_name);

        res
    }

    pub fn save(&self, module_name: &str) {
        let data =
            serde_json::to_string(self).expect(&format!("Cannot serialize {module_name} data"));

        let target_ffi_dir = Self::ffi_dir();

        std::fs::create_dir_all(&target_ffi_dir)
            .expect(&format!("Cannot create {target_ffi_dir:?} folder"));

        let target_ffi_file = Self::ffi_file(module_name);

        std::fs::write(&target_ffi_file, data)
            .expect(&format!("Cannot save {target_ffi_file:?} FFI data file"));
    }

    fn ffi_dir() -> PathBuf {
        // TODO: env!("OUT_DIR") doesn't work
        PathBuf::new().join("target").join("ffi")
    }

    fn ffi_file(module_name: &str) -> PathBuf {
        Self::ffi_dir().join(format!("{module_name}.json"))
    }
}

/// Generation
impl FfiGenerator {
    pub fn generate(&self) {
        let luajit_ffi_gen_dir = match std::env::var(LUAJIT_FFI_GEN_DIR_ENV) {
            Ok(var) => {
                if !var.is_empty() {
                    var
                } else {
                    LUAJIT_FFI_GEN_DIR.into()
                }
            }
            Err(VarError::NotPresent) => LUAJIT_FFI_GEN_DIR.into(),
            Err(err) => {
                println!("Cannot read '{LUAJIT_FFI_GEN_DIR_ENV}' environment variable. Use default value: {LUAJIT_FFI_GEN_DIR}. Error: {err}");

                LUAJIT_FFI_GEN_DIR.into()
            }
        };

        let cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let luajit_ffi_gen_dir_path = cargo_manifest_dir.join(&luajit_ffi_gen_dir);
        assert!(
            luajit_ffi_gen_dir_path.exists(),
            "FFI directory '{luajit_ffi_gen_dir_path:?}' doesn't exist"
        );

        let luajit_ffi_module_path =
            luajit_ffi_gen_dir_path.join(format!("{}.lua", self.module_name));
        let mut file = File::create(&luajit_ffi_module_path).expect(&format!(
            "Cannot create file: {luajit_ffi_module_path:?}\nCurrent folder: {:?}",
            std::env::current_dir()
        ));

        // Header
        writeln!(
            &mut file,
            "-- {} {:-<2$}",
            self.module_name,
            "-",
            80 - 4 - self.module_name.len()
        )
        .unwrap();
        writeln!(&mut file, "local ffi = require('ffi')").unwrap();
        writeln!(&mut file, "local libphx = require('libphx').lib").unwrap();
        writeln!(&mut file, "local {}\n", self.module_name).unwrap();

        // Type declaration
        writeln!(&mut file, "function declareType()").unwrap();

        match &self.type_decl {
            TypeDecl::NoDecl => {}
            TypeDecl::Opaque => {
                writeln!(&mut file, "{IDENT}ffi.cdef [[").unwrap();
                writeln!(
                    &mut file,
                    "{IDENT}{IDENT}typedef struct {0} {{}} {0};",
                    self.module_name
                )
                .unwrap();
                writeln!(&mut file, "{IDENT}]]\n").unwrap();
            }
            TypeDecl::Transparent(ty) => {
                writeln!(&mut file, "{IDENT}ffi.cdef [[").unwrap();
                writeln!(
                    &mut file,
                    "{IDENT}{IDENT}typedef {ty} {};",
                    self.module_name
                )
                .unwrap();
                writeln!(&mut file, "{IDENT}]]\n").unwrap();
            }
        }
        writeln!(
            &mut file,
            "{IDENT}return {}, '{}'",
            self.type_decl.id(),
            self.module_name
        )
        .unwrap();

        writeln!(&mut file, "end\n").unwrap();

        // C Definitions
        writeln!(&mut file, "do -- C Definitions").unwrap();
        writeln!(&mut file, "{IDENT}ffi.cdef [[").unwrap();

        self.c_definitions
            .iter()
            .for_each(|def| writeln!(&mut file, "{def}").unwrap());

        writeln!(&mut file, "{IDENT}]]").unwrap();
        writeln!(&mut file, "end\n").unwrap();

        // Global Symbol Table
        writeln!(&mut file, "do -- Global Symbol Table").unwrap();
        writeln!(&mut file, "{IDENT}{} = {{", self.module_name).unwrap();

        self.global_symbol_table
            .iter()
            .for_each(|def| writeln!(&mut file, "{def}").unwrap());

        writeln!(&mut file, "{IDENT}}}\n").unwrap();

        if self.is_mt_clone {
            writeln!(&mut file, "{IDENT}local mt = {{").unwrap();
            writeln!(
                &mut file,
                "{IDENT}{IDENT}__call = function(t, ...) return {}_t(...) end,",
                self.module_name
            )
            .unwrap();
            writeln!(&mut file, "{IDENT}}}\n").unwrap();
        }

        writeln!(
            &mut file,
            "{IDENT}if onDef_{0} then onDef_{0}({0}, mt) end",
            self.module_name
        )
        .unwrap();
        writeln!(
            &mut file,
            "{IDENT}{0} = setmetatable({0}, mt)",
            self.module_name
        )
        .unwrap();
        writeln!(&mut file, "end\n").unwrap();

        // Metatype for class instances
        if self.to_string_method.is_some() || !self.metatype.is_empty() {
            writeln!(&mut file, "do -- Metatype for class instances").unwrap();
            writeln!(
                &mut file,
                "{IDENT}local t  = ffi.typeof('{}')",
                self.module_name
            )
            .unwrap();
            writeln!(&mut file, "{IDENT}local mt = {{").unwrap();

            if let Some(method) = &self.to_string_method {
                writeln!(
                    &mut file,
                    "{IDENT}{IDENT}__tostring = function(self) return ffi.string(libphx.{}_{method}(self)) end,",
                    self.module_name,
                )
                .unwrap();
            }

            writeln!(&mut file, "{IDENT}{IDENT}__index = {{").unwrap();

            self.metatype
                .iter()
                .for_each(|mt| writeln!(&mut file, "{mt}").unwrap());

            writeln!(&mut file, "{IDENT}{IDENT}}},").unwrap();
            writeln!(&mut file, "{IDENT}}}\n").unwrap();

            writeln!(
                &mut file,
                "{IDENT}if onDef_{0}_t then onDef_{0}_t(t, mt) end",
                self.module_name
            )
            .unwrap();
            writeln!(
                &mut file,
                "{IDENT}{}_t = ffi.metatype(t, mt)",
                self.module_name
            )
            .unwrap();
            writeln!(&mut file, "end\n").unwrap();
        }

        writeln!(&mut file, "return {}", self.module_name).unwrap();
    }
}
