# About

This is my imitation of an AI
It doesn't work that well (I think)  
For the moment it isn't even a library, just me doing stuff around. Maybe I'll make it a library someday

# Install

Just use `cargo add dumb_ai` to add it to your project

# Documentation

Train your ai with the `train_ai()` function. I recommend 0.01 (or 0.1 at most) for the `precision` field. 
To test your ai,  use the `test_ai()` function with two more vectors to get the accuracy of the ai.
If you want to have only one element per vector (inside a vector), you can use the `to_vector_of_vector()` function.
In the two cases, the two vectors need to have the same length, and the vectors inside of them also needs to always have the same length.

To predict a value, use `AI::predict()` on your AI struct to get a value. 
You can save an AI struct to the disk and read from the disk with the associated functions.
`vector_tools` is also avaiable to use alongside the AI part, and the name of the functions are for most self-explanatory.
For those who are not self-explanatory, here's some explications: 
- `to_correct_amount()` is a function that takes a vector of vector of f64s, with only one element per small vector, to a vector of vector of f64s, but with multiple times the same item. That allows for difference in length of small vectors between the input and the output.
