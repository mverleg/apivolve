//! The generating executable should emit [GenerateConfig] as json on a single line of stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in the desired format on its stdin.

use ::std::borrow::Cow;
use ::std::env;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::io::{BufReader, Read, Write};
use ::std::io::BufRead;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::{Child, Stdio};
use ::std::process::Command;
use ::std::rc::Rc;
use ::std::thread;
use ::std::vec::IntoIter;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use ::lazy_static::lazy_static;
use ::log::debug;
use ::log::info;
use ::regex::Regex;
use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::which::which;
use ::which::which_re;

use crate::{ApivResult, FullEvolution, load_dir};

const GEN_NAME_PREFIX: &str = "apivolve-gen1-";

lazy_static! {
    static ref GEN_NAME_RE: Regex = Regex::new(&format!("^{}.*", GEN_NAME_PREFIX)).unwrap();
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GenerateInputLayout {
    /// The complete data layout per version.
    Layout,
    /// The steps to be taken to parse and generate input per version.
    Steps,
}

impl fmt::Display for GenerateInputLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputLayout::Layout => write!(f, "layout"),
            GenerateInputLayout::Steps => write!(f, "steps"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GenerateInputFormat {
    Json,
}

impl fmt::Display for GenerateInputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputFormat::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateConfig {
    apivolve_version: Version,
    data_structure: GenerateInputLayout,
    encoding: GenerateInputFormat,
}

impl GenerateConfig {
    pub fn new(apivolve_version: Version, data_structure: GenerateInputLayout, encoding: GenerateInputFormat) -> Self {
        GenerateConfig { apivolve_version, data_structure, encoding }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateChangesInput {

}

impl From<&FullEvolution> for GenerateChangesInput {
    fn from(evolutions: &FullEvolution) -> Self {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    name: String,
    path: PathBuf,
}

impl Generator {
    pub fn from_path(path: PathBuf) -> Self {
        let full_name = path.file_name().expect("no filename").to_str().expect("filename is not unicode");
        debug_assert!(full_name.starts_with(GEN_NAME_PREFIX));
        let name = full_name[GEN_NAME_PREFIX.len()..].to_owned();
        Generator { name, path }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Generators {
    generators: Vec<Generator>,
}

pub async fn apivolve_list_generators() -> ApivResult<Generators> {
    find_all_generators()
}

impl IntoIterator for Generators {
    type Item = Generator;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.generators.into_iter()
    }
}

impl fmt::Display for Generators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for gen in &self.generators {
            writeln!(f, "{} (@ {})", gen.name, gen.path.to_string_lossy());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResult {
    generators: Vec<Generator>,
}

impl fmt::Display for GenerateResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: &[String]) -> ApivResult<GenerateResult> {
    assert!(!targets.is_empty());
    if targets.is_empty() {
        return Err("Need at least one target to generate".to_owned())
    }
    let evolutions = Arc::new(load_dir(evolution_dir)?);
    let mut threads = vec![];
    for generator in find_target_generators(&targets)?.into_iter() {
        debug!("building generator {} (at {})", generator.name(), generator.path().to_string_lossy());
        let name = generator.name.clone();
        let handler = GeneratorHandler::new(generator, evolutions.clone())?;
        let (sender, receiver) = mpsc::channel();
        let join_handle = thread::Builder::new()
            .name(format!("generate-{}", &handler.generator.name))
            .spawn(move || {
                let target_name = handler.generator.name.clone();
                debug!("starting generator {}", &target_name);
                let res = handler.run();
                debug!("sending {} result for {} generator", if res.is_ok() { "successful" } else { "FAILED" }, &target_name);
                sender.send(res);
                debug!("finished generator {}", &target_name);
            }).unwrap();
        threads.push((name, join_handle, receiver));
    }
    debug!("waiting for {} generators", targets.len());
    for (name, thread, receiver) in threads {
        let res = receiver.recv_timeout(Duration::from_secs(60))
            .map_err(|err| format!("failed to get result from generator thread"))??;
        thread.join();
    }
    info!("all {} generators done", targets.len());
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

fn encode_evolution_changes(input_format: GenerateInputFormat, evolutions: &FullEvolution) -> ApivResult<Vec<u8>> {
    //TODO @mark: create a cache?
    //TODO @mark: can serde directly write to buffer, instead of allocating the whole thing?
    let changes = GenerateChangesInput::from(evolutions);
    Ok(match input_format {
        GenerateInputFormat::Json => serde_json::to_string(&changes)
            .map_err(|err| format!("failed to convert evolutions to json; generator {}, err {}", input_format, err))?.into_bytes(),
    })
}

#[derive(Debug)]
struct GeneratorHandler {
    generator: Generator,
    evolutions: Arc<FullEvolution>,
    proc: Child,
}

impl GeneratorHandler {
    fn new(generator: Generator, evolutions: Arc<FullEvolution>) -> ApivResult<Self> {
        let proc = Command::new(generator.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        Ok(GeneratorHandler {
            generator,
            evolutions,
            proc,
        })
    }

    fn run(self) -> ApivResult<()> {
        let GeneratorHandler { generator, evolutions, mut proc } = self;

        let mut buffer = String::with_capacity(512);
        let mut reader = BufReader::new(proc.stdout.as_mut().expect(&format!("failed to read from generator {}", &generator.name)));
        reader.read_line(&mut buffer)
            .map_err(|err| format!("failed to read config (first line) from {} generator; err {}", &generator.name, err))?;
        debug!("received generator config: {}", buffer.trim_end());
        let config: GenerateConfig = serde_json::from_str(&buffer)
            .map_err(|err| format!("failed to parse config (first line) from {} generator; got {}; err {}", &generator.name, buffer.trim_end(), err))?;

        let data = encode_evolution_changes(config.encoding, &*evolutions)?;
        let len = proc.stdin.expect("failed to send to generator").write(&data)
            .expect(&format!("failed to write evolutions to generator {}", &generator.name));
        assert_eq!(len, data.len());

        Ok(())
    }
}

fn find_all_generators() -> ApivResult<Generators> {
    debug!("PATH = {}", env::var("PATH").unwrap_or("".to_owned()));
    let generators = which_re(&*GEN_NAME_RE).unwrap()
        .map(Generator::from_path)
        .collect::<Vec<_>>();
    if generators.is_empty() {
        return Err("no generators found on $PATH".to_owned());
    }
    Ok(Generators {
        generators,
    })
}

fn find_target_generators(targets: &[String]) -> ApivResult<Generators> {
    debug!("PATH = {}", env::var("PATH").unwrap_or("".to_owned()));
    Ok(Generators {
        generators: targets.iter()
            .map(|target| {
                let gen_name = format!("{}{}", GEN_NAME_PREFIX, &target);
                match which(&gen_name) {
                    Ok(path) => Ok(Generator { name: target.to_owned(), path }),
                    Err(_) => Err(format!("failed to find executable '{}' for target '{}' in $PATH; use 'gen list' to find available targets", gen_name, target)),
                }
            })
            .collect::<ApivResult<Vec<_>>>()?
    })
}

#[cfg(test)]
mod tests {
    use ::std::str::FromStr;

    use crate::ast::evolution::Block;
    use crate::ast::object::{FieldOp, ObjectAdd, ObjectOp};
    use crate::ast::Span;
    use crate::ast::term::Iden;
    use crate::load::evolution::Evolution;

    use super::*;

    #[test]
    fn serialization_compatibility_generate_config() {
        let json = serde_json::to_string(&GenerateConfig {
            apivolve_version: Version::new(1, 2, 4),
            data_structure: GenerateInputLayout::Steps,
            encoding: GenerateInputFormat::Json,
        }).unwrap();
        assert_eq!(json, "{\"apivolve_version\":\"1.2.4\",\"data_structure\":\"Steps\",\"encoding\":\"Json\"}");
    }

    #[test]
    fn serialization_compatibility_generators() {
        let json = serde_json::to_string(&Generators {
            generators: vec![
                Generator { name: "test-cmd".to_string(), path: PathBuf::from_str("/path/apivolve-gen1-test-cmd").unwrap() },
            ]
        }).unwrap();
        assert_eq!(json, "[{\"name\":\"test-cmd\",\"path\":\"/path/apivolve-gen1-test-cmd\"}]");
    }

    #[test]
    fn serialization_compatibility_generate_result() {
        let json = serde_json::to_string(&GenerateResult {
            generators: vec![
                Generator { name: "test-cmd".to_string(), path: PathBuf::from_str("/path/apivolve-gen1-test-cmd").unwrap() },
            ]
        }).unwrap();
        assert_eq!(json, "{\"generators\":[{\"name\":\"test-cmd\",\"path\":\"/path/apivolve-gen1-test-cmd\"}]}");
    }
}
