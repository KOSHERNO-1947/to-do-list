use std::io::{stdin};
fn show(l: Vec<String>) {
	let mut num: i32 = 0;
	for i in l.iter() {
		println!("{} {}", num, *i);
		num+=1;
	}
}
fn delete(mut l: Vec<String>) -> Vec<String> {
	println!("Write number: \n");
	let mut input: String = String::new();
	stdin().read_line(&mut input).unwrap();
	input.pop();
	let num: i32 = input.parse::<i32>().unwrap();
	l.remove(0);
	return l;
}
fn add(mut l: Vec<String>) -> Vec<String> {
	println!("Write a task: \n");
	let mut input: String = String::new();
	stdin().read_line(&mut input).unwrap();
	l.push(input.clone());
	return l;
}
fn main() {
	let mut list: Vec<String> = vec![];
	println!("Welcome to my to-do list CLI app");
	println!("Write a task to complete:");
	let mut i: String = String::new();
	stdin().read_line(&mut i).unwrap();
	list.push(i.clone());
	println!("Congrats, you wrote your first task");
	println!("You can use these operations: add, delete, exit, show");
	loop {
		let mut input: String = String::new();
		stdin().read_line(&mut input).unwrap();
		if input == "add\n" {list = add(list.clone());}
		if input == "delete\n" {list = delete(list.clone());}
		if input == "exit\n" {break;}
		if input == "show\n" {show(list.clone());}
		if input != "add\n" && input != "delete\n" && input != "exit\n" && input != "show\n"  {println!("Wrong input"); println! ("{}", input);}
	}
}
