use anyhow::Context;
use grpc::proto_types::{BindRequest, LoadDriverRequest, LoginRequest};
use shelgon::{command, renderer};

mod config;
mod grpc;

struct FinShellExecutor {}

#[derive(Debug, Clone)]
enum Command {
    Insmod {
        driver_name: String,
        driver_version: String,
        driver_fn: String,
    },
    Rmmod {
        driver_name: String,
        driver_version: String,
    },
    Mount {
        driver_name: String,
        driver_version: String,
        path: String,
    },
    Cd(String),
    Ls,
    LsMod,
    Pwd,
    Clear,
    Exit,
    Help,
    Stat(String),
    Cat(String),
    Unknown(String),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.first().copied() {
            Some("cd") => Command::Cd(parts.get(1).unwrap_or(&"").to_string()),
            Some("ls") => Command::Ls,
            Some("lsdriver") => Command::LsMod,
            Some("pwd") => Command::Pwd,
            Some("clear") => Command::Clear,
            Some("exit") => Command::Exit,
            Some("help") => Command::Help,
            Some("stat") => Command::Stat(parts.get(1).unwrap_or(&"").to_string()),
            Some("cat") => Command::Cat(parts.get(1).unwrap_or(&"").to_string()),
            Some("insdriver") => {
                let driver_name = parts.get(1).unwrap_or(&"").to_string();
                let driver_version = parts.get(2).unwrap_or(&"").to_string();
                let driver_fn = parts.get(3).unwrap_or(&"").to_string();
                Command::Insmod {
                    driver_name,
                    driver_version,
                    driver_fn,
                }
            }
            Some("rmdriver") => {
                let driver_name = parts.get(1).unwrap_or(&"").to_string();
                let driver_version = parts.get(2).unwrap_or(&"").to_string();
                Command::Rmmod {
                    driver_name,
                    driver_version,
                }
            }
            Some("link") => {
                let driver_name = parts.get(1).unwrap_or(&"").to_string();
                let driver_version = parts.get(2).unwrap_or(&"").to_string();
                let path = parts.get(3).unwrap_or(&"").to_string();
                Command::Mount {
                    driver_name,
                    driver_version,
                    path,
                }
            }
            Some(cmd) => Command::Unknown(cmd.to_string()),
            None => Command::Unknown("".to_string()),
        }
    }
}

struct ShellContext {
    current_dir: String,
    client: grpc::Clients,
    user_token: String,
}

impl command::Execute for FinShellExecutor {
    type Context = ShellContext;

    fn prompt(&self, ctx: &Self::Context) -> String {
        format!("{}$", ctx.current_dir)
    }

    fn execute(
        &self,
        ctx: &mut Self::Context,
        cmd: command::CommandInput,
    ) -> anyhow::Result<command::OutputAction> {
        let command = Command::from(cmd.command.as_str());
        let rt = tokio::runtime::Runtime::new()?;

        let output = match command {
            Command::Cd(path) => {
                #[allow(unused_assignments)]
                let mut new_dir = ctx.current_dir.clone();
                if path.starts_with('/') {
                    new_dir = path.clone();
                } else {
                    let mut new_path = std::path::PathBuf::from(&ctx.current_dir);
                    new_path.push(&path);
                    new_dir = new_path.to_string_lossy().to_string();
                }

                let paths = rt
                    .block_on(
                        ctx.client
                            .driver_client
                            .list_resolver(grpc::proto_types::ListResolverRequest {}),
                    )?
                    .into_inner();

                let paths = paths
                    .path_mapping
                    .iter()
                    .map(|inner| &inner.path)
                    .filter(|p| p.starts_with(new_dir.as_str()))
                    .map(|p| p.strip_prefix(new_dir.as_str()).unwrap_or(p).to_string())
                    .collect::<Vec<String>>();

                if paths.is_empty() {
                    vec![format!("cd: no such file or directory: {}", path)]
                } else {
                    if new_dir == "/" {
                        ctx.current_dir = new_dir;
                    } else {
                        ctx.current_dir = format!("{}/", new_dir);
                    }
                    vec![String::new()]
                }
            }
            Command::Ls => {
                let paths = rt.block_on(
                    ctx.client
                        .driver_client
                        .list_resolver(grpc::proto_types::ListResolverRequest {}),
                );

                let paths = match paths {
                    Ok(p) => p.into_inner(),
                    Err(e) => {
                        return Ok(command::OutputAction::Command(command::CommandOutput {
                            prompt: cmd.prompt,
                            command: cmd.command,
                            stdin: cmd.stdin.unwrap_or_default(),
                            stdout: Vec::new(),
                            stderr: vec![format!("ls: cannot access '{}': {}", ctx.current_dir, e)],
                        }))
                    }
                };

                paths
                    .path_mapping
                    .iter()
                    .filter(|p| p.path.starts_with(ctx.current_dir.as_str()))
                    .map(|p| {
                        let path = p
                            .path
                            .strip_prefix(ctx.current_dir.as_str())
                            .unwrap_or(&p.path)
                            .to_string();
                        format!("{}\t{}@{}", path, p.driver_name, p.driver_version)
                    })
                    .collect()
            }
            Command::LsMod => {
                let output = rt
                    .block_on(
                        ctx.client
                            .driver_detail_client
                            .send_details(grpc::proto_types::DriverDetailsRequest {}),
                    )?
                    .into_inner();

                output
                    .driver_data
                    .iter()
                    .map(|d| format!("{}@{}", d.name, d.version))
                    .collect()
            }
            Command::Help => {
                vec![
                    "Available commands:".to_string(),
                    "  cat <path>                                        Display the contents of a file".to_string(),
                    "  cd <path>                                         Change the current directory".to_string(),
                    "  clear                                             Clear the screen".to_string(),
                    "  exit                                              Exit the shell".to_string(),
                    "  help                                              Display this help text".to_string(),
                    "  insdriver <driver_name> <driver_version> <driver_fn> Load a driver".to_string(),
                    "  ls                                                List files in the current directory".to_string(),
                    "  lsdriver                                             List loaded kernel modules".to_string(),
                    "  link <driver_name> <driver_version> <path>       Bind a driver".to_string(),
                    "  pwd                                               Print the current working directory".to_string(),
                    "  rmdriver <driver_name> <driver_version>              Unload a driver".to_string(),
                    "  stat <path>                                       Display detailed information about a file".to_string(),
                ]
            }
            Command::Pwd => vec![ctx.current_dir.to_string()],
            Command::Clear => return Ok(shelgon::OutputAction::Clear),
            Command::Exit => return Ok(shelgon::OutputAction::Exit),
            Command::Unknown(cmd) => vec![format!("shell: command not found: {}", cmd)],
            Command::Stat(name) => {
                let path = format!("{}{}", ctx.current_dir, name);
                let paths = rt
                    .block_on(
                        ctx.client
                            .driver_client
                            .list_resolver(grpc::proto_types::ListResolverRequest {}),
                    )?
                    .into_inner()
                    .path_mapping
                    .into_iter()
                    .find(|f| f.path == path);

                match paths {
                    None => vec![format!(
                        "stat: cannot stat '{}': No such file or directory",
                        name
                    )],
                    Some(p) => vec![
                        format!("path: {}", p.path),
                        format!("driver: {}", p.driver_name),
                        format!("version: {}", p.driver_version),
                    ],
                }
            }
            Command::Cat(name) => {
                let path = format!("{}{}", ctx.current_dir, name);
                let paths = rt
                    .block_on(
                        ctx.client
                            .driver_client
                            .list_resolver(grpc::proto_types::ListResolverRequest {}),
                    )?
                    .into_inner()
                    .path_mapping
                    .into_iter()
                    .find(|f| f.path == path);

                match paths {
                    None => vec![format!(
                        "cat: cannot read '{}': No such file or directory",
                        name
                    )],
                    Some(p) => p.account_info.split('\n').map(|s| s.to_string()).collect(),
                }
            }
            Command::Insmod {
                driver_name,
                driver_version,
                driver_fn,
            } => {
                let path = std::path::PathBuf::from(
                    &driver_fn
                        .strip_prefix("file://")
                        .ok_or_else(|| anyhow::anyhow!("invalid path"))?,
                );
                let data = std::fs::read(path)?;
                let output = rt
                    .block_on(ctx.client.driver_client.load_driver(LoadDriverRequest {
                        driver_name,
                        driver_version,
                        driver_binary: data,
                    }))?
                    .into_inner();

                vec![format!(
                    "The driver {}@{} has been loaded",
                    output.driver_name, output.driver_version
                )]
            }
            Command::Rmmod {
                driver_name,
                driver_version,
            } => {
                let output = rt
                    .block_on(ctx.client.driver_client.unload_driver(
                        grpc::proto_types::UnloadDriverRequest {
                            driver_name,
                            driver_version,
                        },
                    ))?
                    .into_inner();

                vec![format!(
                    "The driver {}@{} has been unloaded",
                    output.driver_name, output.driver_version
                )]
            }
            Command::Mount {
                driver_name,
                driver_version,
                path,
            } => {
                let input = cmd
                    .stdin
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("stdin is required"))?;
                let output = rt.block_on(ctx.client.bind_client.bind(BindRequest {
                    driver_name: driver_name.clone(),
                    driver_version: driver_version.clone(),
                    path: path.clone(),
                    account_info: input.join("\n"),
                }))?;

                vec![format!(
                    "The bind action has been completed: {}@{} -> {}",
                    driver_name, driver_version, path
                )]
            }
        };

        Ok(command::OutputAction::Command(command::CommandOutput {
            prompt: cmd.prompt,
            command: cmd.command,
            stdin: cmd.stdin.unwrap_or_default(),
            stdout: output,
            stderr: Vec::new(),
        }))
    }

    fn prepare(&self, cmd: &str) -> shelgon::Prepare {
        let base = cmd.split_whitespace().next().unwrap_or_default();
        match base {
            "link" => shelgon::Prepare {
                command: cmd.to_string(),
                stdin_required: true,
            },
            _ => shelgon::Prepare {
                command: cmd.to_string(),
                stdin_required: false,
            },
        }
    }

    fn completion(
        &self,
        _ctx: &Self::Context,
        incomplete_command: &str,
    ) -> anyhow::Result<(String, Vec<String>)> {
        let commands = [
            "cat",
            "cd",
            "clear",
            "exit",
            "help",
            "insdriver",
            "ls",
            "lsdriver",
            "link",
            "pwd",
            "rmdriver",
            "stat",
        ];
        let halfway: Vec<_> = commands
            .into_iter()
            .filter(|c| c.starts_with(incomplete_command))
            .map(|c| c.to_string())
            .collect();

        if halfway.len() == 1 {
            Ok((
                halfway[0]
                    .clone()
                    .strip_prefix(incomplete_command)
                    .ok_or(anyhow::anyhow!("Failed to get the command"))?
                    .to_string(),
                Vec::new(),
            ))
        } else {
            Ok((String::new(), Vec::new()))
        }
    }
}

fn main() -> anyhow::Result<()> {
    let username = std::env::var("UNITS_USERNAME").context("`UNITS_USERNAME` not found")?;
    let password = std::env::var("UNITS_PASSWORD").context("`UNITS_PASSWORD` not found")?;

    let rt = tokio::runtime::Runtime::new()?;

    let config = config::Config {
        url: "http://localhost:8080".to_string(),
    };
    let mut client = rt.block_on(grpc::Clients::new(config.url))?;


    let token = rt.block_on(client.user_login_client.login(LoginRequest {
        user_name: username,
        password,
    }))?.into_inner().message;


    let app = renderer::App::<FinShellExecutor>::new_with_executor(
        rt,
        FinShellExecutor {},
        ShellContext {
            current_dir: "/".to_string(),
            client,
            user_token: token,
        },
    );
    app.execute()
}
