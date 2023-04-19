use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub struct Val {
    x: f64,
    y: f64,
}

impl Val {
    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Val {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(',').map(|s| s.trim()).collect();

        if values.len() != 2 {
            return Err("Invalid number of values");
        }

        let x = match values[0].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Err("Invalid x value"),
        };

        let y = match values[1].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Err("Invalid y value"),
        };

        Ok(Val { x, y })
    }
}

#[derive(Clone)]
pub struct ValVec(Vec<Val>);

impl ValVec {
    pub fn from_vec(vec: Vec<Val>) -> Self {
        ValVec(vec)
    }
}

impl FromStr for ValVec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(';').collect();

        let mut vec = Vec::with_capacity(values.len());

        for val_str in values {
            let val = match Val::from_str(val_str.trim()) {
                Ok(val) => val,
                Err(_) => return Err("Invalid value"),
            };
            vec.push(val);
        }

        Ok(ValVec(vec))
    }
}

impl fmt::Display for ValVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, val) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }

        write!(f, "]")
    }
}

impl From<ValVec> for Vec<Val> {
    fn from(val_vec: ValVec) -> Vec<Val> {
        val_vec.0
    }
}
