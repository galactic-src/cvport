#Architecture Ideas

##Commentary

Currently, the model is executed in a series of top-level functions.
It is primarily C code, not using OO or control flow features of C++.
The data is essentially all kept in memory, so access should be very fast.
A lot of state is global.
Functions are very long, modify a lot of varied data, have heavily-nested conditionals and loops.
A few files contain most of the logic, and are very large, with very broad responsibilities.
Data structures are very large, so references to a tremendous amount of data are passed between functions.
Rand.cpp contains a lot of transcribed code (at least some from Pascal) which involves several gotos.
Several variables are not descriptive, having just 1-3 letters.

The large functions and data structures mean the code is very resistant to unit testing in its current form.
However, given that it is based around sampling random values, the fact that the random values are seeded explicitly allows for deterministic runs.
This has allowed at least a basic regression test to be formulated, based on a hash of outputs after known seeds were used.

##Issues

The lack of testability is a big concern. We can't get around this without
- avoiding global state
- making much simpler methods with smaller responsibilities
- making state objects smaller so that mocks and/or engineered versions can be produced by test factories

I also wonder whether it would make sense to allow this to work on machines with less than the recommended RAM, using a database to hold state. Data from previous runs could then be made easily accessible for comparison.
The code has several areas where optimisations have been made. Depending how long ago they were made, they may or may not be appropriate for modern hardware.
In reimplementing, simplicity should take priority and hot code can be identified later.

##Possible Approach

My initial instinct is to split the model state down into smaller optional pieces, which may impact each other during a sweep via a publish/subscribe model.
During setup, the required objects would be created and subscribe to the events emitted by the other objects that may trigger updates in them.
This may not be appropriate, depending on the conditionals that take state from many such objects into account.
However, if it did mostly fit the code, it would make for much more testable code.

So a brief initial order of play:

1. Create a model consisting only of cells
2. Add microcells
3. Add people
4. Add something like AssignPeopleToPlaces to distribute people across cells
5. Add ability for people to be infected (boolean) 
6. Add some modelling for spread of disease between people

If I get that far, hopefully by then I'll have some idea of how I should have approached it in the first place and a better way to structure it!
