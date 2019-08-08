use indexmap::IndexMap;

#[derive(Debug)]
pub struct UnitFile {
    map: IndexMap<String, Section>,
}

impl UnitFile {
    pub fn new() -> Self {
        UnitFile {
            map: IndexMap::new(),
        }
    }

    pub fn add_section<T: Into<String>>(&mut self, name: T) {
        let section = Section::new();
        self.map.insert(name.into(), section);
    }

    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.map.get_mut(name)
    }

    pub fn output(&self) -> String {
        let mut ret = String::new();
        for (name, section) in self.map.iter() {
            ret.push('[');
            ret.push_str(name);
            ret.push_str("]\n");

            section.output_to_string(&mut ret);

            ret.push('\n');
        }
        ret
    }
}

#[derive(Debug)]
pub struct Section {
    map: IndexMap<String, String>,
}

impl Section {
    pub fn new() -> Self {
        Section {
            map: IndexMap::new(),
        }
    }

    pub fn add<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.map.insert(key.into(), value.into());
    }

    pub fn output_to_string(&self, s: &mut String) {
        for (key, val) in self.map.iter() {
            s.push_str(key);
            s.push_str(" = ");
            s.push_str(val);
            s.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUARKY_UNIT: &str = r#"[Unit]
Description = quarky
Wants = network-online.target
After = network-online.target

[Service]
WorkingDirectory = /home/pi/adumbidiot/quarky/
Environment = "RUSTC_WRAPPER=sccache"
ExecStart = /home/pi/.cargo/bin/cargo +stable run --release
Restart = always
User = pi

"#;

    #[test]
    fn it_works() {
        let mut unit_file = UnitFile::new();

        unit_file.add_section("Unit");
        let unit_section = unit_file.get_section_mut("Unit").unwrap();
        unit_section.add("Description", "quarky");
        unit_section.add("Wants", "network-online.target");
        unit_section.add("After", "network-online.target");

        unit_file.add_section("Service");
        let service_section = unit_file.get_section_mut("Service").unwrap();
        service_section.add("WorkingDirectory", "/home/pi/adumbidiot/quarky/");
        service_section.add("Environment", "\"RUSTC_WRAPPER=sccache\"");
        service_section.add(
            "ExecStart",
            "/home/pi/.cargo/bin/cargo +stable run --release",
        );
        service_section.add("Restart", "always");
        service_section.add("User", "pi");

        let output = unit_file.output();
        assert_eq!(output, QUARKY_UNIT);
    }
}
