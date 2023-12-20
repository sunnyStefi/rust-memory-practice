#[cfg(test)]
mod tests_simple {
    #[test]
    #[ignore]
    fn working_sum(){
        let res = 2+2;
        assert_eq!(res,4);
    }
}

#[derive(Debug)]
struct Line {
    len: u32
}

impl Line {
    fn is_bigger(&self, other: &Line) -> bool {
        return self.len > other.len
    }
}
#[cfg(test)]
mod tests_line_same_name {
    use super::*;
    #[test]
    fn test_is_bigger(){
        let line1 = Line {
            len : 3
        };
        let line2 = Line {
            len : 4
        };
        assert!(line2.is_bigger(&line1));
    }
}

#[cfg(test)]
mod test_result_same_name {
    #[test]
    fn test_result() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Err"))
        }
    }
}

#[cfg(test)]
pub mod adder {
    #[test]
    fn add_two(number :i32) -> i32
    {
        number +2
    }
}