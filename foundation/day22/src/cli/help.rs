pub fn help(program: &str) -> String {
    format!(
        "Usage: {program} [command] [options] \n
     Commands: \n
      print  Print items \n
       add   Add item \n
        remove   Remove item \n
        \n
        Options: \n
        --file <path>   Path to data \n
        --sort <spec>   Sort order \n
        (price | name | price_then_name) \n
        --help, -h   Show this help"
    )
}
