use bobbin_config::BobbinConfig;
use cargo_config::CargoConfig;
use clap::ArgMatches;
use Result;
use toml;
use std::io::Read;
use std::fs::File;
use std::path::Path;


pub fn config(args: &ArgMatches) -> Result<Config> {    
    Ok(Config {
        bobbin: read_bobbin()?,
        cargo: read_cargo()?,
    })
}

#[derive(Debug)]
pub struct Config {
    pub bobbin: Option<BobbinConfig>,
    pub cargo: Option<CargoConfig>,
}

impl Config {
    pub fn target(&self) -> Option<&str> {
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref builder) = bobbin.builder {
                if let Some(ref target) = builder.target {
                    return Some(target)
                }
            }
        }

        if let Some(ref cargo) = self.cargo {
            if let Some(ref build) = cargo.build {
                if let Some(ref target) = build.target {
                    return Some(target)
                }

            }

            if let Some(ref target) = cargo.target {
                if target.keys().len() == 1 {
                    for key in target.keys() {
                        return Some(key)
                    }
                }
            }
        }

        

        None
    }

    pub fn filter_host(&self) -> Option<&str> {    
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref filter) = bobbin.filter {
                if let Some(ref host) = filter.host {
                    return Some(host)
                }
            }
        }
        None        
    }

    pub fn device(&self, args: &ArgMatches) -> Option<String> {
        args.value_of("device").or_else(|| self.filter_device()).map(String::from)
    }

    pub fn filter_device(&self) -> Option<&str> {    
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref filter) = bobbin.filter {
                if let Some(ref device) = filter.device {
                    return Some(device)
                }
            }
        }
        None        
    }

    pub fn console(&self, args: &ArgMatches) -> Option<String> {
        args.value_of("console").or_else(|| self.cfg_console()).map(String::from)
    }

    pub fn cfg_console(&self) -> Option<&str> {    
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref console) = bobbin.console {
                if let Some(ref path) = console.path {
                    return Some(path)
                }
            }
        }
        None        
    }

    pub fn itm_target_clock(&self) -> Option<u32> {
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref itm) = bobbin.itm {
                return itm.target_clock
            }
        }
        None        
    }

    pub fn jlink_device(&self, args: &ArgMatches) -> Option<String> {
        args.value_of("jlink-device").or_else(|| self.cfg_jlink_device()).map(String::from)
    }

    pub fn cfg_jlink_device(&self) -> Option<&str> {        
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref loader) = bobbin.loader {
                if let Some(ref jlink_device) = loader.jlink_device {
                    return Some(jlink_device)
                }
            }
        }
        None
    }    

    pub fn teensy_mcu(&self, args: &ArgMatches) -> Option<String> {
        args.value_of("teensy-mcu").or_else(|| self.cfg_teensy_mcu()).map(String::from)
    }

    pub fn cfg_teensy_mcu(&self) -> Option<&str> {
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref loader) = bobbin.loader {
                if let Some(ref teensy_mcu) = loader.teensy_mcu {
                    return Some(teensy_mcu)
                }
            }
        }
        None        
    }

    pub fn blackmagic_mode(&self, args: &ArgMatches) -> Option<String> {
        args.value_of("blackmagic_mode").or_else(|| self.cfg_blackmagic_mode()).map(String::from)
    }

    pub fn cfg_blackmagic_mode(&self) -> Option<&str> {
        if let Some(ref bobbin) = self.bobbin {
            if let Some(ref loader) = bobbin.loader {
                if let Some(ref blackmagic_mode) = loader.blackmagic_mode {
                    return Some(blackmagic_mode)
                }
            }
        }
        None
    }    
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Option<String>> {
    let path = path.as_ref();
    if path.exists() {
        let mut data = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut data)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

pub fn read_bobbin() -> Result<Option<BobbinConfig>> {    
    if let Some(s) = read_file("./.bobbin/config")? {
        Ok(Some(toml::from_str(&s)?))
    } else {
        Ok(None)
    }
}

pub fn read_cargo() -> Result<Option<CargoConfig>> {
    if let Some(s) = read_file("./.cargo/config")? {
        Ok(Some(toml::from_str(&s)?))
    } else {
        Ok(None)
    }
}
