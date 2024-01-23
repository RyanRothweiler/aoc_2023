pub struct Mapping {
    start: i64,
    range: i64,
    dif: i64,
}

impl Mapping {
    pub fn new(dest: i64, source: i64, range: i64) -> Mapping {
        Mapping {
            start: source,
            range: range,
            dif: source - dest,
        }
    }

    pub fn parse_list(lineizer: &mut super::Lineizer) -> Vec<Mapping> {
        let mut ret: Vec<Mapping> = vec![];

        while lineizer.valid() && lineizer.lines[lineizer.current_line].len() > 1 {
            let inputs: Vec<&str> = lineizer.lines[lineizer.current_line].split(' ').collect();

            let dest: i64 = inputs[0].trim().parse().unwrap_or_else(|error| 0);
            let source: i64 = inputs[1].trim().parse().unwrap_or_else(|error| 0);
            let range: i64 = inputs[2].trim().parse().unwrap_or_else(|error| 0);

            ret.push(Mapping::new(dest, source, range));

            lineizer.current_line = lineizer.current_line + 1;
        }

        return ret;
    }

    pub fn within_range(&self, input: i64) -> bool {
        return (input >= self.start) && (input - self.start) < self.range;
    }

    pub fn convert(&self, input: i64) -> i64 {
        if self.within_range(input) {
            return input - self.dif;
        }

        return input;
    }
}

#[test]
fn conversion() {
    let conv = Mapping::new(50, 98, 2);

    assert_eq!(conv.convert(95), 95);
    assert_eq!(conv.convert(96), 96);
    assert_eq!(conv.convert(97), 97);

    assert_eq!(conv.convert(98), 50);
    assert_eq!(conv.convert(99), 51);

    assert_eq!(conv.convert(100), 100);
    assert_eq!(conv.convert(101), 101);
}
