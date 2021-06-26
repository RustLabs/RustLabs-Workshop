//declare a struct

struct Course {

    name: String,

    level: String,

    code:i32

}

//impl construct to define struct methods

impl Course {

    fn name_code(&self) -> String {

        format!("{} {}", self.name, self.code)

    }

}



fn main() {

    let course_1 = Course {

        name: "Rust".to_string(),

        level:"beginner".to_string(),

        code:132

    };

    //call the non-static method

    println!("This is a {} course: {}", course_1.level, course_1.name_code());

}

