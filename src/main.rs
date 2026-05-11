use std::io;
struct Bill {
    name: String,
    amount: f64,
}
fn main() {
    let mut bills: Vec<Bill> = Vec::new();

    loop {
        println!("\n== Welcome to Bill Manager ==");
        println!("1: Add Bill");
        println!("2: View Bills");
        println!("3: Remove Bill");
        println!("4: Edit Bill");
        println!("5: Exit");
        println!("Please choose an option:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let mut name = String::new();

                println!("Enter bill name:");
                io::stdin()
                    .read_line(&mut name)
                    .expect("No name entered");

                if name.trim().is_empty() {
                    println!("Bill name cannot be empty. Please try again.");
                    continue;
                }

                let mut amount = String::new();

                println!("Enter bill amount:");
                io::stdin()
                    .read_line(&mut amount)
                    .expect("No amount entered");

                let amount: f64 = amount
                    .trim()
                    .parse::<f64>()
                    .expect("Please enter a valid number for the amount");

                let bill = Bill {
                    name: name.trim().to_owned(),
                    amount,
                };
                bills.push(bill);
                println!("Bill added successfully!");
            }
            "2" => {
                if bills.is_empty() {
                    println!("No bills to display.");
                } else {
                    for (index, bill) in bills.iter().enumerate() {
                        println!(
                            "{}. {} - Ksh{:.2}",
                            index + 1,
                            bill.name,
                            bill.amount
                        );
                    }
                }
            }
            "3" => {
                if bills.is_empty() {
                    println!("No bills to remove.");
                } else {
                    for (index, bill) in bills.iter().enumerate() {
                        println!(
                            "{}. {} - Ksh{:.2}",
                            index + 1,
                            bill.name,
                            bill.amount
                        );
                    }
                    println!("Enter bill number to remove:");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");

                    let index: usize = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Please enter a valid number.");
                            continue;
                        }
                    };
                    if index == 0 {
                        println!("Invalid bill number.");
                        continue;
                    }
                    let index = index - 1;
                    if index >= bills.len() {
                        println!("Bill number does not exist.");
                        continue;
                    }
                    let removed_bill = bills.remove(index);
                    println!(
                        "Removed bill: {} - Ksh{:.2}",
                        removed_bill.name, removed_bill.amount
                    );
                }
            }
            "4" => {
                if bills.is_empty() {
                    println!("No bills to edit.");
                    continue;
                }

                for (index, bill) in bills.iter().enumerate() {
                    println!(
                        "{}. {} - ${:.2}",
                        index + 1,
                        bill.name,
                        bill.amount
                    );
                }

                println!("Enter bill number to edit:");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let index: usize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number.");
                        continue;
                    }
                };

                if index == 0 {
                    println!("Invalid bill number.");
                    continue;
                }

                let index = index - 1;

                if index >= bills.len() {
                    println!("Bill number does not exist.");
                    continue;
                }

                let bill = &mut bills[index];

                println!("Current name: {}", bill.name);
                println!("Current amount: ${:.2}", bill.amount);

                println!("Enter new bill name (or type CANCEL):");

                let mut new_name = String::new();

                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read input");

                let new_name = new_name.trim();

                if new_name.eq_ignore_ascii_case("cancel") {
                    println!("Edit cancelled.");
                    continue;
                }

                println!("Enter new amount:");

                let mut new_amount = String::new();

                io::stdin()
                    .read_line(&mut new_amount)
                    .expect("Failed to read input");

                let new_amount: f64 = match new_amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount.");
                        continue;
                    }
                };

                bill.name = new_name.to_string();
                bill.amount = new_amount;

                println!("Bill updated successfully!");
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

