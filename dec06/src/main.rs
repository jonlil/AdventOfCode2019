fn orbits() {

}

fn main() {
}

#[derive(Debug, PartialEq)]
struct OrbitRelation {
    center: String,
    orbit: String,
}

struct Graph {
    nodes: Vec<()>,
}


impl OrbitRelation {
    pub fn new(center: String, orbit: String) -> Self {
        OrbitRelation {
            center,
            orbit,
        }
    }
}

impl std::str::FromStr for OrbitRelation {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = input.split(")")
            .map(|s| s.to_string()).collect();

        if parts.len() != 2 {
            return Err("Failed parsing orbit relations");
        }
        Ok(OrbitRelation {
            orbit: parts[0].to_owned(),
            center: parts[1].to_owned(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_map_orbit_relationships() {
        let input: Vec<OrbitRelation> = vec![
            "COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L",
        ].iter().map(|value| value.parse().unwrap()).collect();
    }

    #[test]
    fn it_can_parse_orbit_relationship() {
        /// A)B Meeans B is in orbit around A
        let orbit_relation: Result<OrbitRelation, &'static str> = "B)C".parse();

        assert_eq!(orbit_relation, Ok(OrbitRelation::new("C".to_string(), "B".to_string())));
    }
}
