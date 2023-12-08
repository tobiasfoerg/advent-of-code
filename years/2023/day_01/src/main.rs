use anyhow::Result;

pub fn main() -> Result<()>{
    println!("Hello, world!");
    Ok(())
}


pub(crate) fn part_one() -> Result<u32> {
    todo!();
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_one_test() {
        let result = part_one().unwrap();

        assert_eq!(result, 4);
    }
}
