#COVID port

## Purpose

The simulation used by the government to guid policy, and made public at https://github.com/mrc-ide/covid-sim was made years ago. 
It is relatively untested, has code transpiled from Fortran to C and people are whingeing hard that any political decisions
have been made off the back of it.

It's a huge project, and this is definitely not going to reimplement it, but I thought I'd do a bit of porting to see if it turned up any odd behaviour, so I can raise it as an issue.
Also an opportunity for bit of Rust (always welcome).

## A vague plan

1. Read the project documentation
2. Write a high-level overview of function calls within CovidSim.cpp
3. Build list of outstanding queries related to docs/CovidSim.cpp
4. Build summary of types and their uses
5. Come up with vocab for stages in pipeline for later reference as to where labels are used
6. List some leaf functions to port
7. Summarise contents of all files
8. Begin porting some functions

## High-level overview

1. Entrypoint: main
2. Parses command line parameters (and referenced files) into a global parameters object, P
3. Calculates further reference values in ReadParams() and reads parameters from param files
   1. Configure data to output
   2. Configure kernel resolution
   3. Include transmission within houses modelling (or not)
   4. Include transmission via shared administrative units (or not)
   5. Include age-specific factors
   6. Include house-proximity-based transmission
   7. Include Airport/hotel modelling (calls ReadAirTravel)
   8. Include place-based transmission
   9. Model seasonality
   10. Configure initial infections
   11. Model disease progression and infectiousness profile (optionally including age-specific symptoms and severity)
   12. Configure interventions (Treatment, vaccination, social distancing, restricted movement, household quarantine, contact tracing, place closure, admin unit closure, airport closure)
   13. Calls into readInterventions which parses XML files and stores various data
   14. Model impact of public (or school?) holidays
   15. Vary efficacy of interventions over time
   16. Model Key worker households
   17. Model sensible age makeup of households
4. Initialises the model SetupModel. Optionally setup Interventions, AirTravel
   1. Initial values for State, AdUnits, StateT (thread State?), Hosts, Cells, Households, MCells, Places, InfEventLog
   2. Initial state for various variables in params object P (for variables that vary over time)
   3. Calls SeedInfection,
      1. Models initial infection either all in one location, in random locations or some other criteria around adUnits and population density 
      2. A variable m is used as some kind of sanity check for all methods
      3. which calls out to DoInfect (Update.cpp) which transitions people from susceptible to infected
      4. In turn may call out to doIncub, doCase, doRecover, but not at this point since time = 0
5. For each realisation, sets data seeds and calls RunModel a number of times
   1. Option to interrupt at any point. Option to run until a number of infections is reached, option to regenerate random number seeds
   2. Model infected airport arrivals (uses SeedInfection)
   3. Model false Positives
   4. Call InfectSweep, IncubRecoverySweep, DigitalContactTracingSweep, TreatSweep, TravelReturnSweep (Sweep.cpp)
   5. Call SaveSnapshot (optionally) - a load of lines written to a file with format ## <number>
   6. Call UpdateProbs - updates lots of data via CellLookup
6. For each realisation, optionally calls SaveResults, always calls SaveEvents
7. For overall run, optionally outputs origin/destination matrix. Calls RecordSample now if not called for every run.
8. calls SaveSummaryResults

#####Peripheral features
- Performs a number of 'realisations' of the model
- Uses an InterruptRun boolean for early exit
- setall uses two longs to seed all random number generators (Rand.cpp from Pascal)
- origin destination matrix; is this to show where spread is predicted from/to?

## Outstanding questions or areas to flesh out

1. Where are GetInputParameter, GetInputParameter2, GetInputParameter3 defined?
2. Seems to allow predefined data generation seeds?
3. Seems to allow snapshotting and loading model state?
4. Why is NumRealisations used so oddly?
5. RunModel() returns true once according to comments, to reset holiday time. HOw does this work?
6. Rand.cpp mltmod mentions running on a 32bit machine in a comment. Is 64bit handled?
7. What do 'holidays' cover? public/school?
8. What is the third initial seeding mode modelling?
9. What does CellLookup reference?