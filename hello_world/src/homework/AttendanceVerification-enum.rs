#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        // use match statement)
        println!("My name is {}",self.name);

        let grade_msg = match self.grade {
           GradeLevel::Bachelor => "I am a Bachelor",
           GradeLevel::Master => "I am a Master",
           GradeLevel::PhD => "I have a PhD",
        };
        println!("{}",grade_msg);

        let major_msg = match self.major {
            Major::ComputerScience => "I am a ComputerScience major",
            Major::ElectricalEngineering => "I am a Electrical Engineering major",
         };
         println!("{}",major_msg);


    }
}

fn main() {
    let s1 = Student::new("Asael".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
    s1.introduce_yourself();
}
