use std::collections::HashMap;
use std::error::Error;
use petgraph::graphmap::DiGraphMap;

pub struct OrbitalDirection {
  pub orbiting_body: CelestialBody,
  pub orbited_body: CelestialBody
}

#[derive(Eq, PartialEq, Debug, Clone, PartialOrd, Ord, Hash, Copy)]
pub struct CelestialBody {
  pub one : char,
  pub two : char,
  pub three : char
}


impl OrbitalDirection {
  pub fn parse(value: &str) -> Result<Self, Box<dyn Error>> {
    let parts = value.split(")").collect::<Vec<&str>>();
    let p1 = parts[1].chars().collect::<Vec<char>>();
    let p0 = parts[0].chars().collect::<Vec<char>>();

    Result::Ok(OrbitalDirection {
      orbiting_body: CelestialBody { one: p1[0], two: p1[1], three: p1[2] },
      orbited_body: CelestialBody { one: p0[0], two: p0[1], three: p0[2] }
    })
  }

  pub fn to_edge(&self) -> (CelestialBody, CelestialBody) {
    (self.orbited_body, self.orbiting_body)
  }
}

pub fn generate_map(flat_directions: &Vec<OrbitalDirection>) -> DiGraphMap<CelestialBody, ()> {
  let graph = DiGraphMap::from_edges(flat_directions.iter().map(|d| d.to_edge()));
  graph
}

pub fn calculate_orbits(graph: &DiGraphMap<CelestialBody, ()>) -> u32 {
  // foreach node, calculate path back to beginning and sum
  let mut total = 0;
  for n in graph.nodes() {
    if (n == CelestialBody { one: 'C', two: 'O', three: 'M' }) { continue; }

    total += calculate_orbits_recursive(graph, n, 1)
  }

  total
}

fn calculate_orbits_recursive(graph: &DiGraphMap<CelestialBody, ()>, from: CelestialBody, preceeding_orbits: u32) -> u32 {
  let dir = graph.neighbors_directed(from, petgraph::Direction::Incoming);

  let mut orbits = preceeding_orbits;
  for n in dir {
    if (n == CelestialBody { one: 'C', two: 'O', three: 'M' }) { continue; }

    orbits = calculate_orbits_recursive(&graph, n, preceeding_orbits + 1);
  }
  
  orbits
}

pub fn calculate_ancestors(graph: &DiGraphMap<CelestialBody, ()>, from: CelestialBody) -> HashMap<CelestialBody, u32> {
  let mut ancestors = HashMap::new();

  let mut ancestor = graph.neighbors_directed(from, petgraph::Direction::Incoming).nth(0).unwrap(); // There's always one
  let mut distance = 0;

  while (ancestor != CelestialBody { one: 'C', two: 'O', three: 'M' }) {
    ancestors.insert(ancestor, distance);
    distance += 1;
    ancestor = graph.neighbors_directed(ancestor, petgraph::Direction::Incoming).nth(0).unwrap(); // There's always one
  }

  ancestors
}

pub fn calculate_most_common_ancestor(one: &HashMap<CelestialBody, u32>, two: &HashMap<CelestialBody, u32>) -> u32 {
  let mut min_len = 20000000;

  for (node, value) in one {
    if let Some(two_val) = two.get(node) {
      if value + two_val < min_len {
        min_len = value + two_val;
      }
    }
  }
  
  min_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_input_part1() { 
      let input = "COM)00B
      00B)00C
      00C)00D
      00D)00E
      00E)00F
      00B)00G
      00G)00H
      00D)00I
      00E)00J
      00J)00K
      00K)00L".lines().map(|l| OrbitalDirection::parse(l.trim()).unwrap()).collect();
      let map = generate_map(&input);
      assert_eq!(calculate_orbits(&map), 42);
    }

    #[test]
    fn given_input_part2(){
      let input = "COM)00B
      00B)00C
      00C)00D
      00D)00E
      00E)00F
      00B)00G
      00G)00H
      00D)00I
      00E)00J
      00J)00K
      00K)00L
      00K)YOU
      00I)SAN".lines().map(|l| OrbitalDirection::parse(l.trim()).unwrap()).collect();
      let map = generate_map(&input);
      let santa_ancestors = calculate_ancestors(&map, CelestialBody { one: 'S', two: 'A', three: 'N' });
      let you_ancestors = calculate_ancestors(&map, CelestialBody { one: 'Y', two: 'O', three: 'U' });

      dbg!(&santa_ancestors);
      dbg!(&you_ancestors);

      assert_eq!(calculate_most_common_ancestor(&santa_ancestors, &you_ancestors), 4);
    }
}