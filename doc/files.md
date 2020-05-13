#Repo File Contents

Obviously the project is under active development, so this is very much a snapshot of state, more for my benefit/orientation than anything else.


|File|Lines|Content|
|----|---|---------|
| /src/ |||
| binio.cpp | 164 | Utils to transfer large amounts of data between FILE and buffer. |
| binio.h | 5 | |
| Bitmap.cpp | 304 | Code to draw a bitmap and save as a .png on Windows or Linux. Colours according to infected, treated, recovered, none, country border. |
| Bitmap.h | 59 | BITMAP_HEADER struct, imports switches for windows and for presence of IMAGE_MAGICK, directory switch for UNIX/Windows. |
| CalcinfSusc.cpp | 70 | 8 short infectiousness/susceptibility functions for households, places, spatial and person. These objects are effectively treated as entities that can hold the disease in their own right. |
| CalcinfSusc.h | 15 | |
| Constants.h | 40 | Broad set of numerical constants, including PI and Earth's radius; number of age groups; days per year (364?!); maximum contacts, airports; max number of intervention change times and others.|
| Country.h | 19 | A few defines and consts presumably related to countries. |
| CovidSim.cpp | 5404 | Core simulation code, including: entrypoint, argument/data file parsing, running a number of realisations, storing results.
| CovidSim.h | 34 | Defines place type constants for the 4 place types. |
| Dist.cpp | 193 | Distance Functions - convert units to UTM, and calculate distance between people, cells, microcells. Uses sinx/cosx/asin2sqx caches declared in Dist.h and populated in CovidSim.cpp. |
| Dist.h | 15 | |
| Error.cpp | 14 | A panic function, ErrorCritical(). |
| Error.h | 18 | compiler directives for Microsoft/GCC (noreturn) for ErrorCritical. |
| InfStat.h | 33 | enums for Infection status (InfStat) and Infection severity (Severity) and an unused NUM_SYMPTO_SEVERITY_CLASSES which is out of date. |
| Kernels.cpp | 136 | various linear functions, and some code to pre-calculate distances between sets (P.NCP - possibly cells containing people). |
| Kernels.h | 18 | |
| MachineDefines.h | 9 | MAX_NUM_THREADS (96) and CACHE_LINE_SIZE (64). |
| Model.h | 363 | Many types: person, household, infection, contactevent, popvar, results, events, indexlist, airport, microcell, cell, place, intervention, adminunit. References to arrays: Hosts (person[]), Households, State/StateT (popvar[]), Cells, CellLookup (cell**), Mcells (microcell[]), McellLookup (microcell**), Places, AdUnits (adminunit[]), Airports, InfEventLog (event[]), and various tally and results arrays. |
| ModelMacros.h | 27 | Utility defines for conditionals based on Hosts[] and Places[] arrays, e.g. HOST_TREATED() and macros for HOST_AGE_YEAR and HOST_AGE_GROUP. |
| Param.h | 283 | A single massive type: param (instance, P, declared in CovidSim.cpp). Contains arrays, booleans, configuration constants. |
| Rand.cpp | 2403 | Various random number sampling functions, transcribed from Pascal to C++, some with academic paper references, or Pascal source. |
| Rand.h | 41 | Defines a few magic numbers used by Rand.cpp. |
| SetupModel.cpp | 2547 | Initial and calculated value assignment for many data structures, often conditional on whether 'doing' a particular feature. |
| SetupModel.h | 27 | Struct bin_file, used in SetupModel.cpp to store various data and write to the configured OutDensFile. |
| Sweep.cpp | 1766 | Model updating functions, modelling people travelling, new infections, changes in progression of the disease for individuals, digital contact tracing efforts and impact of treatments. |
| Sweep.h | 14 | |
| Update.cpp | 1337 | Methods to handle many data transitions and associated housekeeping, e.g. a person making a full recovery|
| Update.h | 29 | |
| CMakeLists.txt | 44 | Declares single executable and includes the current source dir. Optionally pulls in OpenMP. Pulls in OS-specific image handling libraries. |
||||
|/ci/|||
|install_dependencies.sh|3|Use sh to install Python and build tools for fedora/debian/arch/ubuntu/centos/suse/alpine. |
|install_macos_dependences.sh|50|Use sh to install Python and build tools on Mac. |
||||
|/data/admin_units/|||
|/data/param_files/|||
|/data/populations/|||
|/data/run_sample.py|||
|/tests/|||
|/tests/us-input|||
|Dockerfile|39||
||||
|/Rscripts/|||
|CompareRuns.R|228||
|PlotRuns.R|365||
|ReadCompareParamFiles.R|199||
|Rscripts.Rproj|13||
