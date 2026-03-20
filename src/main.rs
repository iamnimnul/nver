use std::env;
use std::fmt;
use std::io;
use std::process::Command;

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    GitCommandFailed(String),
    Usage(String),
    NoValidTagFound,
    InvalidVersionTag(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {err}"),
            AppError::GitCommandFailed(msg) => write!(f, "Git command failed: {msg}"),
            AppError::Usage(msg) => write!(f, "{msg}"),
            AppError::NoValidTagFound => write!(
                f,
                "No valid version tag found. Expected tags like X.X.X, vX.X.X, or v.X.X.X."
            ),
            AppError::InvalidVersionTag(tag) => write!(
                f,
                "Invalid version tag '{tag}'. Expected X.X.X, vX.X.X, or v.X.X.X."
            ),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    fn bump(self, bump_type: BumpType) -> Self {
        match bump_type {
            BumpType::Major => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            BumpType::Minor => Version {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            BumpType::Patch => Version {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            },
        }
    }

    fn as_tag(self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BumpType {
    Major,
    Minor,
    Patch,
}

impl BumpType {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "major" => Some(BumpType::Major),
            "minor" => Some(BumpType::Minor),
            "patch" => Some(BumpType::Patch),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    bump_type: BumpType,
    dry_run: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = parse_args(env::args().skip(1).collect())?;
    let latest_tag = find_latest_version_tag()?;
    let current_version = parse_version_tag(&latest_tag)?;
    let next_version = current_version.bump(args.bump_type);
    let next_tag = next_version.as_tag();

    if args.dry_run {
        println!("Current tag: {latest_tag}");
        println!("Next tag: {next_tag}");
        return Ok(());
    }

    let commit_lines = collect_commit_lines_since(&latest_tag)?;
    let message = build_tag_message(&latest_tag, &next_tag, &commit_lines);
    create_annotated_tag(&next_tag, &message)?;

    println!("Created annotated tag: {next_tag}");
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<CliArgs, AppError> {
    if args.is_empty() {
        return Err(AppError::Usage(usage()));
    }

    let bump_type = BumpType::parse(&args[0]).ok_or_else(|| AppError::Usage(usage()))?;
    let mut dry_run = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            _ => return Err(AppError::Usage(usage())),
        }
    }

    Ok(CliArgs { bump_type, dry_run })
}

fn usage() -> String {
    "Usage: nver <major|minor|patch> [--dry-run]".to_string()
}

fn run_git(args: &[&str]) -> Result<String, AppError> {
    let output = Command::new("git").args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(AppError::GitCommandFailed(format!(
            "git {}: {}",
            args.join(" "),
            if stderr.is_empty() {
                "unknown git error"
            } else {
                &stderr
            }
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn find_latest_version_tag() -> Result<String, AppError> {
    let tags_output = run_git(&["tag", "--sort=-version:refname"])?;

    for tag in tags_output.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if parse_version_tag(tag).is_ok() {
            return Ok(tag.to_string());
        }
    }

    Err(AppError::NoValidTagFound)
}

fn parse_version_tag(tag: &str) -> Result<Version, AppError> {
    let trimmed = tag.trim();
    let without_prefix = if let Some(rest) = trimmed.strip_prefix("v.") {
        rest
    } else if let Some(rest) = trimmed.strip_prefix('v') {
        rest
    } else {
        trimmed
    };

    let parts: Vec<&str> = without_prefix.split('.').collect();
    if parts.len() != 3 {
        return Err(AppError::InvalidVersionTag(tag.to_string()));
    }

    let major = parts[0]
        .parse::<u64>()
        .map_err(|_| AppError::InvalidVersionTag(tag.to_string()))?;
    let minor = parts[1]
        .parse::<u64>()
        .map_err(|_| AppError::InvalidVersionTag(tag.to_string()))?;
    let patch = parts[2]
        .parse::<u64>()
        .map_err(|_| AppError::InvalidVersionTag(tag.to_string()))?;

    Ok(Version {
        major,
        minor,
        patch,
    })
}

fn collect_commit_lines_since(tag: &str) -> Result<Vec<String>, AppError> {
    let range = format!("{tag}..HEAD");
    let output = run_git(&["log", &range, "--pretty=format:%s (%h)"])?;
    let commits = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("- {line}"))
        .collect();

    Ok(commits)
}

fn build_tag_message(previous_tag: &str, new_tag: &str, commits: &[String]) -> String {
    let mut message = format!("Release {new_tag}\n\nChanges since {previous_tag}:\n");

    if commits.is_empty() {
        message.push_str("- No commits found since previous tag.\n");
    } else {
        for line in commits {
            message.push_str(line);
            message.push('\n');
        }
    }

    message
}

fn create_annotated_tag(tag: &str, message: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .args(["tag", "-a", tag, "-m", message])
        .output()?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(AppError::GitCommandFailed(format!(
        "git tag -a {tag} -m <message>: {}",
        if stderr.is_empty() {
            "unknown git error"
        } else {
            &stderr
        }
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_version() {
        let v = parse_version_tag("1.2.3").expect("should parse");
        assert_eq!(
            v,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[test]
    fn parses_v_prefixed_version() {
        let v = parse_version_tag("v1.2.3").expect("should parse");
        assert_eq!(
            v,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[test]
    fn parses_v_dot_prefixed_version() {
        let v = parse_version_tag("v.1.2.3").expect("should parse");
        assert_eq!(
            v,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[test]
    fn rejects_invalid_tag() {
        assert!(parse_version_tag("v1.2").is_err());
        assert!(parse_version_tag("vx.y.z").is_err());
    }

    #[test]
    fn bumps_major() {
        let current = Version {
            major: 1,
            minor: 4,
            patch: 9,
        };
        assert_eq!(
            current.bump(BumpType::Major),
            Version {
                major: 2,
                minor: 0,
                patch: 0
            }
        );
    }

    #[test]
    fn bumps_minor() {
        let current = Version {
            major: 1,
            minor: 4,
            patch: 9,
        };
        assert_eq!(
            current.bump(BumpType::Minor),
            Version {
                major: 1,
                minor: 5,
                patch: 0
            }
        );
    }

    #[test]
    fn bumps_patch() {
        let current = Version {
            major: 1,
            minor: 4,
            patch: 9,
        };
        assert_eq!(
            current.bump(BumpType::Patch),
            Version {
                major: 1,
                minor: 4,
                patch: 10
            }
        );
    }

    #[test]
    fn parses_args_with_dry_run() {
        let args = vec!["minor".to_string(), "--dry-run".to_string()];
        let parsed = parse_args(args).expect("args should parse");
        assert_eq!(
            parsed,
            CliArgs {
                bump_type: BumpType::Minor,
                dry_run: true
            }
        );
    }

    #[test]
    fn parses_args_without_dry_run() {
        let args = vec!["patch".to_string()];
        let parsed = parse_args(args).expect("args should parse");
        assert_eq!(
            parsed,
            CliArgs {
                bump_type: BumpType::Patch,
                dry_run: false
            }
        );
    }

    #[test]
    fn build_message_includes_commits() {
        let commits = vec![
            "- feat: add release command (abc123)".to_string(),
            "- fix: handle empty tag list (def456)".to_string(),
        ];
        let msg = build_tag_message("v1.0.0", "v1.1.0", &commits);
        assert!(msg.contains("Release v1.1.0"));
        assert!(msg.contains("Changes since v1.0.0:"));
        assert!(msg.contains("- feat: add release command (abc123)"));
        assert!(msg.contains("- fix: handle empty tag list (def456)"));
    }
}
