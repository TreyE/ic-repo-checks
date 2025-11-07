pub(crate) fn escape_data<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

pub(crate) fn escape_property<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

pub(crate) fn group(group_name: &str) {}
