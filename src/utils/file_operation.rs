use std::fs::{File,create_dir_all};
use std::io::{BufWriter, Write, ErrorKind};

// Vec<String> を　pathに保存
// 引数はmoveする、String\nString・・・形式で保存
pub fn output_txt(vsc_strings: Vec<String>, path:&str) -> anyhow::Result<()> {
    let file = File::create(path);
    let file = match file {
        Ok(t) => t,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            create_folder_from_filepath(path)?;
            File::create(path).map_err(|e| anyhow::anyhow!(e))?
        },
        Err(e) => return Err(anyhow::anyhow!("file_operation/output_txt/File::create でエラー:{:?}",e)),
    };

    let mut buf = BufWriter::new(file);

    for string in vsc_strings {
        buf.write(string.as_bytes()).unwrap();
        buf.write("\n".as_bytes()).unwrap();
    }

    Ok(())
}

fn create_folder_from_filepath(path:&str) -> anyhow::Result<()>{
    let vec: Vec<&str> = path.split('/').collect();

    let mut folder_path = String::new();
    for num in 0..vec.len() - 1 {
        folder_path.push_str(vec[num]);
        folder_path.push('/');
    }
    create_dir_all(&folder_path).map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}