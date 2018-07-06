use syshelpers::{dump_file};
use toml;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    srv: String,
    interface: String
}

#[derive(Debug, Deserialize)]
pub struct InitArgs {
    key: String,
    value: String,
    at: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct InterfaceVariants {
    interface: String,
    variant: String
}

#[derive(Debug, Deserialize)]
pub struct Component {
    name: String,
    img: String,
    entry_addr: Option<String>,
    baseaddr: Option<String>,
    deps: Option<Vec<Dependency>>,
    initargs: Option<Vec<InitArgs>>,
    variants: Option<Vec<InterfaceVariants>>
}

#[derive(Debug, Deserialize)]
pub struct SysInfo {
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct CosSystem {
    system: SysInfo,
    components: Vec<Component>,
}

impl Dependency {
    pub fn new(name: String) -> Dependency {
        Dependency { srv: name.clone(), interface: "".to_string() }
    }

    pub fn get_name(&self) -> String {
        self.srv.clone()
    }
}

impl Component {
    pub fn new(name: String, img: String) -> Component {
        Component {
            name: name,
            img: img,
            entry_addr: None,
            baseaddr: None,
            deps: Some(Vec::new()),
            initargs: None,
            variants: None
        }
    }

    fn update_deps(&mut self) -> () {
        if self.deps.is_none() {
            let vs = Vec::new();
            self.deps = Some(vs);
        }
    }

    pub fn deps(&self) -> &Vec<Dependency> {
        self.deps.as_ref().unwrap()
    }

    pub fn img(&self) -> &String {
        &self.img
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl CosSystem {
    fn comp_exists(&self, cname: String) -> bool {
        self.comps().iter().fold(false, |found, c2| found || (cname == c2.name))
    }

    // This MUST be called before anything else in the API
    fn validate(&mut self) -> Result<(), String> {
        self.comps_mut().
            iter_mut().
            for_each(|c| c.update_deps());
        let mut fail = false;
        let mut err_accum = String::new();

        // Validate that we don't repeat components.
        //
        // TODO: This should really aggregate error strings with fold,
        // and return them as part of the function's Err, but I
        // appearantly can't rust.
        self.comps().iter().for_each(|c| {
            if 1 < self.comps().iter().fold(0, |n, c2| {
                if c.name != c2.name { n } else { n+1 }
            }) {
                err_accum.push_str(&format!("Error: Component name {} is defined multiple times.", c.name));
                fail = true;
            }
        });

        // Validate that all dependencies resolve to a defined
        // component.
        //
        // TODO: same as above...should aggregate error strings
        for c in self.comps() {
            for d in c.deps() {
                if !self.comp_exists(d.get_name()) {
                    err_accum.push_str(&format!("Error: Cannot find component referenced by dependency {} in component {}.",
                                                d.get_name(), c.name));
                    fail = true;
                }
            }
        }
        // validate that all directed initargs are to declared
        // components
        for c in self.comps() {
            if let Some(ref args) = c.initargs {
                for ia in args.iter() {
                    if let Some(ref name) = ia.at {
                        if !self.comp_exists(name.to_string()) {
                            err_accum.push_str(&format!("Error: Cannot find component referenced by directed initargs {} in component {}.",
                                                        name, c.name));
                            fail = true;
                        }
                    }
                }
            }
        }

        if fail {
            Err(err_accum)
        } else {
            Ok(())
        }
    }

    pub fn comps(&self) -> &Vec<Component> {
        &self.components
    }

    pub fn comps_mut(&mut self) -> &mut Vec<Component> {
        &mut self.components
    }

    pub fn resolve_dep(&self, name: String) -> Option<&Component> {
        self.components.iter().filter(|c| name == c.name).next()
    }

    pub fn parse(sysspec_path: String) -> Result<CosSystem,String> {
        let conf = dump_file(&sysspec_path)?;
        // This is BRAIN DEAD.  There has to be a better way to get a str
        let cossys_pre: Result<CosSystem,_> = toml::from_str(String::from_utf8(conf).unwrap().as_str());

        if let Err(cs) = cossys_pre {
            let mut e = String::from("Error when parsing TOML:\n");
            e.push_str(&format!("{:?}", cs));
            return Err(e);
        }

        let mut cossys = cossys_pre.unwrap();
        if let Err(s) = cossys.validate() {
            let mut e = String::from("Error in system specification:\n");
            e.push_str(&format!("{:?}", s));
            return Err(e);
        }

        Ok(cossys)
    }
}
