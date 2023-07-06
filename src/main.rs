use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn SignUp(username: &mut String, password: &mut String) {
    // Make A SignUp func
    let mut inputforname = String::new(); // create this for inputting the file name

    println!("Hello type the name of the file you will store your user and password in");

    io::stdin().read_line(&mut inputforname).unwrap(); // user input

    let mut inputholder = inputforname; // doing this because it fixed a problem where it would
                                        // create a weird file and so incase if inputforname dies

    fs::File::create(inputholder.trim().clone().to_owned() + ".txt"); // create file

    println!("Ok now you have done that, Enter Your Username: "); // println

    io::stdin().read_line(username).unwrap(); // user input

    fs::write(
        inputholder.trim().clone().to_owned() + ".txt",
        username.trim().to_owned() + "\n",
    ); // write the
       // username to file

    println!("Now Enter The Password: "); // print

    io::stdin().read_line(password).unwrap(); // input password

    let mut myinputfile = OpenOptions::new()
        .append(true)
        .read(true)
        .open(inputholder.trim().clone().to_owned() + ".txt")
        .unwrap();

    writeln!(&mut myinputfile, "{}", password.trim()).unwrap();

    // write password
}

fn Login(username: &mut String, password: &mut String) {
    println!("Hello I see You Signed up and you have a file put the file name to acceses the data");

    let mut inputforname = String::new();

    io::stdin().read_line(&mut inputforname).unwrap();

    let mut inputholder = inputforname;

    let file = fs::File::open(inputholder.trim().clone().to_owned() + ".txt").unwrap();

    let buf = BufReader::new(file);

    let vectorstorer: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    let first_line = &vectorstorer[0];

    let second_line = &vectorstorer[1];

    println!("Ok now you entered your file name,  Enter your username: ");

    io::stdin().read_line(username).unwrap();

    let mut clonedusername = username;

    println!("Now enter your Password");

    io::stdin().read_line(password).unwrap();

    let mut clonedpass = password;

    if clonedusername.trim() == first_line.trim() && clonedpass.trim() == second_line.trim() {
        println!("Sucesses your now logged in");
    } else {
        println!("Stop Trying to fucking hack someones account you fucking weirdo");
    }
}

fn main() {
    let mut username: String = Default::default();
    let mut password: String = Default::default();
    let mut input = String::new();

    println!("Hello Type S to Signup and L to Login");

    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "S" || input.trim() == "s" {
        SignUp(&mut username, &mut password);
    } else if input.trim() == "L" || input.trim() == "l" {
        Login(&mut username, &mut password);
    }
}
