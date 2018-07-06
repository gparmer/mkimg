use syshelpers::{dump_file, emit_file, reset_dir, exec_pipeline};
use cossystem::{CosSystem};
use compobject::CompObject;
use std::collections::BTreeMap;

pub struct ComposeSpec {
    sysspec: String,
    sys: CosSystem,
    binaries: BTreeMap<String, Vec<u8>>,
}

pub struct Compose<'a> {
    spec: &'a ComposeSpec,
    comp_objs: Vec<CompObject<'a>>
}

impl ComposeSpec {
    pub fn parse_spec(sysspec: String) -> Result<ComposeSpec, String> {
        let sys = CosSystem::parse(sysspec.clone())?;
        let mut bins = BTreeMap::new();

        for c in sys.comps().iter() {
            let obj_contents = dump_file(&c.img())?;

            bins.insert(c.name().clone(), obj_contents);
        }

        Ok(ComposeSpec {
            sysspec: sysspec,
            sys: sys,
            binaries: bins,
        })
    }

    pub fn sysspec_output(&self) -> &CosSystem {
        &self.sys
    }
}

impl<'a> Compose<'a> {
    pub fn parse_binaries(spec: &'a ComposeSpec) -> Result<Compose<'a>, String> {
        let mut cs = Vec::new();

        for c in spec.sys.comps().iter() {
            let cbin = spec.binaries.get(c.name()).unwrap();
            cs.push(CompObject::parse(c.name().clone(), cbin)?);
        }

        Ok(Compose {
            spec: spec,
            comp_objs: cs
        })
    }

    pub fn components(&'a self) -> &'a Vec<CompObject<'a>> {
        &self.comp_objs
    }

    // sect_addrs is a vector of tuples of "section name", "address" pairs
    // fn relink(obj: String, sect_addrs: Vec<(String, String)>, tolink: Vec<String>) -> String {

    // }
}
