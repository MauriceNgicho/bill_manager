Bill Manager
A simple interactive command-line bill and expense manager built with Rust.
This project was created to practice core Rust concepts through a real-world CRUD-style application. Users can add, view, remove, and edit bills directly from the terminal.

Features


Add bills with a name and amount


View all existing bills


Remove bills safely


Edit existing bills


Cancel edits when needed


Input validation for safer interaction



Concepts Practiced
This project helped reinforce several important Rust concepts, including:


Structs and custom data types


Vectors (Vec<T>) for data storage


Ownership and borrowing


Mutable references (&mut)


Pattern matching with match


Error handling using Result


Safe indexing and validation


User input handling with stdin


Loops and interactive program flow


String handling (String vs &str)



Lessons Learned
One of the biggest lessons from this project was understanding how Rust manages memory safely through ownership and borrowing rules.
Working on editing and removing bills also helped demonstrate:


how mutable borrowing works,


how vectors behave during mutation,


and how Rust encourages explicit error handling instead of unsafe assumptions.


This project also showed the importance of choosing the right data structure. A Vec<Bill> was a better fit than a HashMap because the application is list-oriented and relies heavily on ordered indexing.

Running the Project
Clone the repository and run:
cargo run

Future Improvements
Possible future upgrades include:


Persistent file storage


Unique bill IDs


Categories and due dates


Monthly reports


Better error handling


Splitting logic into modules and functions


Using HashMap for advanced lookup scenarios



Author
Built by Maurice Ngicho while learning Rust through hands-on projects.