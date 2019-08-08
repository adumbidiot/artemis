use crate::unit::UnitFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(alias = "Service")]
    service: Service,
    #[serde(default)]
    meta: HashMap<String, String>,
}

impl Config {
    pub fn to_unit(&self) -> UnitFile {
        let mut unit_file = UnitFile::new();

        unit_file.add_section("Unit");
        let unit_section = unit_file.get_section_mut("Unit").unwrap();
        unit_section.add("Description", self.get_service().get_name());

        unit_file.add_section("Service");
        let service_section = unit_file.get_section_mut("Service").unwrap();
        service_section.add("ExecStart", self.get_service().get_start());

        unit_file
    }

    pub fn get_service(&self) -> &Service {
        &self.service
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Service {
    #[serde(alias = "Name")]
    name: String,

    #[serde(alias = "Start")]
    start: String,
}

impl Service {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_start(&self) -> &str {
        &self.start
    }
}
