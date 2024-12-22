# Restaurant Reservation and Meal Ordering System

This Rust program simulates a simple restaurant reservation and meal ordering system. It demonstrates the use of `Option` and `Result` types for handling potential errors and invalid states.

## Core Concepts

The code revolves around two main structs: `Food` and `Restaurant`.

* **`Food`**: Represents a meal with a `name` attribute.
* **`Restaurant`**: Represents a restaurant with `reservations` (number of current reservations) and `has_mice_infestation` (a boolean indicating a hygiene issue).

The `Restaurant` struct has two key methods:

* **`book_reservation()`**: Attempts to book a reservation. Returns an `Option<bool>`.
    * `Some(true)`: Reservation successful.
    * `Some(false)`: Reservation failed (restaurant full).
    * `None`: Reservation cannot be processed due to a mice infestation.
* **`cook_meal()`**: Attempts to cook a meal. Returns a `Result<Food, String>`.
    * `Ok(Food)`: Meal successfully cooked.
    * `Err(String)`: Meal could not be cooked due to a mice infestation.

## Usage Examples

The `main` function demonstrates how to interact with the `Restaurant` struct:

```rust
fn main() {
    // Restaurant with 10 reservations and no mice
    let restaurant_case1 = Restaurant {
        reservations: 10,
        has_mice_infestation: false,
    };

    // Attempt to book a reservation
    match restaurant_case1.book_reservation() {
        Some(true) => println!("Reservation successful!"),
        Some(false) => println!("Restaurant is full."),
        None => println!("Cannot book due to mice infestation."),
    }

    // Attempt to cook a meal
    match restaurant_case1.cook_meal("Pizza".to_string()) {
        Ok(food) => println!("Meal cooked: {:?}", food),
        Err(error) => println!("Error cooking meal: {}", error),
    }


    // Restaurant with 13 reservations and no mice
    let restaurant_case2 = Restaurant {
        reservations: 13,
        has_mice_infestation: false,
    };

    // Attempt to book a reservation
    match restaurant_case2.book_reservation() {
        Some(true) => println!("Reservation successful!"),
        Some(false) => println!("Restaurant is full."),
        None => println!("Cannot book due to mice infestation."),
    }

    // Restaurant with 10 reservations and mice infestation
    let restaurant_case3 = Restaurant {
        reservations: 10,
        has_mice_infestation: true,
    };

    // Attempt to book a reservation
    match restaurant_case3.book_reservation() {
        Some(true) => println!("Reservation successful!"),
        Some(false) => println!("Restaurant is full."),
        None => println!("Cannot book due to mice infestation."),
    }

    // Attempt to cook a meal
    match restaurant_case3.cook_meal("Burger".to_string()) {
        Ok(food) => println!("Meal cooked: {:?}", food),
        Err(error) => println!("Error cooking meal: {}", error),
    }
}
