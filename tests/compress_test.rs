#[cfg(test)]
mod tests {
    use rovkit::compresskit::*;
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
