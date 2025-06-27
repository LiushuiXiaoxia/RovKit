use rovkit::envkit;

fn main() {
    envkit::load_dotenv();

    if let Some(val) = envkit::get_var("JAVA_HOME") {
        println!("JAVA_HOME = {}", val);
    }

    if let Some(port) = envkit::get_var_as::<u16>("PORT") {
        println!("Server will run on port: {}", port);
    }

    if envkit::contains_var("HOME") {
        println!("HOME is set");
    }

    for (k, v) in envkit::get_all_vars() {
        println!("{} = {}", k, v);
    }
}
