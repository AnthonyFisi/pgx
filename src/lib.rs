use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_my_extension() -> &'static str {
    "Hello, my_extension"
}

#[pg_extern]
fn to_lowercase(input: Option<&str>) -> String{
    info!("we are converting {:?} to lowercase ", input);

    match input {
        Some(s) => s.to_lowercase(),
        None => String::from("INPUT WAS NULL"),
    }
    //input.to_lowercase()
}

#[pg_extern]
fn sum_array(input: Vec<Option<i64>>) -> i64 {
    //input.into_iter().sum()
    input.into_iter().map(|i| i.unwrap_or(0)).sum()
}


#[pg_extern]
fn my_generate_series(start : i64,end: i64,step: default!(i64,1)) -> impl std::iter::Iterator<Item = i64> {
    (start..=end).into_iter().step_by(step as usize)
    //54
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_my_extension() {
        assert_eq!("Hello, my_extension", crate::hello_my_extension());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
