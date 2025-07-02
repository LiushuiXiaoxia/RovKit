#[cfg(test)]
mod tests {
    use rovkit::iokit::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        write_string_to_file(&file_path, "hello world").unwrap();
        assert_eq!(read_file_to_string(&file_path).unwrap(), "hello world");

        append_string_to_file(&file_path, "!!!").unwrap();
        assert_eq!(read_file_to_string(&file_path).unwrap(), "hello world!!!");

        let bytes = read_file_to_bytes(&file_path).unwrap();
        assert_eq!(bytes, b"hello world!!!");
    }

    #[test]
    fn test_copy_and_remove() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src.txt");
        let dst = dir.path().join("dst.txt");

        write_string_to_file(&src, "copy me").unwrap();
        copy_file(&src, &dst).unwrap();
        assert!(path_exists(&dst));
        assert_eq!(read_file_to_string(&dst).unwrap(), "copy me");

        remove_all(&src).unwrap();
        assert!(!path_exists(&src));

        remove_all(&dst).unwrap();
        assert!(!path_exists(&dst));
    }

    #[test]
    fn test_create_dir_all() {
        let dir = tempdir().unwrap();
        println!("test_create_dir_all: dir = {:?}", dir);
        let nested = dir.path().join("a/b/c");
        create_dir_all(&nested).unwrap();
        assert!(is_dir(&nested));
    }

    #[test]
    fn test_get_file_size() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("size.txt");
        write_string_to_file(&file, "12345").unwrap();
        assert_eq!(get_file_size(&file).unwrap(), 5);
    }

    #[test]
    fn test_temp_file_and_dir() {
        let (path, file) = create_temp_file().unwrap();
        assert!(path.exists());
        drop(file); // 关闭文件，但文件仍存在
        assert!(path.exists());

        let temp_dir = create_temp_dir().unwrap();
        assert!(temp_dir.path().exists());
        // TempDir drop时自动删除
    }

    #[test]
    fn test_is_symlink() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        let link_path = dir.path().join("link.txt");
        create_symlink(&file_path, &link_path).unwrap();

        assert_eq!(is_symlink(&file_path).unwrap(), false);
        assert_eq!(is_symlink(&link_path).unwrap(), true);
    }

    #[test]
    fn test_create_symlink() {
        let dir = tempdir().unwrap();

        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "hello").unwrap();

        let link_path = dir.path().join("file_link.txt");
        create_symlink(&file_path, &link_path).unwrap();

        assert!(link_path.exists());
        assert!(is_symlink(&link_path).unwrap());
    }
}
