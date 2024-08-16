use chrono::{DateTime, Datelike, Local, Utc};
use inquire::{Confirm, Select, Text};
use std::env;
use std::fs::{create_dir, read_to_string, File};
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::Command;

const MONTHS: [&str; 13] = [
    "",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const CHECK_FILE: &str = "kairo_msg";

const LANG: &str = "en_US";

const COMMITS_TYPES: [&str; 68] = [
    "Star: New feature or enhancement",
    "Comet: Bug fix or error resolution",
    "Nebula: Code refactoring",
    "Pulsar: Performance improvement",
    "Quasar: Documentation or clarity improvement",
    "Asteroid Belt: Code cleanup and maintenance",
    "Solar Flare: Testing-related changes",
    "Dwarf Planet: Minor updates or fixes",
    "Terraform: Infrastructure changes",
    "Black Hole: Removing large chunks of code or features",
    "Wormhole: Merging branches or connecting code parts",
    "Big Bang: Initial commit or major feature start",
    "Launch: Deploying to production or releasing a version",
    "Lightspeed: Significant performance improvements",
    "Mission Control: Project management changes",
    "Spacewalk: Urgent hotfixes",
    "Moon Landing: Major milestone or goal completion",
    "First Contact: Initial integrations with external systems",
    "Interstellar Communication: Improving documentation or communication",
    "Solar Eclipse: Temporarily masking functionality",
    "Supernova: Major, transformative change",
    "Meteor Shower: Series of small changes or fixes",
    "Solar Wind: Refactoring code structure",
    "Lunar Eclipse: Temporarily disabling a feature",
    "Cosmic Dawn: Initial implementation of a feature",
    "Solar Storm: Rapid, impactful changes",
    "Lunar Transit: Minor, temporary change",
    "Perihelion: Brings the project closer to its goals or objectives",
    "Aphelion: Immediate goals, but is necessary for long-term progress",
    "White Dwarf: Improving code comments or documentation",
    "Red Giant: Expanding a feature or functionality",
    "Neutron Star: Optimizing code for performance",
    "Binary Star: Merging features or components",
    "Brown Dwarf: Undeveloped feature with potential",
    "Quark Star: Experimental or speculative change",
    "Rogue Planet: Independent change",
    "Stellar Nursery: Creation of new components",
    "Planetary Nebula: Removal or deprecation of a component",
    "Globular Cluster: Collection of related changes",
    "Void: Removal of a module, component, or feature",
    "Gravity: Resolving merge conflicts or dependencies",
    "Dark Matter: Fixing unknown or mysterious bugs",
    "Time Dilation: Improving code performance",
    "Spacetime: Changes to date, time, or scheduling",
    "Gravitational Lensing: Altering data or information flow",
    "Cosmic String: Connecting code parts",
    "Quantum Fluctuation: Small, random change",
    "Hawking Radiation: Removing technical debt",
    "Quantum Entanglement: Establishing close relationships between code parts",
    "Gravitational Redshift: Slowing down or reducing code performance",
    "Space Probe: Testing new features or technologies",
    "Station: Creating or improving environments",
    "Rocket Launch: Deploying to production",
    "Spacewalk: Urgent production hotfixes",
    "Space Elevator: Making codebase more accessible",
    "Warp Drive: Significant speed improvement",
    "Dyson Sphere: Comprehensive optimization of a specific area",
    "Generation Ship: Long-term project for a self -sustaining system",
    "Lagrange Point: Stabilizing or balancing code parts",
    "Orbital Maneuver: Changing project direction",
    "Mission Control: Represents project management-related changes",
    "Moon Landing: Celebrates the completion of major milestones",
    "Interstellar Travel: Migration to a new architecture or language",
    "Rover: Exploration of new technologies or approaches",
    "Singularity: Resolution of a complex or hard-to-reproduce issue",
    "Relativity: Changes related to time, dates, or timestamps",
    "Expansion: Scaling up the system or increasing capacity",
    "Big Crunch: Reduction of codebase size or removal of features",
];

const COMMIT_TEMPLATE: &str = "%type%(%scope%): %summary%\n\n\tThe following changes were made :\n\n%why%\n\n%footer%\n\n\tAuthored by :\n\n\t\t* %author% <%email%> the %date%\n";

fn check_commit(sentence: &str) -> Result<(), Error> {
    let mut f: File = File::create(CHECK_FILE).expect("msg");
    writeln!(f, "{sentence}").expect("msg");
    let o = Command::new("hunspell")
        .arg("-d")
        .arg(LANG)
        .arg("-l")
        .arg(CHECK_FILE)
        .output()
        .expect("msg")
        .stdout;
    if o.is_empty() {
        return Ok(());
    }
    arrange_commit()
}
fn arrange_commit() -> Result<(), Error> {
    let _ = Command::new("hunspell")
        .arg("-d")
        .arg(LANG)
        .arg(CHECK_FILE)
        .spawn()
        .expect("Missing dic")
        .wait()
        .unwrap()
        .success();
    check_commit(
        read_to_string(CHECK_FILE)
            .expect("failed to parse zen file")
            .as_str(),
    )
}
fn get_commit_types() -> String {
    let mut t: String;
    loop {
        t = Select::new("Please enter the commit type : ", COMMITS_TYPES.to_vec())
            .prompt()
            .unwrap()
            .to_string();
        if t.is_empty() {
            continue;
        }
        if confirm(format!("Really use the commit type : {t}").as_str(), false) {
            break;
        }
    }
    let x: Vec<&str> = t.split(':').collect();
    let mut s: String = String::from("\n");
    s.push_str((*x.first().unwrap()).to_string().as_str());
    s
}
fn commit_summary() -> String {
    let mut summary: String;
    loop {
        summary = Text::new("Please enter the commit summary : ")
            .prompt()
            .unwrap();
        if summary.is_empty() {
            continue;
        }
        if summary.len().gt(&50) {
            println!("Summary must be contains less than 50 chararacter");
            continue;
        }
        if confirm(format!("Use the summary : {summary}").as_str(), false) {
            break;
        }
    }
    summary
}
fn commit_why() -> String {
    let mut why: String = String::new();
    loop {
        let w = Text::new("Please explain the reasoning behind the change : ")
            .prompt()
            .unwrap();
        if w.is_empty() {
            continue;
        }
        if w.len().gt(&50) {
            println!("The reasoning behind the change must be contains less than 50 chararacter");
            continue;
        }
        why.push_str(format!("\n\t\t* {w}").as_str());
        if confirm("Continue to write the changes : ", false) {
            continue;
        }
        break;
    }
    why
}
fn commit_footer() -> String {
    let mut footer: String = String::new();
    if confirm("Code has breaking changes ?", false) {
        footer.push_str("\n\tThe following changes break :\n");
        loop {
            let b = Text::new("Please enter the breaking change description: ")
                .prompt()
                .unwrap();
            if b.is_empty() {
                continue;
            }
            if confirm(
                format!("Use breaking change description : {b}").as_str(),
                false,
            ) {
                footer.push_str(format!("\n\t\t* {b}\n").as_str());
                if confirm("Add a new description line ?", false).eq(&true) {
                    continue;
                }
                break;
            }
        }
    }
    if confirm("Code has resolving issues ?", false) {
        footer.push_str("\n\tThe commit resolve their issues :\n");
        loop {
            footer.push_str("\n\t\tFixes ");
            loop {
                let f = Text::new("Please enter the issue number : ")
                    .prompt()
                    .unwrap();
                if f.is_empty() {
                    continue;
                }
                footer.push_str(format!("#{f}\n").as_str());
                break;
            }
            if confirm("Code resolving an other issues ?", false) {
                continue;
            }
            break;
        }
    }
    if confirm("Code close an issue ?", false) {
        footer.push_str("\n\tThe commit close their issues :\n");
        loop {
            footer.push_str("\n\t\tCloses ");
            loop {
                let f = Text::new("Please enter the issue number : ")
                    .prompt()
                    .unwrap();
                if f.is_empty() {
                    continue;
                }
                footer.push_str(format!("#{f}\n").as_str());
                break;
            }
            if confirm("Code resolve an other issue ?", false) {
                continue;
            }
            break;
        }
    }
    footer
}
fn confirm(msg: &str, default: bool) -> bool {
    if let Ok(rep) = Confirm::new(msg).with_default(default).prompt() {
        return rep.eq(&true);
    }
    false
}

fn email() -> Result<String, env::VarError> {
    std::env::var("KAIRO_EMAIL")
}

fn commit_scope() -> String {
    let mut scope: String;
    loop {
        scope = Text::new("Please enter the commit scope : ")
            .prompt()
            .unwrap();
        if scope.is_empty() {
            continue;
        }
        if scope.len().gt(&20) {
            println!("scope can be superior to 20 character");
            continue;
        }
        if confirm(
            format!("Really use the commit scope : {scope}").as_str(),
            false,
        ) {
            break;
        }
    }
    scope
}
fn get_scope() -> String {
    let mut scope: String;
    loop {
        scope = commit_scope();
        if check_commit(scope.as_str()).is_ok() {
            break;
        }
    }
    scope
}
fn diff() -> Result<(), Error> {
    assert!(Command::new("kairo_diff")
        .current_dir(".")
        .spawn()?
        .wait()?
        .success());
    Ok(())
}
fn zuu() -> Result<(), Error> {
    Ok(())
}

fn get_summary() -> String {
    let mut summary: String;
    loop {
        summary = commit_summary();
        if check_commit(summary.as_str()).is_ok() {
            break;
        }
    }
    summary
}
fn get_footer() -> String {
    let mut footer: String;
    loop {
        footer = commit_footer();
        if check_commit(footer.as_str()).is_ok() {
            break;
        }
    }
    footer
}
fn get_why() -> String {
    let mut why: String;
    loop {
        why = commit_why();
        if check_commit(why.as_str()).is_ok() {
            break;
        }
    }
    why
}
fn author() -> Result<String, env::VarError> {
    env::var("KAIRO_AUTHOR")
}
fn commit() -> Result<(), Error> {
    assert!(zuu().is_ok());
    assert!(diff().is_ok());
    let date: DateTime<Local> = Local::now();
    if let Ok(author) = author() {
        if Path::new(format!("./.chronos/commits/{author}").as_str())
            .is_dir()
            .eq(&false)
        {
            create_dir(format!("./.chronos/commits/{author}").as_str())?;
            create_dir(format!("./.chronos/commits/{author}/{}", date.year()).as_str())?;
            for month in MONTHS {
                if month.is_empty() {
                    continue;
                }
                create_dir(
                    format!("./.chronos/commits/{author}/{}/{month}", date.year()).as_str(),
                )?;
            }
        }
        if let Ok(e) = email() {
            if let Some(m) = MONTHS.get(date.month() as usize) {
                let c = COMMIT_TEMPLATE
                    .replace("%type%", get_commit_types().trim())
                    .replace("%scope%", get_scope().trim())
                    .replace("%summary%", get_summary().trim())
                    .replace("%why%", get_why().as_str())
                    .replace("%footer%", get_footer().trim())
                    .replace("%date%", Utc::now().date_naive().to_string().trim())
                    .replace("%author%", author.as_str())
                    .replace("%email%", e.as_str());

                let commit = format!("./.chronos/commits/{author}/{}/{m}/", date.year());

                if let Ok(mut file) =
                    File::create_new(format!("{commit}/{m}/{}.commit", date.timestamp()).as_str())
                {
                    assert!(writeln!(file, "{}", c).is_ok());
                }

                return Ok(());
            }
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "Missing KAIRO_EMAIL variable",
        ));
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "Missing KAIRO_AUTHOR variable",
    ))
}
fn main() -> Result<(), Error> {
    commit()
}
