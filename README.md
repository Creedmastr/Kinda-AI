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
