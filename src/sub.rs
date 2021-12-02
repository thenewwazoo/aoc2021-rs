
/// Possible moves your sub can make
#[derive(Debug, PartialEq, Eq)]
pub enum Nav {
    /// Sub moves forward
    Fore(u64),
    /// Sub moves up (depth decreases)
    Up(u64),
    /// Sub moves down (depth increases)
    Down(u64),
}

/// Errors resulting from attempts to parse nav input
#[derive(Debug, PartialEq, Eq)]
pub struct NavParseError;

impl TryFrom<&str> for Nav {
    type Error = NavParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {

        if let [f, d] = &(line.split(' ').collect::<Vec<&str>>()).as_slice() {
            let dist = d.parse::<u64>().map_err(|_| NavParseError{})?;
            match *f {
                "forward" => Ok(Nav::Fore(dist)),
                "down" => Ok(Nav::Down(dist)),
                "up" => Ok(Nav::Up(dist)),
                _ => Err(NavParseError{}),
            }
        } else {
            Err(NavParseError{})
        }
    }
}

/// A representation of the state of your submarine
#[derive(Debug, PartialEq, Eq)]
pub struct Sub {
    /// How far forward the sub has travelled
    pub dist: u64,
    /// How deep the sub is (deeper -> higher value)
    pub depth: u64,
    /// The amount of incline the sub has (neg -> pointed up, pos -> pointed down)
    pub aim: i64,
}


/// Errors relating to submarines
#[derive(Debug, PartialEq, Eq)]
pub enum SubError {
    Overflow(u64),
    Nav(Nav),
}

impl Sub {
    pub fn try_move(&mut self, m: Nav) -> Result<(), SubError> {
        match m {
            Nav::Fore(d) => {
                self.dist = self.dist.checked_add(d).ok_or(SubError::Nav(m))?;

                let depth_adj: i64 = self.aim * i64::try_from(d).map_err(|_| SubError::Overflow(d))?;
                if depth_adj > 0 {
                    self.depth = self.depth
                        .checked_add(
                            depth_adj.try_into().unwrap()
                        )
                        .ok_or(SubError::Overflow(d))?;
                } else {
                    self.depth = self.depth
                        .checked_sub(
                            depth_adj.abs().try_into().unwrap()
                        ).ok_or(SubError::Overflow(d))?;
                }

            },
            Nav::Up(d) => {
                let adj = d.try_into().map_err(|_| SubError::Overflow(d))?;
                self.aim = self.aim.checked_sub(adj).ok_or(SubError::Overflow(d))?;
            },
            Nav::Down(d) => {
                let adj = d.try_into().map_err(|_| SubError::Overflow(d))?;
                self.aim = self.aim.checked_add(adj).ok_or(SubError::Overflow(d))?;
            },
        }
        Ok(())
    }
}

impl std::fmt::Display for Sub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "< dist: {}, depth: {} >", self.dist, self.depth)
    }
}

impl Default for Sub {
    fn default() -> Self {
        Sub{
            dist: 0,
            depth: 0,
            aim: 0,
        }
    }
}

#[cfg(test)]
mod sub_tests {

    use super::*;

    #[test]
    fn test_move_sub_level() {
        let mut s = Sub::default();
        s.try_move(Nav::Fore(1)).unwrap();
        assert_eq!(
            Sub {
                dist: 1,
                depth: 0,
                aim: 0,
            },
            s
        );
    }

    #[test]
    fn test_move_sub_down() {
        let mut s = Sub { dist: 5, depth: 0, aim: 0 };
        s.try_move(Nav::Down(5)).unwrap();
        s.try_move(Nav::Fore(8)).unwrap();
        assert_eq!(
            Sub {
                dist: 13,
                depth: 40,
                aim: 5,
            },
            s
        );
    }

}
