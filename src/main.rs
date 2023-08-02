// Define the main function
fn main() {
    // Create a vector containing elements 1, 2, 3, and 4
    let my_vector = vec![1, 2, 3, 4];

    // Create a FilterCondition object with the desired value 3
    let my_struct = FilterCondition {
        value: 3,
    };

    // Call the custom_filter function with the vector and the FilterCondition object
    let filtered_vector = custom_filter(my_vector, &my_struct);

    // Print the filtered vector to the console
    println!("{:?}", filtered_vector);
}

// Define the FilterCondition struct with a generic type T for filtering
struct FilterCondition<T> {
    value: T,
}

// Implement methods for the FilterCondition struct
impl<T: PartialEq> FilterCondition<T> {
    // Implement the is_match method, which checks if an item matches the filter condition
    fn is_match(&self, item: &T) -> bool {
        *item == self.value
    }
}

// Define the custom_filter function that filters elements based on the condition
fn custom_filter<T>(vec: Vec<T>, obj: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
{
    // Create an empty vector to store filtered elements
    let mut filtered_collection = Vec::new();

    // Iterate through each item in the input vector
    for item in vec {
        // Check if the item matches the filter condition using the FilterCondition's is_match method
        if obj.is_match(&item) {
            // If the item matches, add it to the filtered_collection
            filtered_collection.push(item);
        }
    }

    // Return the filtered_collection containing only the elements that matched the filter condition
    filtered_collection
}
