use std::{ffi::OsString, fs::{self, DirEntry, Metadata}, os::unix::fs::{MetadataExt, PermissionsExt}, usize};
use clap::Parser;
use toml::Value;
use users::{get_user_by_uid, get_group_by_gid};
use colored::{ColoredString, Colorize};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    show_meta: bool,
}

struct Line {
    is_dir: bool,
    permssion: String,
    nlink: String,
    owner_user: String,
    owner_group: String,
    name: String,
    description: String,
}

fn main() -> std::io::Result<()>{
    display_dir(&".".to_string())?;
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

fn display_dir(root_path: &String) -> std::io::Result<()> {
    let mut paths : Vec<_> = fs::read_dir(root_path).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());
    let mut lines : Vec<Line> = vec![];
    let mut max_file_len : usize = 0;
    let mut max_nlink_len : usize = 0;
    let value: Option<toml::Value>;

    let meta_file = fs::read_to_string(".meta.toml").unwrap_or_default();
    value = toml::from_str::<toml::Value>(&meta_file).ok();
    
    
    for path in paths{
        lines.push(get_new_line(path, root_path, &value, &mut max_file_len, &mut max_nlink_len));
    }
    
    
    for line in lines {
        print_line(line, &max_file_len, &max_nlink_len);
    }
    
    Ok(())

}

fn print_line(line: Line, max_file_len: &usize, max_nlink_len: &usize) {
    let is_dir = line.is_dir;
    let permission = line.permssion;
    let mut nlink = line.nlink;
    let user_name = line.owner_user;
    let group_name = line.owner_group;
    let mut file_name: String = line.name;
    let description = line.description;

    if max_file_len > &file_name.len() {
        let width = max_file_len - file_name.len();
        file_name = format!("{:<width$}{file_name}", " ");
    }

    let colored_file_name: ColoredString;
    if is_dir {
        colored_file_name = format!("{file_name}").white().bold();
    } else {
        colored_file_name = file_name.normal();
    }

    if max_nlink_len > &nlink.len() {
        let width = max_nlink_len - nlink.len();
        nlink =  format!("{:<width$}{nlink}", " ");
    }
    
    let colored_description = format!("{description}").to_string().italic().cyan();
    
    println!("{permission} {nlink} {user_name} {group_name} {colored_file_name} {colored_description}")
}

fn get_folder_description(folder_path:&String, name:&String) -> String {
    let folder_path = format!("{folder_path}/{name}");
    let result: String;

    let meta_file = fs::read_to_string(format!("{folder_path}/.meta.toml")).unwrap_or_default();
    let value = toml::from_str::<toml::Value>(&meta_file).ok();
    result = value.as_ref().and_then(|v| v.get("folder"))
    .and_then(|t| t.get("description"))
    .and_then(|d| d.as_str())
    .unwrap_or("")
    .to_string();

    if result != "" {
        format!("<--{result}")
    }else {format!("")}
    
}

fn get_new_line(path: DirEntry,root_path:&String, value: &Option<Value> , max_file_len: &mut usize, max_nlink_len: &mut usize) -> Line {
    let name = path.file_name();
    let meta_data = fs::metadata(&name).unwrap();
    let permission = get_permission(&meta_data).unwrap();
    let nlink = meta_data.nlink().to_string();
    let user_name: String = get_owner_user(&meta_data);
    let group_name: String = get_owner_group(&meta_data);
    let is_dir = meta_data.is_dir();
    let display_name = name.display().to_string();
    let name_len = display_name.len();
    let nlink_len = nlink.len();
    
    let description: String;
    if is_dir{
        description = get_folder_description(&root_path, &display_name);
    }
    else {
        description = get_file_description(&name, &value);
    }

    let line: Line = Line {is_dir: is_dir, permssion: permission, nlink: nlink,owner_user: user_name, owner_group: group_name, name: display_name, description: description };

    
    match *max_file_len < name_len {
        true => {
            *max_file_len = name_len;
        }
        false => (),
    }

    match *max_nlink_len < nlink_len {
        true => {
            *max_nlink_len = nlink_len;
        }
        false => (),
    }

    line
}

fn get_file_description(file_name:&OsString, value:&Option<toml::Value>) -> String {
    
    let result: String = value.as_ref().and_then(|v| v.get("file"))
    .and_then(|t| t.get(file_name.to_string_lossy().to_string()))
    .and_then(|d| d.as_str())
    .unwrap_or("")
    .to_string();

    if result != "" {
        format!("<--{result}")
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

