use std::collections::BTreeMap;

type EnginesSet = BTreeMap<String, String>;

/// A bug contacting form.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Bug {
    /// The email to use for contact.
    pub email: Option<String>,
    /// The url to use to submit bugs.
    pub url: Option<String>,
}


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// The package name.
    pub name: String,
    /// The package version.
    pub version: String,
    /// The optional package description.
    pub description: Option<String>,
    /// The optional list of keywords.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// The optional package homepage.
    pub homepage: Option<String>,
    /// The optional bug contact form.
    pub bugs: Option<Bug>,
    /// The optional package license.
    pub license: Option<String>,
    /// The optional author.
    pub author: Option<PersonReference>,
    /// The optional list of contributors.
    #[serde(default)]
    pub contributors: Vec<PersonReference>,
    /// The optional list of files to include. Each entry defines a regex
    /// pattern.
    #[serde(default)]
    pub files: Vec<String>,
    /// The optional package main entry file.
    pub main: Option<String>,
    /// The optional package browser entry file.
    ///
    /// This is usually defined in libraries that are meant to be consumed by
    /// browsers. These Thoes can refer to objects that are not available inside
    /// a `nodejs` environment (like `window`).
    pub browser: Option<String>,
    /// The optional set of binary definitions.
    #[serde(default)]
    pub bin: BinSet,
    /// The optional list of man page references.
    pub man: Option<ManReference>,
    /// The optional repository reference.
    //#[serde(flatten)]
    pub repository: Option<RepositoryReference>,
    /// The optional list of script entries.
    #[serde(default)]
    pub scripts: ScriptsSet,
    /// The optional list of dependencies.
    #[serde(default)]
    pub dependencies: DepsSet,
    /// The optional list of development dependencies.
    #[serde(default)]
    pub dev_dependencies: DepsSet,
    /// The optional list of peer dependencies.
    #[serde(default)]
    pub peer_dependencies: DepsSet,
    /// The optional list of bundled dependencies.
    #[serde(default)]
    pub bundled_dependencies: DepsSet,
    /// The optional list of optional dependencies.
    #[serde(default)]
    pub optional_dependencies: DepsSet,
    /// The optional list of engine entries.
    #[serde(default)]
    pub engines: EnginesSet,
    /// The package privacy.
    #[serde(default)]
    pub private: bool,
    /// The OS' that the package can run on.
    #[serde(default)]
    pub os: Vec<String>,
    /// The CPU architectures that the package can run on.
    #[serde(default)]
    pub cpu: Vec<String>,
    /// The optional config object.
    pub config: Option<Value>,
    /// Other custom fields that have been defined inside the `package.json`
    /// file.
    #[serde(flatten)]
    pub others: BTreeMap<String, Value>,
}