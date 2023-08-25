// Step 2: Define a struct called FilterCondition
struct FilterCondition<T> {
    condition: T,
}

// Step 3: Implement the is_match method on the FilterCondition struct
impl<T> FilterCondition<T>
where
    T: PartialEq,
{
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

// Step 4: Define the custom_filter function
fn custom_filter<T>(collection: &[T], filter_condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone + PartialEq,
{
    collection
        .iter()
        .filter(|item| filter_condition.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    // Step 5: Create a collection and a FilterCondition object
    let numbers = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { condition: 3 };

    // Step 6: Call the custom_filter function and store the result
    let filtered_numbers = custom_filter(&numbers, &filter_condition);

    // Step 7: Print the filtered result to the console
    println!("Filtered Numbers: {:?}", filtered_numbers);
}