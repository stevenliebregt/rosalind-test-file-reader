use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct RosalindIO {
    pub input: String,
    pub output: String,
}

impl RosalindIO {
    pub fn from_path(path: &str) -> Result<RosalindIO, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;

        Ok(RosalindIO {
            // Start 6 chars in, input = 5 and a newline char makes 6
            input: String::from(contents[6..contents.find("Output").unwrap()].trim()),
            // Start 7 chars after starting pos of Output
            output: String::from(contents[(contents.find("Output").unwrap() + 7)..].trim()),
        })
    }

    pub fn from_strings(input: String, output: String) -> RosalindIO {
        RosalindIO { input, output }
    }
}

#[cfg(test)]
mod tests {
    use crate::RosalindIO;

    #[test]
    fn it_works() {
        assert_eq!(
            RosalindIO::from_path("data/test-1.txt").unwrap(),
            RosalindIO {
                input: String::from("Hello"),
                output: String::from("World"),
            }
        );
    }
}
