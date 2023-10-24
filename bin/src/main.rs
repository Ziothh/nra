use std::path::Path;

use clap::Parser;

use crate::{cli::CLI, utils::project::ProjectPath};

mod cli;
mod next;
mod utils;

const OUT_DIR: &str = "/home/zioth/projects/libs/nra/playground/generated";

fn bin() -> Result<(), String> {
    let cli = CLI::parse();

    let Ok(project_path) = Path::new(&cli.project_path).canonicalize() else {
        return Err(format!(
            "Project path \"{}\" does not exist.",
            cli.project_path
        ));
    };
    println!("Project at \"{}\"\n", project_path.to_str().unwrap());

    let src = next::project::find_project_source(&project_path.to_owned()).unwrap();

    let out_dir = Path::new(&OUT_DIR).canonicalize().unwrap();
    std::fs::create_dir_all(&out_dir).unwrap();

    // Handle pages dir
    let pages_dir_str = match src.join("pages").canonicalize() {
        Ok(pages_dir) => {
            let mut entries = Vec::new();
            next::routers::pages::parse_dir(ProjectPath::from_root(pages_dir), &mut entries)
                .unwrap();

            serde_json::to_string_pretty(&entries).unwrap()
        }
        Err(_) => "[]".to_owned(),
    };

    // Handle app dir
    let app_dir_str = match src.join("app").canonicalize() {
        Ok(app_dir) => {
            let mut entries = Vec::new();
            next::routers::app::parse_dir(ProjectPath::from_root(app_dir), &mut entries).unwrap();

            serde_json::to_string_pretty(&entries).unwrap()
        }
        Err(_) => "[]".to_owned(),
    };

    write_node_modules_types(&pages_dir_str, &app_dir_str).unwrap();
    write_ts(&out_dir, &pages_dir_str, &app_dir_str).unwrap();

    return Ok(());
}

/// Write the parsed routes to a TypeScript file.
fn write_node_modules_types(pages_dir_str: &str, app_dir_str: &str) -> anyhow::Result<()> {
    std::fs::write(
        Path::new("/home/zioth/projects/libs/nra/lib/generated/routes.d.ts"),
        [
            "/* NOTE: THIS FILE HAS BEEN AUTOMATICALLY GENERATED. DO NOT EDIT. */\n",
            &format!("\nexport type PAGES_ROUTES = {};", pages_dir_str),
            &format!("\nexport type APP_ROUTES = {};", app_dir_str,),
        ]
        .join("\n"),
    )
    .unwrap();

    println!("Wrote types.");

    return anyhow::Ok(());
}

/// Write the parsed routes to a TypeScript file.
fn write_ts(out_dir: &Path, pages_dir_str: &str, app_dir_str: &str) -> anyhow::Result<()> {
    let mut contents =
        "/* NOTE: THIS FILE HAS BEEN AUTOMATICALLY GENERATED. DO NOT EDIT. */\n\n".to_owned();
    contents += &format!("\nexport const PAGES_ROUTES = {} as const;", pages_dir_str);
    contents += &format!("\nexport const APP_ROUTES = {} as const;", app_dir_str,);

    std::fs::write(out_dir.join("routes.ts"), contents).unwrap();

    println!("Wrote ts generated output to {out_dir:?}");

    return anyhow::Ok(());
}

fn main() {
    match bin() {
        Ok(_) => (),
        Err(err) => {
            eprint!("Error:\n{}", err);
            std::process::exit(1);
        }
    }
}
