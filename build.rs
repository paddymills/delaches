use std::{env, fs, io, path::Path};

pub fn main() -> io::Result<()> {
    if Ok("release".to_owned()) == env::var("PROFILE") {
        // copy assets
        copy_dir("assets", "dist")?;
        copy_dir("public", "dist")?;
        copy_dir("style", "dist")?;
        fs::copy(".env", "dist/.env")?;
    }

    Ok(())
}

fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    // println!("{:?} -> {:?}", src, dest);
    fs::create_dir_all(&dest.join(src))?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir(entry.path(), dest.join(entry.path()))?;
        } else {
            fs::copy(entry.path(), dest.join(entry.path()))?;
        }
    }

    Ok(())
}
