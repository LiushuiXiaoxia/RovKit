use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;
use tar::{Archive, Builder};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

/// ZIP 文件压缩：将目录或文件压缩成 zip
pub fn zip<P: AsRef<Path>>(src_path: P, dst_zip: P) -> io::Result<()> {
    let src_path = src_path.as_ref();
    let dst_file = File::create(&dst_zip)?;
    let mut zip = ZipWriter::new(BufWriter::new(dst_file));
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    if src_path.is_file() {
        let mut f = File::open(src_path)?;
        zip.start_file(src_path.file_name().unwrap().to_string_lossy(), options)?;
        io::copy(&mut f, &mut zip)?;
    } else {
        for entry in WalkDir::new(src_path) {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(src_path).unwrap();

            if path.is_file() {
                let mut f = File::open(path)?;
                zip.start_file(name.to_string_lossy(), options)?;
                io::copy(&mut f, &mut zip)?;
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name.to_string_lossy(), options)?;
            }
        }
    }

    zip.finish()?;
    Ok(())
}

/// ZIP 文件解压
pub fn unzip<P: AsRef<Path>>(zip_file: P, dst_dir: P) -> io::Result<()> {
    let zip_file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = dst_dir.as_ref().join(file.mangled_name());

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&out_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

/// TAR.GZ 压缩：目录或文件压缩为 tar.gz
pub fn tar_gz<P: AsRef<Path>>(src_path: P, dst_tar_gz: P) -> io::Result<()> {
    let src_path = src_path.as_ref();
    let tar_gz = File::create(dst_tar_gz)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    if src_path.is_file() {
        tar.append_path(src_path)?;
    } else {
        tar.append_dir_all(".", src_path)?;
    }
    tar.finish()?;
    Ok(())
}

/// TAR.GZ 解压
pub fn untar_gz<P: AsRef<Path>>(tar_gz_file: P, dst_dir: P) -> io::Result<()> {
    let tar_gz = File::open(tar_gz_file)?;
    let dec = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(dec);
    archive.unpack(dst_dir)?;
    Ok(())
}

/// 纯 tar 压缩（不带 gzip）
/// 支持单文件或目录压缩
pub fn tar<P: AsRef<Path>>(src_path: P, dst_tar: P) -> io::Result<()> {
    let src_path = src_path.as_ref();
    let tar_file = File::create(dst_tar)?;
    let mut tar_builder = Builder::new(tar_file);

    if src_path.is_file() {
        tar_builder.append_path(src_path)?;
    } else {
        tar_builder.append_dir_all(".", src_path)?;
    }

    tar_builder.finish()?;
    Ok(())
}

/// 纯 tar 解压（不带 gzip）
pub fn untar<P: AsRef<Path>>(tar_file: P, dst_dir: P) -> io::Result<()> {
    let tar_file = File::open(tar_file)?;
    let mut archive = Archive::new(tar_file);
    archive.unpack(dst_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_zip() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let file_path = src_dir.join("hello.txt");
        let mut f = File::create(&file_path).unwrap();
        writeln!(f, "hello zip").unwrap();

        let zip_path = dir.path().join("archive.zip");
        zip(&src_dir, &zip_path).unwrap();

        let unzip_dir = dir.path().join("unzip");
        unzip(&zip_path, &unzip_dir).unwrap();

        let content = fs::read_to_string(unzip_dir.join("hello.txt")).unwrap();
        assert_eq!(content.trim(), "hello zip");
    }

    #[test]
    fn test_targz() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        let file_path = src_dir.join("hello.txt");
        let mut f = File::create(&file_path).unwrap();
        writeln!(f, "hello tar.gz").unwrap();

        let tar_gz_path = dir.path().join("archive.tar.gz");
        tar_gz(&src_dir, &tar_gz_path).unwrap();

        let untar_dir = dir.path().join("untar");
        untar_gz(&tar_gz_path, &untar_dir).unwrap();

        let content = fs::read_to_string(untar_dir.join("hello.txt")).unwrap();
        assert_eq!(content.trim(), "hello tar.gz");
    }

    #[test]
    fn test_tar() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();

        let file_path = src_dir.join("hello.txt");
        let mut f = File::create(&file_path).unwrap();
        writeln!(f, "hello tar").unwrap();

        let tar_path = dir.path().join("archive.tar");
        tar(&src_dir, &tar_path).unwrap();

        let untar_dir = dir.path().join("untar");
        untar(&tar_path, &untar_dir).unwrap();

        let content = fs::read_to_string(untar_dir.join("hello.txt")).unwrap();
        assert_eq!(content.trim(), "hello tar");
    }
}
