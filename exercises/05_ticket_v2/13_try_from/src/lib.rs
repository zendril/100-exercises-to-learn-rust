// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
#[error("{invalid_status} is not a valid status")]
struct ParseStructError {
    invalid_status: String,
}

impl TryFrom<String> for Status {
    type Error = ParseStructError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
        // match value.to_lowercase().as_str() {
        //     "todo" => Ok(Self::ToDo),
        //     "done" => Ok(Self::Done),
        //     "inprogress" => Ok(Self::InProgress),
        //     _ => Err("Cannot Convert".to_string())
        // }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStructError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::ToDo),
            "done" => Ok(Self::Done),
            "inprogress" => Ok(Self::InProgress),
            _ => Err(ParseStructError {invalid_status: value.to_string()})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
