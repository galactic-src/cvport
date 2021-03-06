# Imperial pandemic simulator port

## Purpose

The simulation was created by researchers at Imperial and is one source used by the UK government to guide policy. It was open-sourced at https://github.com/mrc-ide/covid-sim relatively recently.
It is relatively untested, has code translated from Pascal to C and people are whingeing hard that any political decisions
have been made off the back of it.

It's a huge project, and this is definitely not going to fully reimplement it, but I thought I'd do a bit of porting to see if it turned up any odd behaviour, so I can raise it as an issue.
This is also an opportunity for bit of Rust (always welcome!).

## A vague plan

1. Read the project documentation ✓
2. Write a high-level overview of function calls within CovidSim.cpp ✓
3. Build list of outstanding queries related to docs/CovidSim.cpp ✓
4. Summarise contents of code files ✓
5. Jot down initial architecture ideas for new implementation ✓
6. Code to set up a micro-cell based population grid. ✓
7. Code to run a single proximity-transmission update step.
8. Code to run a configurable number of steps.
9. Code to run a number of realisations each of n steps.

Ongoing 
- Build summary of types and their uses (WIP)
- Revisit outstanding queries

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

##### Peripheral features
- Performs a number of 'realisations' of the model
- Uses an InterruptRun boolean for early exit
- setall uses two longs to seed all random number generators (Rand.cpp from Pascal)
- origin destination matrix; is this to show where spread is predicted from/to?
- option for coordinates to be UTM (Universal Transverse Mercator = longitude/latitude)

## Areas to flesh out understanding

- Seems to allow predefined data generation seeds?
- Seems to allow snapshotting and loading model state?
- Why is NumRealisations used so oddly (adding/subtracting 1)?
- RunModel() returns true once according to comments, to reset holiday time. HOw does this work?
- Rand.cpp mltmod mentions running on a 32bit machine in a comment. Is 64bit handled?
- What do 'holidays' cover? public/school?
- What is the third initial seeding mode modelling?
- What does CellLookup reference?
- Why is DAYS_PER_YEAR defined to 364 (Constants.h)?
- NUM_PLACE_TYPES (Country.h) is 4, and the places are: primary/secondary schools, universities and offices. Shouldn't there be other places too? Supermarkets?!
- Is it a problem that the cached values for trig functions are integer-degrees only? Limited resolution?
- Are the kernels square? Shouldn't they be circular?
- Kernel resolution - high-res option 1600 times higher - when does it get used (and why ever use the low-res one)?
- What is Kernel 'shape'? e.g. P.AirportKernelShape. Seems like a scale factor with different meanings for different kernel types.
- CACHE_LINE_SIZE 64 - how important is this? Presumably it varies between machines
- DoInitUpdateProbs - set in SetupModel, triggers UpdateProbs. What does the criterion for doing this again mean? `(lcI - cI) > 0.2` (CovidSim.cpp) 
- Comment in Update.cpp starting "currently commenting this out" relating to household digital contact tracting. Is this fine?
- P.NC (int) is initialised to -1 in CovidSim.cpp but SetupModel runs P.ncw = P.nch = (int)sqrt((double)P.NC); P.NC = P.ncw * P.nch; if !P.DoHeteroDensity.
- what is the significance of size of P.InvLifeExpecDist[MAX_ADUNITS][1001]
- there is something like a TODO in Model.h which states DONE by only some points - see `above quantities need to be amended in following parts of code:`.
- what are the queues in the State (struct PopVar) used for?

## Cleanup

- InitKernel (Kernels.cpp): DoPlaces parameter is unreferenced, and norm parameter is always 1.0 (no effect)
- SetupModel.cpp "Binary densi\zty file should contain %i cells." remove \z
- SetupModel.cpp reads into P.BinFileLen to inspect a magic number in first 4 bytes of density file. Use a different variable.
- InitProbs() declaration in SetupModel.h, but implementation in CovidSim.cpp
- input-params.txt has a leftover merge conflict marker
- SetupModel.cpp "Cell %i has adunits < 0 (indexing AdUnits)\n" logging should refer to microcells
- SetupModel.cpp "Unable to allocate cell storage\n" logging should refer to microcells
- remove obstructive mcl variable in SetupPopulation (SetupModel.cpp)
*** resolved to here
- Inconsistent use of brackets to assign default value to P.LongitudeCutLine
- Simpler to always compare <= maximum of the SpatialBoundingBox and not add the fudge factor? 
- Remove GotNR CLI arg - read directly to P.NumRealisations
- unpack complex conditional in CovidSim.cpp `if(((!P.DoAlertTriggerAfterInterv) && (trigAlert >= P.PreControlClusterIdCaseThreshold))`
- error on more than MAXINTFILE /I: options supplied (overflows array)
- assert no spaces when reading SaveSnapshot parameter?
- log warning (or panic?) if max num threads CLI arg supplied but _OPENMP not set
- use local variable for number of people in cell?
- SetupModel typo infectionz
- lot of renaming around nested loops at l = ((j / P.get_number_of_micro_cells_high()) / P.NMCL) * P.nch + ((j % P.get_number_of_micro_cells_high()) / P.NMCL)
- RunModel rename continueEvents
- Pull GetInputParameter function declarations out into a CovidSim header file