use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct HostsFile;

#[derive(Debug,Clone)]
pub struct HostsFileAddresses(HashMap<String,Vec<String>>);

impl HostsFile {
    pub fn get_default_path() -> PathBuf {
        // Final Path
        let final_path: PathBuf = PathBuf::from("System32\\drivers\\etc\\hosts");
        
        // System Root
        let system_root = std::env::var("SystemRoot").unwrap();
        let mut HOSTS_PATH = PathBuf::from(&system_root);
        HOSTS_PATH.push(final_path);
        
        return HOSTS_PATH
    }
    
    pub fn new<T: AsRef<Path>>(hosts_path: T) -> HostsFileAddresses {
        let file = match File::open(hosts_path.as_ref()) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Failed to open hosts file");
                std::process::exit(1);
            }
        };
        
        let reader = BufReader::new(file);
        let mut hosts_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for line in reader.lines() {
            if let Ok(line) = line {
                // Skip comment lines and empty lines
                if line.starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    // Invalid line format, skip
                    continue;
                }
                
                let ip_address = parts[0].trim().to_string();
                let hostnames: Vec<String> = parts[1..].iter().map(|s| s.trim().to_string()).collect();
                
                // Insert into HashMap
                if let Some(existing) = hosts_map.get_mut(&ip_address) {
                    existing.extend(hostnames);
                } else {
                    hosts_map.insert(ip_address, hostnames);
                }
            }
        }
        
        return HostsFileAddresses(hosts_map)
    }
    pub fn read_file(addresses: HostsFileAddresses) {
        for (ip_address, hostnames) in addresses.0 {
            println!("IP: {}", ip_address);
            for hostname in hostnames {
                println!("  Hostname: {}", hostname);
            }
        }
    }
}

impl HostsFileAddresses {
    pub fn read(&self) {
        let h = &self.0;

        if h.is_empty() {
            println!("[X] HOSTS File is empty or all commented out");
        }

        else {
            for (ip_address, hostnames) in &self.0 {
                println!("IP: {}", ip_address);
                for hostname in hostnames {
                    println!("  Hostname: {}", hostname);
                }
            }
        }
    }
    /// Checks whether addresses/hostnames is empty
    pub fn has_no_addresses(&self) -> bool {
        let map = &self.0;

        if map.is_empty() {
            return true
        }
        else {
            return false
        }
    }
    /// Checks whether it has address/hostnames on it
    pub fn has_addresses(&self) -> bool {
        let map = &self.0;

        if map.is_empty() {
            return false
        }
        else {
            return true
        }
    }
    pub fn get_all(&self) -> &HashMap<String, Vec<String>> {
        return &self.0
    }
}
mod tests {
    use super::*;
    
    #[test]
    fn test_file() {
        let default_path = HostsFile::get_default_path();
        let hosts = HostsFile::new(default_path);
        hosts.read();
    }
}