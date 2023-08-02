# Rust Custom Filter Program

This Rust program demonstrates how to filter elements in a collection based on a custom filter condition using a user-defined `FilterCondition` struct.

## How It Works

The program consists of three main components:

1. `FilterCondition` Struct: The `FilterCondition` struct is defined with a generic type `T` to enable filtering of elements of any type that implements the `PartialEq` trait. It has a single field `value` of type `T`, which represents the desired value for filtering.

2. `is_match` Method: The `FilterCondition` struct implements the `is_match` method that takes a reference to an item of the same type as the filter condition. The method compares the item with the `value` field of the `FilterCondition` and returns a boolean indicating whether the item matches the condition.

3. `custom_filter` Function: The `custom_filter` function is defined to filter elements from a collection (a vector in this case) based on the provided `FilterCondition`. It iterates over each element in the collection and checks if it matches the condition using the `is_match` method. If the element matches, it adds it to a new filtered collection.

## How to Use

1. Install Rust: Make sure you have Rust installed on your system. If not, you can download and install Rust from the official website: https://www.rust-lang.org/tools/install

2. Clone the Repository: Clone this repository to your local machine using the following command:


3. Build and Run the Program: Navigate to the project directory and build and run the program using the following command:


4. Output: The program will create a `filtered_vector` containing only the elements that match the filter condition (in this example, elements equal to `3`). It will print the filtered vector to the console.

## Customization

You can modify the `my_vector` and `my_struct` values in the `main` function to test the program with different collections and filter conditions. Ensure that the elements in the collection and the `value` field of the `FilterCondition` struct have the same type.

```rust
fn main() {
    // Create a vector with your desired elements
    let my_vector = vec![1, 2, 3, 4];

    // Create a FilterCondition object with your desired value for filtering
    let my_struct = FilterCondition {
        value: 3,
    };

    // Rest of the code remains the same...
}
