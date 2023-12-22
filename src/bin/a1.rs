#[derive(Debug)]
enum Day{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

struct Date{
    year: i32,
    month: i32,
    day: i32,
    day_of_week: Day
}

fn main(){
    let birthday = Date{year: 1995, month: 9, day: 26, day_of_week: Day::Tuesday};
    display_date(&birthday);
    display_month(&birthday);
}

fn display_date(date: &Date){
    println!("{:?}/{:?}/{:?} is a {:?}", date.month, date.day, date.year, date.day_of_week);
}

fn display_month(date: &Date){
    match date.month{
        1 => println!("January"),
        2 => println!("February"),
        3 => println!("March"),
        4 => println!("April"),
        5 => println!("May"),
        6 => println!("June"),
        7 => println!("July"),
        8 => println!("August"),
        9 => println!("September"),
        10 => println!("October"),
        11 => println!("November"),
        12 => println!("December"),
        _ => println!("Invalid month")
    }
}