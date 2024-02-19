use std::{collections::{hash_map::{self, DefaultHasher}, BTreeMap}, hash::{Hash, Hasher}};

use rand::{RngCore, SeedableRng};

pub struct ServerLocation {
    hostname : String,
    ring_location : u64
}

pub struct HashRing {
    max_value : u64,
    num_copies: u64,
    server_locations : BTreeMap<u64, ServerLocation>,
}

impl HashRing {
    pub fn new(max_value : u64, num_copies : u64) -> Self {
        HashRing {
            max_value,
            num_copies,
            server_locations : BTreeMap::new()
        }

    }

    pub fn hostname_for_key(&self, key: &str) -> Option<String> {
        let mut hasher  = hash_map::DefaultHasher::new();

        key.hash(&mut hasher);
        let node = hasher.finish() % self.max_value;

        self.hostname_for_node(node)
    }

    fn hostname_for_node(&self, node : u64) -> Option<String> {

        assert!(node < self.max_value);

        if let Some(back) = self.server_locations.range(node..).next() {
            Some(back.1.hostname.clone())
        } else if let Some(front) = self.server_locations.first_key_value() {
            Some(front.1.hostname.clone())
        } else {
            None
        }
    }

    pub fn remove_hostname(&mut self, hostname : &str) {
        let to_remove : Vec<u64> = self.server_locations.iter().filter_map(|x| if &x.1.hostname == hostname { Some(*x.0) } else {None}).collect();

        for i in to_remove {
            self.server_locations.remove(&i);
        }
    }

    pub fn insert_server(&mut self, hostname: &str) {
        let mut hasher  = hash_map::DefaultHasher::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(23423);

        hostname.hash(&mut hasher);

        for _ in 0..self.num_copies {
            hasher.write_u64(rng.next_u64());
            let node = hasher.finish() % self.max_value;
            self.server_locations.insert(node, ServerLocation {
                hostname : hostname.to_string(),
                ring_location : node
            });
        }
    }

}

#[cfg(test)]
mod hash_ring_text {
    use std::panic::UnwindSafe;

    use super::HashRing;

    #[test]
    fn one_server_insert_retrieve() {
        let mut ring = HashRing::new(32_567, 1);

        ring.insert_server("server1.com");

        assert_eq!("server1.com", &ring.hostname_for_key("k1").unwrap());
        assert_eq!("server1.com", &ring.hostname_for_key("k2").unwrap());
        assert_eq!("server1.com", &ring.hostname_for_key("k3").unwrap());

    }

    #[test]
    fn two_servers_consistent_over()
    {
        let mut ring = HashRing::new(32_567, 1);

        ring.insert_server("server1.com");
        ring.insert_server("server2.com");

        for i in 0..ring.max_value {
            // Make sure there's nothing stateful here
            assert_eq!(ring.hostname_for_node(i).unwrap(),ring.hostname_for_node(i).unwrap())
        }
    }

    #[test]
    fn two_servers_switchover()
    {
        let mut ring = HashRing::new(1028, 1);

        ring.insert_server("server1.com");
        ring.insert_server("server2.com");

        let mut num_flips = 0;
        let mut last = ring.hostname_for_node(0).unwrap();
        for i in 1..ring.max_value {
            let new = ring.hostname_for_node(i).unwrap();
            if new != last {
                num_flips += 1;
                last = new;
            }
        }

        assert_eq!(num_flips, 2);
    }

    #[test]
    pub fn two_servers_multi_insert_switchover() {
        let mut ring = HashRing::new(128_456, 1024);

        ring.insert_server("server1.com");
        ring.insert_server("server2.com");

        let mut num_flips = 0;
        let mut last = ring.hostname_for_node(0).unwrap();
        for i in 1..ring.max_value {
            let new = ring.hostname_for_node(i).unwrap();
            if new != last {
                num_flips += 1;
                last = new;
            }
        }

        assert!(num_flips > (1024 / 2));
    }

    #[test]
    pub fn add_server_rehash()
    {
        let mut ring = HashRing::new(128_456, 32);

        for i in 0..16 {
            let s = format!("server{}.com", i);
            ring.insert_server(&s);
        }

        let mut first_hash = vec![];
        for i in 0..ring.max_value {
            first_hash.push(ring.hostname_for_node(i));
        }

        ring.insert_server("server16.com");

        let mut second_hash = vec![];
        for i in 0..ring.max_value {
            second_hash.push(ring.hostname_for_node(i))
        }

        let num_differences : usize = first_hash
            .iter()
            .zip(second_hash.iter())
            .map(|(x1,x2)| {
                if x1.as_ref().unwrap() == x2.as_ref().unwrap() { 0 } else { 1 }
            }).sum();

        let average_differences = ring.max_value / 17;
        let percent_difference = (num_differences as f64 - average_differences as f64).abs() / (average_differences as f64);

        assert!(  percent_difference < 0.1  );
    }

    #[test]
    pub fn remove_server_rehash()
    {
        let mut ring = HashRing::new(128_456, 32);

        for i in 0..16 {
            let s = format!("server{}.com", i);
            ring.insert_server(&s);
        }

        let mut first_hash = vec![];
        for i in 0..ring.max_value {
            first_hash.push(ring.hostname_for_node(i));
        }

        ring.remove_hostname("server15.com");

        let mut second_hash = vec![];
        for i in 0..ring.max_value {
            second_hash.push(ring.hostname_for_node(i))
        }

        let num_differences : usize = first_hash
            .iter()
            .zip(second_hash.iter())
            .map(|(x1,x2)| {
                if x1.as_ref().unwrap() == x2.as_ref().unwrap() { 0 } else { 1 }
            }).sum();

        let average_differences = ring.max_value / 16;
        let percent_difference = (num_differences as f64 - average_differences as f64).abs() / (average_differences as f64);

        assert!(  percent_difference < 0.1  );
    }


    #[test]
    pub fn remove_hostname_removes_all_servers()
    {
        let mut ring = HashRing::new(128_456, 32);

        for i in 0..16 {
            let s = format!("server{}.com", i);
            ring.insert_server(&s);
        }

        ring.remove_hostname("server1.com");

        let contains_host = ring.server_locations.values().any(|x| &x.hostname == "server1.com" );
        assert!(!contains_host);
    }
}
