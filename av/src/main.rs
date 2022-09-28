use regex::Match;
use regex::Regex;
use std::collections::hash_map::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    // 读取文件夹下的内容
    let root = "./";
    let file_paths = fs::read_dir(root)?
        .map(|v| v.unwrap().path())
        .filter(|v| Path::is_file(v))
        .collect::<Vec<_>>();

    println!("{:?}", file_paths);

    // 正则
    let re = Regex::new(r"^(\w{2,}-\d{2,})(?:_(\w))?").unwrap();
    #[derive(Debug)]
    struct FileData<'a> {
        path: &'a str,
        id: &'a str,
        serial_id: Option<Match<'a>>,
        ext: &'a OsStr,
    }

    let mut m: HashMap<_, Vec<FileData>> = HashMap::new();

    for item in file_paths.iter() {
        let p = item.to_str().unwrap();
        let a = {
            match re.captures(p) {
                Some(v) => v,
                _ => break,
            }
        };

        let b = FileData {
            path: p,
            id: &a.get(1).unwrap().as_str(),
            serial_id: a.get(2),
            ext: Path::new(item).extension().unwrap(),
        };
        let c = m.entry(b.id).or_insert(Vec::new());
        (*c).push(b);
    }

    for (key, value) in &m {
        // 是否在有多个不同的拓展名
        let mut is_duplicated = false;
        value.iter().reduce(|pre, cur| {
            if pre.ext != cur.ext {
                is_duplicated = true;
            }
            return cur;
        });

        // 判断存在 有序号 和 没序号同时存在的情况
        let serial_id_iter = value.iter().map(|v| v.serial_id);
        if serial_id_iter.len() >= 2 {
            for serial_id in serial_id_iter {
                if let None = serial_id {
                    is_duplicated = true;
                    break;
                }
            }
        }

        if is_duplicated {
            println!(
                "{}: {:?}",
                key,
                value.iter().map(|v| v.path).collect::<Vec<_>>()
            );
        }
    }

    Ok(())
}
