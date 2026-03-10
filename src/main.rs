use std::{ffi::OsString, fs::{self, Metadata}, os::unix::fs::{MetadataExt, PermissionsExt}};
use clap::Parser;
use users::{get_user_by_uid, get_group_by_gid};
use colored::Colorize;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    show_meta: bool,
}

struct Line {
    permssion: String,
    owner_user: String,
    owner_group: String,
    file_name: String,
    description: String,
}

fn main() -> std::io::Result<()>{
    let args = Args::parse();
    display_dir(".", args.show_meta)?;
    Ok(())
}

fn format_permissions(mode: &u32) -> String {
    let bits = [0o400, 0o200, 0o100, 0o40, 0o20, 0o10,0o4,0o2,0o1];
    let chars = ['r', 'w', 'x', 'r', 'w', 'x', 'r', 'w', 'x'];
    bits.iter().zip(chars.iter()).map(|(bit, ch)| if mode & bit != 0{*ch} else {'-'}).collect::<String>()
}

fn get_permission(meta_data: &Metadata) -> std::io::Result<String> {
        let file_type = get_file_type(&meta_data);
        let permissions = meta_data.permissions();
        let permission_mode = format_permissions(&permissions.mode());
        Ok(format!("{file_type}{permission_mode}"))
}

fn get_owner_user(meta_data: &Metadata) -> String {
    get_user_by_uid(meta_data.uid()).map(|user| user.name().to_string_lossy().to_string()).unwrap_or_default()
}

fn get_owner_group(meta_data: &Metadata) -> String {
    get_group_by_gid(meta_data.gid()).map(|group| group.name().to_string_lossy().to_string()).unwrap_or_default()
}

fn display_dir(path: &str, is_show_meta_toml: bool) -> std::io::Result<()> {
    let mut paths : Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());
    let mut lines : Vec<Line> = vec![];
    let mut max_len : usize = 0;
    let mut is_exist_meta = false;
    let mut value: Option<toml::Value> = None;

    for path in &paths {
        if path.file_name() == ".meta.toml" {
            is_exist_meta = true;
            let meta_file = fs::read_to_string(path.file_name()).unwrap_or_default();
            value = toml::from_str::<toml::Value>(&meta_file).ok();
            break;
        }
    }

    if !is_exist_meta {
        fs::File::create_new(".meta.toml").ok();
    }

    for path in paths{
        let file_name = path.file_name();
        if file_name != ".meta.toml" || is_show_meta_toml{
            let meta_data = fs::metadata(&file_name)?;
            let permission = get_permission(&meta_data)?;
            let user_name: String = get_owner_user(&meta_data);
            let group_name: String = get_owner_group(&meta_data);
            
            let description: String;
            description = get_description(&file_name, &value);

            let line: Line = Line { permssion: permission, owner_user: user_name, owner_group: group_name, file_name: file_name.display().to_string(), description: description };
            lines.push(line);

            let file_len = file_name.display().to_string().len();
            if max_len < file_len {
                max_len = file_len;
            }
        }
    }

    for line in lines {
        let permission = line.permssion;
        let user_name = line.owner_user;
        let group_name = line.owner_group;
        let mut file_name: String = line.file_name;
        if max_len > file_name.len() {
            let width = max_len - file_name.len();
            file_name = format!("{file_name}{:<width$}", " ");
        }
        let description = line.description;
        let colored_description = format!("{description}").to_string().italic().yellow();
        println!("{permission} {user_name} {group_name} {file_name}   {colored_description}")
    }
    Ok(())

}

fn get_description(file_name:&OsString, value:&Option<toml::Value>) -> String {
    
    let result = value.as_ref().and_then(|v| v.get("file"))
    .and_then(|t| t.get(file_name.to_string_lossy().to_string()))
    .and_then(|d| d.as_str())
    .unwrap_or("")
    .to_string();

    if result != "" {
        format!("# {result}")
    }else {
        format!("")
    }
}

fn get_file_type(meta_data: &Metadata) -> String{
    if meta_data.is_dir() {
        "d".to_string()
    } else if meta_data.is_file() {
        "-".to_string()
    } else {
        "l".to_string()
    }
}

