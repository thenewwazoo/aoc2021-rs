
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
            match f {
                &"forward" => Ok(Nav::Fore(dist)),
                &"down" => Ok(Nav::Down(dist)),
                &"up" => Ok(Nav::Up(dist)),
                _ => Err(NavParseError{}),
            }
        } else {
            return Err(NavParseError{});
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
}


/// Errors relating to submarines
#[derive(Debug, PartialEq, Eq)]
pub enum SubError {
    Nav(Nav),
}

impl Sub {
    pub fn try_move(&mut self, m: Nav) -> Result<(), SubError> {
        match m {
            Nav::Fore(d) => {
                self.dist = self.dist.checked_add(d).ok_or(SubError::Nav(m))?;
            },
            Nav::Up(d) => {
                self.depth = self.depth.checked_sub(d).ok_or(SubError::Nav(m))?;
            },
            Nav::Down(d) => {
                self.depth = self.depth.checked_add(d).ok_or(SubError::Nav(m))?;
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
        }
    }
}

#[cfg(test)]
mod sub_tests {

    use super::*;

    #[test]
    fn test_move_sub() {
        let mut s = Sub::default();
        s.try_move(Nav::Fore(1)).unwrap();
        assert_eq!(
            Sub {
                dist: 1,
                depth: 0,
            },
            s
        );

        assert_eq!(
            Err(SubError::Nav(Nav::Up(1))),
            s.try_move(Nav::Up(1)),
        );
    }
}
