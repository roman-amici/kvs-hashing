use std::{fs::read_dir, hash::{self, Hash}};

use tokio::{io::{AsyncBufReadExt, BufReader}, sync::Mutex};

use crate::hash_ring::HashRing;

pub struct ServerLocationService {
    hash_ring : Mutex<HashRing>
}

enum Command {
    Add,
    Remove
}


impl ServerLocationService {
    pub fn new(hash_ring : HashRing) -> Self {
        ServerLocationService {
            hash_ring : Mutex::new(hash_ring)
        }
    }

    pub async fn get_hostname(&self, key : &str) -> Option<String> {
        let guard = self.hash_ring.lock().await;

        guard.hostname_for_key(key)
    }

    pub async fn insert_hostname(&self, hostname : &str) {
        let mut guard = self.hash_ring.lock().await;

        guard.insert_server(hostname);
    }

    pub async fn remove_hostname(&self, hostname : &str) {
        let mut guard = self.hash_ring.lock().await;

        guard.remove_hostname(hostname);
    }

    fn read_line(line : &String) -> Result<(Command, String), ()> {
        let mut split = line.split(" ");

        let command = if let Some(command) =  split.next() {
            if command == "add" {
                Command::Add
            } else if command == "remove" {
                Command::Remove
            } else {
                return Err(())
            }
        } else {
            return Err(())
        };

        let hostname = if let Some(hostname) = split.next() {
            hostname
        } else {
            return Err(())
        };

        Ok((command, hostname.to_string()))
    }

    pub async fn listen_for_updates(&self) {


        // Placeholder for reading from an external server / database
        let stdin = tokio::io::stdin();
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        loop {
            if let Some(line) = lines.next_line().await.unwrap() {
                if let Ok((command,hostname)) = Self::read_line(&line) {
                    match command {
                        Command::Add => {
                            self.insert_hostname(&hostname).await;
                            println!("Added {}", hostname);
                        }
                        Command::Remove => {
                            self.remove_hostname(&hostname).await;
                            println!("removed {}", hostname);
                        }
                    }
                } else {
                    println!("Error parsing command");
                }
            } else {
                return;
            }
        }
    }
}
