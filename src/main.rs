#[derive(Debug)] // Derive the Debug trait for the Food struct, allowing it to be printed with println!("{:?}", food).
struct Food {
    name: String, // The name of the food item.
}

#[derive(Debug)] // Derive the Debug trait for the Restaurant struct.
struct Restaurant {
    reservations: u32,          // Number of reservations currently held.
    has_mice_infestation: bool, // Indicates whether the restaurant has a mice infestation.
}

impl Restaurant {
    // Implementation block for the Restaurant struct.
    fn book_reservation(&self) -> Option<bool> {
        // Method to book a reservation. Returns an Option<bool>.
        if self.has_mice_infestation {
            // Check if the restaurant has a mice infestation.
            None // If so, return None, indicating that a reservation cannot be made.
        } else if self.reservations < 12 {
            // If no mice, check if there are less than 12 reservations.
            Some(true) // If so, return Some(true), indicating a successful reservation.
        } else {
            // Otherwise, the restaurant is full.
            Some(false) // Return Some(false), indicating that the reservation failed.
        }
    }

    fn cook_meal(&self, meal_name: String) -> Result<Food, String> {
        // Method to cook a meal. Returns a Result<Food, String>.
        if self.has_mice_infestation {
            // Check if the restaurant has a mice infestation.
            Err("Sorry we have a mice problem".to_string()) // If so, return an Err with an error message.
        } else {
            // Otherwise, the restaurant can cook the meal.
            Ok(Food { name: meal_name }) // Return Ok(Food) with the cooked meal.
        }
    }
}

fn main() {
    // Main function where the program execution begins.
    // Restaurant with 10 reservations and no mice
    let restaurant_case1 = Restaurant {
        // Create a new Restaurant instance.
        reservations: 10,            // Set initial reservations to 10.
        has_mice_infestation: false, // Set mice infestation to false.
    };

    // Attempt to book a reservation
    match restaurant_case1.book_reservation() {
        // Call the book_reservation method and match the result.
        Some(true) => println!("Reservation successful!"), // If Some(true), print success message.
        Some(false) => println!("Restaurant is full."),    // If Some(false), print full message.
        None => println!("Cannot book due to mice infestation."), // If None, print mice infestation message.
    }

    // Attempt to cook a meal
    match restaurant_case1.cook_meal("Pizza".to_string()) {
        // Call the cook_meal method and match the result.
        Ok(food) => println!("Meal cooked: {:?}", food), // If Ok(food), print the cooked meal.
        Err(error) => println!("Error cooking meal: {}", error), // If Err(error), print the error message.
    }

    // Restaurant with 13 reservations and no mice
    let restaurant_case2 = Restaurant {
        // Create a new Restaurant instance.
        reservations: 13,            // Set initial reservations to 13.
        has_mice_infestation: false, // Set mice infestation to false.
    };

    // Attempt to book a reservation
    match restaurant_case2.book_reservation() {
        // Call the book_reservation method and match the result.
        Some(true) => println!("Reservation successful!"), // If Some(true), print success message.
        Some(false) => println!("Restaurant is full."),    // If Some(false), print full message.
        None => println!("Cannot book due to mice infestation."), // If None, print mice infestation message.
    }

    // Restaurant with 10 reservations and mice infestation
    let restaurant_case3 = Restaurant {
        // Create a new Restaurant instance.
        reservations: 10,           // Set initial reservations to 10.
        has_mice_infestation: true, // Set mice infestation to true.
    };

    // Attempt to book a reservation
    match restaurant_case3.book_reservation() {
        // Call the book_reservation method and match the result.
        Some(true) => println!("Reservation successful!"), // If Some(true), print success message.
        Some(false) => println!("Restaurant is full."),    // If Some(false), print full message.
        None => println!("Cannot book due to mice infestation."), // If None, print mice infestation message.
    }

    // Attempt to cook a meal
    match restaurant_case3.cook_meal("Burger".to_string()) {
        // Call the cook_meal method and match the result.
        Ok(food) => println!("Meal cooked: {:?}", food), // If Ok(food), print the cooked meal.
        Err(error) => println!("Error cooking meal: {}", error), // If Err(error), print the error message.
    }
}
