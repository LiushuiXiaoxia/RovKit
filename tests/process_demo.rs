#[cfg(test)]
mod tests {
    use rovkit::processkit;
    use std::io::BufRead;

    #[test]
    fn test01() {
        processkit::call("cd src && ls -alh")
            .unwrap()
            .stdout
            .lines()
            .for_each(|line| println!("{}", line.unwrap()));
    }
}
