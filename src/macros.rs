#[macro_export]
macro_rules! string {
    ( $x:expr ) => {{
        let string = json::JsonValue::String($x.to_string());
        string
    }};
}

#[macro_export]
macro_rules! number {
    ( $x:expr ) => {{
        let number = json::JsonValue::Number(json::number::Number::from($x));
        number
    }};
}

#[macro_export]
macro_rules! bool {
    ( $x:expr ) => {{
        let boolean = json::JsonValue::Boolean($x);
        boolean
    }};
}

#[macro_export]
macro_rules! json_array {
    ( $( $x:expr ),* ) => {
        {
            let mut json_array = json::JsonValue::Array(json::Array::new());
            $(
                json_array.push($x).unwrap_or_else(|error| {
                    println!("{}", format!("Error: {}", error).red());
                    panic!();
                });
            )*
            json_array
        }
    };
}
