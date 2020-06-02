# Architecture Ideas

## Commentary

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

## Data structure

The most granular information, in terms of located data, is the microcell. Each microcell belongs to a particular cell, country, administrative unit (~country region).
Administrative units are given a code, consisting of 

## Issues

The lack of testability is a big concern. We can't get around this without
- avoiding global state
- making much simpler methods with smaller responsibilities
- making state objects smaller so that mocks and/or configured versions can be sensibly created in factory methods for tests

I also wonder whether it would make sense to allow this to work on machines with less than the recommended RAM, using a database to hold state. Data from previous runs could then be made easily accessible for comparison.
The code has several areas where optimisations have been made. Depending how long ago they were made, they may or may not be appropriate for modern hardware.
In reimplementing, simplicity should take priority and hot code can be identified later.

I am uncertain how good the random number generation is - not my area. Rather than reimplementing the rand.cpp code, I would certainly look to something like the rand crate.

## Possible Approach

My initial instinct is to split the model state down into smaller optional pieces, which may impact each other during a sweep via a publish/subscribe model.
During setup, the required objects would be created and subscribe to the events emitted by the other objects that may trigger updates in them.
This may not be appropriate, depending on the conditionals that take state from many such objects into account.
However, if it did mostly fit the code, it would make for much more testable code.

So a brief initial order of play:

1. Create a model structure consisting only of microcells ✓
2. Add people ✓
3. Add something like AssignPeopleToPlaces to distribute people across cells ✓
4. Add ability for people to be infected (boolean or enum?) 
5. Add some modelling for spread of disease between people (see Sweep.cpp)

If I get that far, hopefully by then I'll have some idea of how I should have approached it in the first place and a better way to structure it!

## Params/Features list

A place for me to dump additional features that could be implemented from the original
All args are expected to start with a '/', or be of the format '?:' or '??:'.
Panics if S and L options supplied (conflicting LoadSaveNetwork types) or if no P or O option present.

| Arg(s) | Type | Meaning |
|---|---|---|
| last 4 args | integer | 2x setup random number seeds and 2x run random number seeds |
| /P: | string | 'ParamFile' (required): Param file (path?) |
| /O: | string | 'OutFileBase' (required): First part of name used for output files |
| /D: | string | 'DensityFile' (optional): Input data for population density. Used to set Bounds, if no bounds supplied. Also sets DoHeteroDensity=1 and DoPeriodicBoundaries=0 |
| /A: | string | 'AdunitFile' (optional): Sets AdminFile_dat for admin unit data (otherwise assume that ParamFile_dat contains admin info, if needed)
| /L: | string | 'NetworkFile' (optional): Sets LoadSaveNetwork=1 and loads network info from the supplied network file |
| /S: | string | 'NetworkFile' (optional): Sets LoadSaveNetwork=2 and saves the network file to the supplied path |
| /R: | double | 'P.R0scale' (optional - default 1.0): scales various transmission rate variables |
| /NR: | int | 'P.NumRealisations' - used to work out how many times to run the simulation |
| /KP: | double | 'P.KernelPowerScale' - used to scale P.MoveKernelShape quickly from the command line |
| /KO: | double | 'P.KernelOffsetScale' - used to scale P.MoveKernelScale, which populates P.PlaceTypeKernelScale[] |
| /CLP[1,2,3,4,5,6]: | double | 'P.clP[1-6]' - generic command line specified parameter, matches #1-6 in param file |
| /AP: | string | 'AirTravelFile': airport data file, used by ReadAirTravel (requires P.DoAirports) |
| /s: | string | 'SchoolFile': school data file, consumed in SetupPopulation; sets DoSchoolFile=1 (requires DoPlaces) |
| /T: | int | 'P.PreControlClusterIdCaseThreshold' |
| /C: | int | 'P.PlaceCloseIndepThresh' |
| /d: | string | 'RegDemogFile' regional demographics file; 'P.DoAdunitDemog' set to 1 (requires P.DoAdunits) |
| /c: | int | 'P.MaxNumThreads' |
| /M: | string | 'OutDensFile': causes binary density file info to be written out; sets P.OutputDensFile=1 |
| /I: | string | Adds an entry to InterventionFile[] (max 10) used to configure interventions applied during simulation |
| /LS: | string | 'SnapshotLoadFile' file path to previously written simulation snapshot file to resume; sets P.DoLoadSnapshot=1 |
| /PP: | string | 'PreParamFile' file path to a separate pre-param file (otherwise assumes param file contains this info) |
| /SS: | string | sets P.DoSaveSnapshot and splits on first comma char into 'P.SnapshotSaveTime' (double) and 'SnapshotSaveFile' (string). Saves a snapshot at the specified time into the simulation |
| /BM: | "bmp" or "png" | sets 'P.BitmapFormat' (png only supported for windows or if IMAGE_MAGICK library available |

## Required parameters

List of the places where GetInputParameter() is called unconditionally, i.e. these data are required for any simulation run.
There are several other parameters that are conditionally required (e.g. if certain optional parameters are supplied, others may become required).

| Description | Variable | Type | Feature |
|---|---|---|---|
| Update timestep | P.TimeStep | double |  |
| Sampling timestep | P.SampleStep | double |  |
| Sampling time | P.SampleTime | double |  |
| Population size | P.PopSize | int |  |
| Number of realisations | P.NumRealisations | int | (can supply via CLI) |
| Number of micro-cells per spatial cell width | P.NMCL | int |  |
| Initial number of infecteds | P.NumInitialInfections | int |  |
| Reproduction number | P.R0 | double |  |
| Infectious period | P.InfectiousPeriod | double |  |


### Features summary

#### Model repetitions, timings and outputs

The model is based on a stepwise routine, each step being one P.TimeStep (measured in days) long.
A sample is taken after every P.SampleStep of time is modelled (an integer number of steps - P.TimeStep is scaled down to accommodate).
The total run time to simulate is determined by P.RunTime.

Each step consists of a single pass of RunModel, and results in various "sweeps" being carried out, to model the effects of e.g. people recovering or dying, and the transmission of the disease.
Various interventions modulate the impact these phases.

The full simulation is carried out a number of times - termed Realisations. 

The output is a series of images (bmp or png) capturing the modelled spread of disease.


#### Cells and Microcells

Data in the model is associated with a square grid, referred to as Cells and Microcells. Each cell is made up of a number of smaller Microcells.
The overall area of interest need not be square.

#### Variable population density

If a density file is supplied with the /D:path/to/file CLI option, it is either an ASCII list of records or an equivalent binary file.
If no density file is supplied, every microcell is given a population weighting (or total population if no total population is supplied) of 1.0.

Total simulation population can optionally be fixed using the [Fix population size at specified value] param file entry.

The ASCII records have format:
"%lf\t%lf\t%d\t%d\t%d"
x_coord y_coord population  country_id  admin_unit_id
e.g.
137.55833	75.825	1	45	456000

The (x, y) coordinates are resolved to a microcell in the grid. When multiple records resolve to the same microcell, the average density is taken.
The population is treated as a relative density (if a total population across the whole simulation is supplied) or as a total population (if not).

The binary density file is output if the /M:path/to/file CLI option is supplied.
It has a magic number 0xf0f0f0f0 as its first 4 bytes.

#### Administrative Units

TODO

#### Severity

TODO

#### Infection Seeding

Number of initial infection seed locations is configured by the param file option "Number of seed locations". The number of initial infected individuals is set via param option "Initial number of infecteds".
The maximum number of initial infections is MAX_NUM_SEED_LOCATIONS (10000)
P.NumSeedLocations

#### Holidays

TODO

#### Seasonality

TODO

#### Households