// struct User {
//     username: String,
//     email: String,
//     active:bool,
//     sing_in_ount: u64
// }
// struct Color (u8, u8, u8);

// struct Point (u8, u8, u8);
// fn main() {

//     let mut user_1 = build_user(String::from("saju"), String::from("gmail"));
//     println!("the value of userName {}", user_1.username);
//     user_1.username.push_str(" kabir");
//     println!("the value of userName {}", user_1.username);

//     let user_2 = User {
//         username: String::from("saju"),
//         ..user_1
//     };
//     println!("the value {}", user_2.active);

//     let black = Color(0,0,0);
//     let white= Color(0,0,0);

//     set_bg_color(black);
// }

// fn set_bg_color(color: Color){
//     println!("{} {} {}", color.0, color.1, color.2);
// }

// fn build_user(username: String, email: String)-> User {
//     User {
//     username,
//     email,
//     active: true,
//     sing_in_ount:1
//    }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    // fn calculate_rectangle_area(self: &Self)-> u32{
    fn calculate_rectangle_area(&self)-> u32{
        self.width*self.height
    }
}

fn main() {
    println!("hey there");
    let rect= Rectangle{
        width:20,
        height:20
    };
//    let area =calculate_rectangle_area(&rect);
    println!("the value of area {}",  rect.calculate_rectangle_area());
    // println!("the {:?}", rect);
    // println!("the pretty {:#?}", rect);
    // dbg!(&rect);  // it give the file and the limne and it can return a value too

}
