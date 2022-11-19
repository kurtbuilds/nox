use std::{collections::HashMap};
use serde::{Deserialize, Deserializer};
/// Module resolution mode
///
/// Specify the module resolution strategy: 'node' (Node.js) or 'classic' (used in TypeScript before the release of 1.6). You probably won’t need to use classic in modern code.
/// There is a handbook reference page [on Module Resolution](https://www.typescriptlang.org/docs/handbook/module-resolution.html).
#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ModuleResolutionMode {
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "classic")]
    Classic,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    pub exclude: Option<Vec<String>>,
    pub extends: Option<String>,
    pub files: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub references: Option<References>,
    pub type_acquisition: Option<TypeAcquisition>,
    pub compiler_options: Option<CompilerOptions>,
}

/// These options make up the bulk of TypeScript’s configuration and it covers how the language should work.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompilerOptions {
    pub allow_js: Option<bool>,
    pub check_js: Option<bool>,
    pub composite: Option<bool>,
    pub declaration: Option<bool>,
    pub declaration_map: Option<bool>,
    pub downlevel_iteration: Option<bool>,
    pub import_helpers: Option<bool>,
    pub incremental: Option<bool>,
    pub isolated_modules: Option<bool>,
    pub jsx: Option<Jsx>,
    pub lib: Option<Vec<Lib>>,
    pub module: Option<Module>,
    pub no_emit: Option<bool>,
    pub out_dir: Option<String>,
    pub out_file: Option<String>,
    pub remove_comments: Option<bool>,
    pub root_dir: Option<String>,
    pub source_map: Option<bool>,
    pub target: Option<Target>,
    pub ts_build_info_file: Option<String>,
    pub always_strict: Option<bool>,
    pub no_implicit_any: Option<bool>,
    pub no_implicit_this: Option<bool>,
    pub strict: Option<bool>,
    pub strict_bind_call_apply: Option<bool>,
    pub strict_function_types: Option<bool>,
    pub strict_null_checks: Option<bool>,
    pub strict_property_initialization: Option<bool>,
    pub allow_synthetic_default_imports: Option<bool>,
    pub allow_umd_global_access: Option<bool>,
    pub base_url: Option<String>,
    pub es_module_interop: Option<bool>,
    pub module_resolution: Option<ModuleResolutionMode>,
    pub paths: Option<HashMap<String, Vec<String>>>,
    pub preserve_symlinks: Option<bool>,
    pub root_dirs: Option<Vec<String>>,
    pub type_roots: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    pub inline_source_map: Option<bool>,
    pub inline_sources: Option<bool>,
    pub map_root: Option<String>,
    pub source_root: Option<String>,
    pub no_fallthrough_cases_in_switch: Option<bool>,
    pub no_implicit_returns: Option<bool>,
    pub no_property_access_from_index_signature: Option<bool>,
    pub no_unchecked_indexed_access: Option<bool>,
    pub no_unused_locals: Option<bool>,
    pub emit_decorator_metadata: Option<bool>,
    pub experimental_decorators: Option<bool>,
    pub allow_unreachable_code: Option<bool>,
    pub allow_unused_labels: Option<bool>,
    pub assume_changes_only_affect_direct_dependencies: Option<bool>,
    #[deprecated]
    pub charset: Option<String>,
    pub declaration_dir: Option<String>,
    #[deprecated]
    pub diagnostics: Option<bool>,
    pub disable_referenced_project_load: Option<bool>,
    pub disable_size_limit: Option<bool>,
    pub disable_solution_searching: Option<bool>,
    pub disable_source_of_project_reference_redirect: Option<bool>,
    #[serde(rename = "emitBOM")]
    pub emit_bom: Option<bool>,
    pub emit_declaration_only: Option<bool>,
    pub explain_files: Option<bool>,
    pub extended_diagnostics: Option<bool>,
    pub force_consistent_casing_in_file_names: Option<bool>,
    // XXX: Is generateCpuProfile available from tsconfig? Or just the CLI?
    pub generate_cpu_profile: Option<bool>,

    pub imports_not_used_as_values: Option<String>,
    pub jsx_factory: Option<String>,
    pub jsx_fragment_factory: Option<String>,
    pub jsx_import_source: Option<String>,

    pub keyof_strings_only: Option<bool>,
    pub list_emitted_files: Option<bool>,
    pub list_files: Option<bool>,
    pub max_node_module_js_depth: Option<u32>,
    pub no_emit_helpers: Option<bool>,
    pub no_emit_on_error: Option<bool>,
    pub no_error_truncation: Option<bool>,
    pub no_implicit_use_strict: Option<bool>,
    pub no_lib: Option<bool>,
    pub no_resolve: Option<bool>,
    pub no_strict_generic_checks: Option<bool>,
    #[deprecated]
    pub out: Option<String>,
    pub preserve_const_enums: Option<bool>,
    pub react_namespace: Option<String>,
    pub resolve_json_module: Option<bool>,
    pub skip_default_lib_check: Option<bool>,
    pub skip_lib_check: Option<bool>,
    pub strip_internal: Option<bool>,
    pub suppress_excess_property_errors: Option<bool>,
    pub suppress_implicit_any_index_errors: Option<bool>,
    pub trace_resolution: Option<bool>,
    pub use_define_for_class_fields: Option<bool>,
    pub preserve_watch_output: Option<bool>,
    pub pretty: Option<bool>,
    pub fallback_polling: Option<String>,
    pub watch_directory: Option<String>,
    pub watch_file: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Es3,
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
    Other(String),
}
impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Target::Es5,
            "ES2015" => Target::Es2015,
            "ES6" => Target::Es6,
            "ES2016" => Target::Es2016,
            "ES7" => Target::Es7,
            "ES2017" => Target::Es2017,
            "ES2018" => Target::Es2018,
            "ES2019" => Target::Es2019,
            "ES2020" => Target::Es2020,
            "ESNEXT" => Target::EsNext,
            other => Target::Other(other.to_string()),
        };

        Ok(d)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    CommonJs,
    Es6,
    Es2015,
    Es2020,
    None,
    Umd,
    Amd,
    System,
    EsNext,
    Other(String),
}

impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let r = match s.as_str() {
            "COMMONJS" => Module::CommonJs,
            "ESNEXT" => Module::EsNext,
            "ES6" => Module::Es6,
            "ES2015" => Module::Es2015,
            "ES2020" => Module::Es2020,
            "NONE" => Module::None,
            "UMD" => Module::Umd,
            "AMD" => Module::Amd,
            "SYSTEM" => Module::System,
            other => Module::Other(other.to_string()),
        };

        Ok(r)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Lib {
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
    Dom,
    WebWorker,
    ScriptHost,
    DomIterable,
    Es2015Core,
    Es2015Generator,
    Es2015Iterable,
    Es2015Promise,
    Es2015Proxy,
    Es2015Reflect,
    Es2015Symbol,
    Es2015SymbolWellKnown,
    Es2016ArrayInclude,
    Es2017Object,
    Es2017Intl,
    Es2017SharedMemory,
    Es2017String,
    Es2017TypedArrays,
    Es2018Intl,
    Es2018Promise,
    Es2018RegExp,
    Es2019Array,
    Es2019Object,
    Es2019String,
    Es2019Symbol,
    Es2020String,
    Es2020SymbolWellknown,
    EsNextAsyncIterable,
    EsNextArray,
    EsNextIntl,
    EsNextSymbol,
    Other(String),
}
impl<'de> Deserialize<'de> for Lib {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Lib::Es5,
            "ES2015" => Lib::Es2015,
            "ES6" => Lib::Es6,
            "ES2016" => Lib::Es2016,
            "ES7" => Lib::Es7,
            "ES2017" => Lib::Es2017,
            "ES2018" => Lib::Es2018,
            "ES2019" => Lib::Es2019,
            "ES2020" => Lib::Es2020,
            "ESNext" => Lib::EsNext,
            "DOM" => Lib::Dom,
            "WEBWORKER" => Lib::WebWorker,
            "SCRIPTHOST" => Lib::ScriptHost,
            "DOM.ITERABLE" => Lib::DomIterable,
            "ES2015.CORE" => Lib::Es2015Core,
            "ES2015.GENERATOR" => Lib::Es2015Generator,
            "ES2015.ITERABLE" => Lib::Es2015Iterable,
            "ES2015.PROMISE" => Lib::Es2015Promise,
            "ES2015.PROXY" => Lib::Es2015Proxy,
            "ES2015.REFLECT" => Lib::Es2015Reflect,
            "ES2015.SYMBOL" => Lib::Es2015Symbol,
            "ES2015.SYMBOL.WELLKNOWN" => Lib::Es2015SymbolWellKnown,
            "ES2015.ARRAY.INCLUDE" => Lib::Es2016ArrayInclude,
            "ES2015.OBJECT" => Lib::Es2017Object,
            "ES2017INTL" => Lib::Es2017Intl,
            "ES2015.SHAREDMEMORY" => Lib::Es2017SharedMemory,
            "ES2017.STRING" => Lib::Es2017String,
            "ES2017.TYPEDARRAYS" => Lib::Es2017TypedArrays,
            "ES2018.INTL" => Lib::Es2018Intl,
            "ES2018.PROMISE" => Lib::Es2018Promise,
            "ES2018.REGEXP" => Lib::Es2018RegExp,
            "ES2019.ARRAY" => Lib::Es2019Array,
            "ES2019.OBJECT" => Lib::Es2019Object,
            "ES2019.STRING" => Lib::Es2019String,
            "ES2019.SYMBOL" => Lib::Es2019Symbol,
            "ES2020.STRING" => Lib::Es2020String,
            "ES2020.SYMBOL.WELLKNOWN" => Lib::Es2020SymbolWellknown,
            "ESNEXT.ASYNCITERABLE" => Lib::EsNextAsyncIterable,
            "ESNEXT.ARRAY" => Lib::EsNextArray,
            "ESNEXT.INTL" => Lib::EsNextIntl,
            "ESNEXT.SYMBOL" => Lib::EsNextSymbol,
            other => Lib::Other(other.to_string()),
        };

        Ok(d)
    }
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Jsx {
    /// Emit .js files with JSX changed to the equivalent React.createElement calls
    React,
    /// Emit .js files with the JSX changed to _jsx calls
    ReactJsx,
    /// Emit .js files with the JSX to _jsx calls
    ReactJsxdev,
    /// Emit .js files with the JSX unchanged
    ReactNative,
    /// Emit .jsx files with the JSX unchanged
    Preserve,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TypeAcquisition {
    Bool(bool),
    Object {
        enable: bool,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        disable_filename_based_type_acquisition: Option<bool>,
    },
}

/// Project references setting
///
/// Project references are a way to structure your TypeScript programs into smaller pieces. Using
/// Project References can greatly improve build and editor interaction times, enforce logical separation
/// between components, and organize your code in new and improved ways.
///
/// You can read more about how references works in the Project References section of [the handbook](https://www.typescriptlang.org/docs/handbook/project-references.html).
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum References {
    Bool(bool),
    References(Vec<Reference>),
}


/// Project references setting
///
/// Project references are a way to structure your TypeScript programs into smaller pieces. Using
/// Project References can greatly improve build and editor interaction times, enforce logical separation
/// between components, and organize your code in new and improved ways.
///
/// You can read more about how references works in the Project References section of [the handbook](https://www.typescriptlang.org/docs/handbook/project-references.html).
#[derive(Deserialize, Debug, Clone)]
pub struct Reference {
    pub path: String,
    pub prepend: Option<bool>,
}
