fn main() {
    let number_list = vec![34,50,60,45,12];
    let mut largest_number = number_list[0];

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }

    println!("{}", largest_number);

    let number_list2 = vec![304,150,6550,450,132];

    let mut largest = number_list2[0];
}
