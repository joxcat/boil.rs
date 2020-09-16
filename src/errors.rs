error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        TomlDeser(::toml::de::Error);
    }
}
