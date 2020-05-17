#State Map

Initially this file serves as a breakdown of existing state objects (and probably some local variables) and what they represent.
It is also an opportunity to sanity check that variables are in fact used.
Later it may well become a mapping to equivalent variables in the reimplemented code.

Maybe I'll get bored of this before I get too far. There are a LOT of state fields.

param

| Field | Represents |
|----------|------------|
| int PopSize | population size (number of individuals) |
| int NH | number of households |
| int NumRealisations | number of realisations |
| int NumNonExtinctRealisations | number of non-extinct realisations |
| int NRactual | nasty temp var used by SaveSummaryResults set to NRactE or NRactNE |
| int NRactE | number of extinct Runs |
| int NRactNE | number of non-extinct Runs |
| int UpdatesPerSample | Number of time steps between samples |
| int NumSamples | Total number of samples that will be made |
| int NKR | Size of kernel lookup table |
| int NK_HR | Factor to expand hi-res kernel lookup table by |
| int MoveKernelType | |
| int AirportKernelType | |
| int KernelType | Used to init particular Kernel type, set from P.MoveKernelType, P.AirportKernelType and P.PlaceTypeKernelType[#place types] |
| unsigned int BinFileLen | |
| int DoBin, DoSaveSnapshot, DoLoadSnapshot | |
| double SnapshotSaveTime | |
| double SnapshotLoadTime | |
| double clP1 | |
| double clP2 | |
| double clP3 | |
| double clP4 | |
| double clP5 | |
| double clP6 | |
| int NC | Number of cells |
| int NMC | Number of microcells |
| int NMCL | Number of microcells wide/high a cell is; i.e. NMC = NC * NMCL * NMCL |
| int NCP | Number of populated cells |
| int NMCP |  |
| int ncw |  |
| int nch |  |
| int nmcw |  |
| int nmch | nch * NMCL |
| int DoUTM_coords |  |
| int nsp |  |
| int DoSeasonality |  |
| int DoCorrectAgeDist |  |
| int DoPartialImmunity |  |
| int DoAdUnits |  |
| int NumAdunits |  |
| int DoAdunitBoundaries |  |
| int AdunitLevel1Divisor |  |
| int AdunitLevel1Mask |  |
| int AdunitBitmapDivisor |  |
| int CountryDivisor |  |
| int DoAdunitOutput |  |
| int DoAdunitBoundaryOutput |  |
| int DoAdunitDemog |  |
| int DoCorrectAdunitPop |  |
| int DoSpecifyPop |  |
| int[] AdunitLevel1Lookup | size=ADUNIT_LOOKUP_SIZE |
| int DoOutputPlaceDistForOneAdunit |  |
| int OutputPlaceDistAdunit |  |
| int OutputDensFile |  |
| int DoOneGen |  |
| int OutputEveryRealisation |  |
| int BitmapMovieFrame |  |
| int MaxCorrSample |  |
| int DoLatent |  |
| int InfQueuePeakLength |  |
| int NumThreads |  |
| int MaxNumThreads |  |
| int bwidth | pixel width of map area in bitmap object |
| int bheight | pixel height of map area in bitmap object |
| int bheight2 | Height in pixels of the entire bitmap output, including both the spectrum at the top and the map area | 
| int bminx |  |
| int bminy |  |
| int OutputBitmap | Whether to output a bitmap |
| BitmapFormats BitmapFormat | Format of bitmap (platform dependent and command-line /BM: specified |
| int DoSI | |
| int DoHeteroDensity | Whether using a density file or assuming homogenous population density |
| int DoPeriodicBoundaries |  |
| int DoImmuneBitmap |  |
| int OutputBitmapDetected |  |
| int DoHouseholds |  |
| int DoPlaces |  |
| int PlaceTypeNum |  |
| int Nplace[] | size=NUM_PLACE_TYPES |
| int SmallEpidemicCases |  |
| int DoPlaceGroupTreat |  |
| int NumInitialInfections[] | size=MAX_NUM_SEED_LOCATIONS |
| int DoRandomInitialInfectionLoc |  |
| int DoAllInitialInfectioninSameLoc |  |
| int MinPopDensForInitialInfection |  |
| int NumSeedLocations |  |
| int MaxPopDensForInitialInfection |  |
| int[] InitialInfectionsAdminUnitId | size=MAX_NUM_SEED_LOCATIONS |
| int[] InitialInfectionsAdminUnit | size=MAX_NUM_SEED_LOCATIONS |
| int DoAge |  |
| int DoSymptoms |  |
| int LoadSaveNetwork |  |
| int IncThreshPop |  |
| int GlobalIncThreshPop |  |
| int OutputOnlyNonExtinct |  |
| int DoInfectiousnessProfile |  |
| int DoInfectionTree |  |
| int DoWholeHouseholdImmunity |  |
| int DoSpatial |  |
| int DoDeath |  |
| int DoAirports |  |
| int Nairports |  |
| int Air_popscale |  |
| int DoSchoolFile |  |
| int DoRealSymptWithdrawal |  |
| int CaseAbsentChildAgeCutoff |  |
| int DoEarlyCaseDiagnosis |  |
| int DoInterventionFile |  |
| int PlaceTypeNoAirNum |  If DoAirports then this is the number of non-airport place types (< PlaceTypeNum), else == PlaceTypeNum (~ no airport places) |
| int HotelPlaceType | If DoAirports then this is place type for hotel (>= PlaceTypeNoAirNum, < PlaceTypeNum), else == PlaceTypeNum (~ unused) |
| long setupSeed1 | RNG seed from the command line, used to initialise the RNG for setup |
| long setupSeed2 | RNG seed from the command line, used to initialise the RNG for setup |
| long runSeed1 | RNG seed from the command line, used to initialise the RNG for running the model |
| long runSeed2 | RNG seed from the command line, used to initialise the RNG for running the model |
| long nextSetupSeed1 | The next RNG setupSeed1 to use when we need to reinitialise the RNG for setup |
| long nextSetupSeed2 | The next RNG setupSeed2 to use when we need to reinitialise the RNG for setup |
| long nextRunSeed1 | The next RNG runSeed1 to use when we need to reinitialise the RNG for the model |
| long nextRunSeed2 | The next RNG runSeed2 to use when we need to reinitialise the RNG for the model |
| int ResetSeeds |  |
| int KeepSameSeeds |  |
| int ResetSeedsPostIntervention |  |
| int ResetSeedsFlag |  |
| int TimeToResetSeeds |  |
| double LongitudeCutLine | Longitude to image earth is cut at to produce a flat map.  Default -360 degrees (effectively -180).  Use to ensure countries have a contiguous boundary |
| double[] SpatialBoundingBox | size=4 |
| double[][] LocationInitialInfection | dims=MAX_NUM_SEED_LOCATIONS * 2
| double[] InitialInfectionsAdminUnitWeight | size=MAX_NUM_SEED_LOCATIONS |
| double TimeStepsPerDay |  |
| double FalsePositiveRate |  |
| double FalsePositivePerCapitaIncidence |  |
| double[] FalsePositiveAgeRate | size=NUM_AGE_GROUPS |
| double[] latent_icdf | size=(CDF_RES + 1) |
| double[] infectious_icdf | size=(CDF_RES + 1) |
| double[] infectious_prof | size=(INFPROF_RES + 1) |
| double[] infectiousness | size=MAX_INFECTIOUS_STEPS |
| double[] MildToRecovery_icdf | size=(CDF_RES + 1) |
| double[] ILIToRecovery_icdf | size=(CDF_RES + 1) |
| double[] SARIToRecovery_icdf | size=(CDF_RES + 1) |
| double[] CriticalToCritRecov_icdf | size=(CDF_RES + 1) |
| double[] CritRecovToRecov_icdf | size=(CDF_RES + 1) |
| double[] ILIToSARI_icdf | size=(CDF_RES + 1) |
| double[] SARIToCritical_icdf | size=(CDF_RES + 1) |
| double[] ILIToDeath_icdf | size=(CDF_RES + 1) |
| double[] SARIToDeath_icdf | size=(CDF_RES + 1) |
| double[] CriticalToDeath_icdf | size=(CDF_RES + 1);
| double Mean_MildToRecovery |  |
| double Mean_ILIToRecovery |  |
| double Mean_SARIToRecovery |  |
| double Mean_CriticalToCritRecov |  |
| double Mean_CritRecovToRecov |  |
| double Mean_TimeToTest |  |
| double Mean_TimeToTestOffset |  |
| double Mean_TimeToTestCriticalOffset |  |
| double Mean_TimeToTestCritRecovOffset |  |
| double Mean_ILIToSARI |  |
| double Mean_SARIToCritical |  |
| double Mean_CriticalToDeath |  |
| double Mean_SARIToDeath |  |
| double Mean_ILIToDeath |  |
| double[] Prop_Mild_ByAge | size=NUM_AGE_GROUPS |
| double[] Prop_ILI_ByAge | size=NUM_AGE_GROUPS |
| double[] Prop_SARI_ByAge | size=NUM_AGE_GROUPS |
| double[] Prop_Critical_ByAge | size=NUM_AGE_GROUPS |
| double[] CFR_SARI_ByAge | size=NUM_AGE_GROUPS |
| double[] CFR_Critical_ByAge | size=NUM_AGE_GROUPS |
| double[] CFR_ILI_ByAge | size=NUM_AGE_GROUPS |



Cell
- Array: Cells (count = P.NC)
- Pointer Array: CellLookup

| Field | Represents |
|----------|------------|
| int n | | see setupmodel -> setuppopulation
| int S | susceptible? |
| int L | latent? |
| int I | infected? |
| int R | recovered? |
| int D | died? |
| int cumTC ||
| int S0 | initial number of susceptibles within cell |
| int tot_treat | total number of people treated within cell |
| int tot_vacc | total number of people vaccinated within cell|
| int *members ||
| int *susceptible ||
| int *latent ||
| int *infected ||
| int *InvCDF ||
| float tot_prob ||
| float *cum_trans ||
| float *max_trans ||
| short[] CurInterv | size=MAX_INTERVENTION_TYPES|

Microcell
Array: Mcells (count = P.NMC)
Pointer Array: McellLookup

| Field | Represents |
|----------|------------|
|||