use anyhow::*;
use std::io::Write;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let dryrun = true; // TODO:

    let home = std::env::var("HOME").context("HOME is not set")?;

    let repo = std::env::args().nth(1).context("invalid argument")?;
    let (dir, _) = split_reponame(&repo).context("invalid input")?;

    let base = format!("{}/dev/src/github.com/{}", &home, &dir);
    let repo_dir = format!("{}/dev/src/github.com/{}", &home, &repo);

    println!("repo dir: {}", &repo_dir);

    if Path::new(&repo_dir).is_dir() {
        println!("directory exists");
        return Ok(());
    }

    if !Path::new(&base).is_dir() {
        println!("creating directory: {}", &base);
        std::fs::create_dir(&base)?; // mkdir
    }
    std::env::set_current_dir(base)?; // cd

    let mut cmd = std::process::Command::new("gh");
    let cmd = cmd.args(&["repo", "clone", &repo, "--", "--filter=blob:none"]);

    if dryrun {
        println!("{:?}", &cmd);
    } else {
        let output = cmd.output().context("failed to execute process")?;
        std::io::stdout().write_all(&output.stdout)?;
    }

    Ok(())
}

fn split_reponame(reponame: &str) -> Option<(&str, &str)> {
    if let Some(idx) = reponame.find('/') {
        let a = &reponame[..idx];
        let b = &reponame[idx + 1..];

        if b.find('/').is_some() {
            return None;
        }

        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_works() {
        let (a, b) = split_reponame("user/repo").unwrap();
        assert_eq!(a, "user");
        assert_eq!(b, "repo");
    }
}
