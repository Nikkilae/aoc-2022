use std::{error::Error, path::Path};

use solutions::*;

fn read_test_file<P: AsRef<Path>>(relative_path: P) -> Result<String, Box<dyn Error>> {
    let full_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("input")
        .join(relative_path);
    Ok(std::fs::read_to_string(full_path)?)
}

#[test]
fn day1() -> Result<(), Box<dyn Error>> {
    assert_eq!(d1::solve_part_1(&read_test_file("d1_test.txt")?)?, "24000");
    assert_eq!(
        d1::solve_part_1(&read_test_file("d1_1_real.txt")?)?,
        "70296"
    );
    assert_eq!(d1::solve_part_2(&read_test_file("d1_test.txt")?)?, "45000");
    assert_eq!(
        d1::solve_part_2(&read_test_file("d1_2_real.txt")?)?,
        "205381"
    );
    Ok(())
}

#[test]
fn day2() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d2::solve_part_1(&read_test_file("d2_test.txt")?)?, "15");
    assert_eq!(&d2::solve_part_1(&read_test_file("d2_real.txt")?)?, "13221");
    assert_eq!(&d2::solve_part_2(&read_test_file("d2_test.txt")?)?, "12");
    assert_eq!(&d2::solve_part_2(&read_test_file("d2_real.txt")?)?, "13131");
    Ok(())
}

#[test]
fn day3() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d3::solve_part_1(&read_test_file("d3_test.txt")?)?, "157");
    assert_eq!(&d3::solve_part_1(&read_test_file("d3_real.txt")?)?, "8243");
    assert_eq!(&d3::solve_part_2(&read_test_file("d3_test.txt")?)?, "70");
    assert_eq!(&d3::solve_part_2(&read_test_file("d3_real.txt")?)?, "2631");
    Ok(())
}

#[test]
fn day4() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d4::solve_part_1(&read_test_file("d4_test.txt")?)?, "2");
    assert_eq!(&d4::solve_part_1(&read_test_file("d4_real.txt")?)?, "584");
    assert_eq!(&d4::solve_part_2(&read_test_file("d4_test.txt")?)?, "4");
    assert_eq!(&d4::solve_part_2(&read_test_file("d4_real.txt")?)?, "933");
    Ok(())
}

#[test]
fn day5() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d5::solve_part_1(&read_test_file("d5_test.txt")?)?, "CMZ");
    assert_eq!(
        &d5::solve_part_1(&read_test_file("d5_real.txt")?)?,
        "HNSNMTLHQ"
    );
    assert_eq!(&d5::solve_part_2(&read_test_file("d5_test.txt")?)?, "MCD");
    assert_eq!(
        &d5::solve_part_2(&read_test_file("d5_real.txt")?)?,
        "RNLFDJMCT"
    );
    Ok(())
}

#[test]
fn day6() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d6::solve_part_1(&read_test_file("d6_test.txt")?)?, "5");
    assert_eq!(&d6::solve_part_1(&read_test_file("d6_real.txt")?)?, "1855");
    assert_eq!(&d6::solve_part_2(&read_test_file("d6_test.txt")?)?, "23");
    assert_eq!(&d6::solve_part_2(&read_test_file("d6_real.txt")?)?, "3256");
    Ok(())
}

#[test]
fn day7() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d7::solve_part_1(&read_test_file("d7_test.txt")?)?, "95437");
    assert_eq!(
        &d7::solve_part_1(&read_test_file("d7_real.txt")?)?,
        "1749646"
    );
    assert_eq!(
        &d7::solve_part_2(&read_test_file("d7_test.txt")?)?,
        "24933642"
    );
    assert_eq!(
        &d7::solve_part_2(&read_test_file("d7_real.txt")?)?,
        "1498966"
    );
    Ok(())
}

#[test]
fn day8() -> Result<(), Box<dyn Error>> {
    assert_eq!(&d8::solve_part_1(&read_test_file("d8_test.txt")?)?, "21");
    assert_eq!(&d8::solve_part_1(&read_test_file("d8_real.txt")?)?, "1805");
    assert_eq!(&d8::solve_part_2(&read_test_file("d8_test.txt")?)?, "8");
    assert_eq!(
        &d8::solve_part_2(&read_test_file("d8_real.txt")?)?,
        "444528"
    );
    Ok(())
}
