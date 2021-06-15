fn main() {

    //String from &str
    let s1 = String::from("123");
    let s2 = "123".to_string();

    //combine strings
    let combine_literals = ["first", "second"].concat(); //produces String
    let combine_w_macro = format!("{} {}", "first", "second"); //produces String
    let combine_String_and_str = String::from("abc") + "def"; //produces String. Note that we're aadding String and &str, in this order, or it won't work
    let combine_String_and_str2 = String::from("abc") + &String::from("abc");

    //append a string
    let mut_string = String::new();
    let combine_w_push = mut_string.push_str("123");

    //apend a char
    let combine_w_push = mut_string.push('a');

    //get a certain character
    let char_by_index = "123".chars().nth(1).unwrap();

}