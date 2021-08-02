#[derive(Debug)]
struct Example{
    name: String,
    age: i8,
    employed: bool,

}

impl Example{
    fn return_name(&self) -> &String{
        &self.name

    }

    fn return_age_function(&self) -> i8 {
        self.age
     }

    fn return_bool_function(&self) -> bool {
        self.employed

    }
}


fn main() {

    
    let person_one = Example{
        name: String::from("Ethan"),
        age: 25,
        employed: true,
    };
    
    let person_two = Example{
        name: String::from("Charlotte"),
        age: 23,
        employed: true,

    };

    println!("{:?}",person_one.return_age_function());
    println!("{:?}",person_one.return_name());
   

    

    


}