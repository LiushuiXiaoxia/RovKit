use rovkit::stringkit;

fn main() {
    let s = "  Hello_World testCase  ";

    println!("trim: '{}'", stringkit::trim(s));
    println!("lower: '{}'", stringkit::to_lower(s));
    println!("upper: '{}'", stringkit::to_upper(s));

    println!("contains 'World': {}", stringkit::contains(s, "World"));
    println!("starts_with '  He': {}", stringkit::starts_with(s, "  He"));
    println!("ends_with 'se  ': {}", stringkit::ends_with(s, "se  "));

    println!("replace first _: '{}'", stringkit::replace(s, "_", "-"));
    println!("replace all _: '{}'", stringkit::replace_all(s, "_", "-"));
    println!(
        "regex replace '\\s+': '{}'",
        stringkit::regex_replace(s, r"\s+", " ").unwrap()
    );

    let parts = stringkit::split(s, " ");
    println!("split by space: {:?}", parts);

    let joined = stringkit::join(&["a", "b", "c"], ",");
    println!("join a,b,c: '{}'", joined);

    println!(
        "to_snake_case 'HelloWorld': '{}'",
        stringkit::to_snake_case("HelloWorld")
    );
    println!(
        "to_camel_case 'hello_world': '{}'",
        stringkit::to_camel_case("hello_world")
    );

    println!("remove_whitespace: '{}'", stringkit::remove_whitespace(s));
}
