use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SequenceStyle {
    SequentialHjkl,
    QuadrantAsdf,
}

pub struct KeyMapper {
    style: SequenceStyle,
}

impl KeyMapper {
    pub fn new(style: SequenceStyle) -> Self {
        Self { style }
    }

    /// Assigns unique character sequences to a list of elements.
    /// This returns a map from sequence (e.g., "hj") to the original element's index.
    pub fn generate_mapping(&self, element_count: usize) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        match self.style {
            SequenceStyle::SequentialHjkl => {
                let chars = vec!['h', 'j', 'k', 'l'];
                for i in 0..element_count {
                    let seq = self.index_to_sequence(i, &chars);
                    map.insert(seq, i);
                }
            }
            SequenceStyle::QuadrantAsdf => {
                // Stub for quadrant-based spatial mapping requested by the user.
                // For the prototype, we fall back to sequential assignment using 'a', 's', 'd', 'f'
                let chars = vec!['a', 's', 'd', 'f'];
                for i in 0..element_count {
                    let seq = self.index_to_sequence(i, &chars);
                    map.insert(seq, i);
                }
            }
        }
        map
    }

    fn index_to_sequence(&self, mut index: usize, chars: &[char]) -> String {
        let base = chars.len();
        if index < base {
            return chars[index].to_string();
        }

        let mut seq = String::new();
        loop {
            let rem = index % base;
            seq.insert(0, chars[rem]);
            index /= base;
            if index == 0 {
                break;
            }
            index -= 1; // Shift to make it purely combinatorial without repeating prefixes unnecessarily
        }
        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_hjkl() {
        let mapper = KeyMapper::new(SequenceStyle::SequentialHjkl);
        let mapping = mapper.generate_mapping(10);
        assert_eq!(mapping.len(), 10);
        // Verify unique sequences
        let mut sequences: Vec<_> = mapping.keys().cloned().collect();
        sequences.sort();
        sequences.dedup();
        assert_eq!(sequences.len(), 10);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[test]
    fn test_quadrant_asdf() {
        let mapper = KeyMapper::new(SequenceStyle::QuadrantAsdf);
        let mapping = mapper.generate_mapping(20);
        assert_eq!(mapping.len(), 20);
        let mut sequences: Vec<_> = mapping.keys().cloned().collect();
        sequences.sort();
        sequences.dedup();
        assert_eq!(sequences.len(), 20);

        // Ensure only 'a', 's', 'd', 'f' are used
        for seq in sequences {
            assert!(seq.chars().all(|c| ['a', 's', 'd', 'f'].contains(&c)));
        }
    }
}
