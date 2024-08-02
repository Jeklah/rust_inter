use std::io;

fn faren_to_celcius(faren: f32) -> f32 {
    (faren - 32.0) * 5.0 / 9.0
}

fn celcius_to_faren(celcius: f32) -> f32 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter the temperature in Farenheit: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let celcius = faren_to_celcius(input.trim().parse().expect("Please enter a valid number"));
        println!("The temperature in Celcius is: {}", celcius);

        println!("Would you like to convert a celcius temperature to farenheit? (y/n)");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "y" {
            println!("Enter the temperature in Celcius: ");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let faren =
                celcius_to_faren(input.trim().parse().expect("Please enter a valid number"));
            println!("The temperature in Farenheit is: {}", faren);
        } else {
            println!("Goodbye!");
            break;
        }
    }
}
