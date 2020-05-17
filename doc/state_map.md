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
| double TimeStep | The length of a time step, in days |
| double SampleTime | The number of days to run for |
| double SampleStep | The length of a sampling step, in days |
| double BitmapAspectScale | Height of bitmap / Width of bitmap |
| int ts_age |  |
| int DoSeverity | Non-zero (true) if severity analysis should be done |
| double scalex | pixels per degree (x) in bitmap output |
| double scaley | pixels per degree (y) in bitmap output |
| double width | Size of spatial domain (x) in degrees |
| double height | Size of spatial domain (y) in degrees |
| double cwidth | Size of spatial domain (x) in cells |
| double cheight | Size of spatial domain (y) in cells |
| double mcwidth | Size of spatial domain (x) in microcells |
| double mcheight | Size of spatial domain (y) in microcells |
| double KernelShape |  |
| double KernelScale |  |
| double KernelP3 |  |
| double KernelP4 |  |
| double KernelDelta |  |
| double MoveKernelShape |  |
| double MoveKernelScale |  |
| double MoveKernelP3 |  |
| double MoveKernelP4 |  |
| double AirportKernelShape |  |
| double AirportKernelScale |  |
| double AirportKernelP3 |  |
| double AirportKernelP4 |  |
| double AirportTrafficScale |  |
| double R0 |  |
| double R0scale |  |
| double LocalBeta |  |
| double LatentPeriod | In days. Mean of icdf (inverse cumulative distribution function) |
| double InfectiousPeriod | In days. Mean of icdf (inverse cumulative distribution function) |
| double R0household |  |
| double R0places |  |
| double R0spatial |  |
| double[] Seasonality | size=DAYS_PER_YEAR |
| double InfectiousnessSD |  |
| double R0DensityScalePower |  |
| double InfectiousnessGamA |  |
| double InfectiousnessGamR |  |
| double[] ProportionSymptomatic | size=NUM_AGE_GROUPS |
| double LatentToSymptDelay |  |
| double SymptInfectiousness |  |
| double SymptSpatialContactRate |  |
| double[] SymptPlaceTypeContactRate | size=NUM_PLACE_TYPES |
| double[] InhibitInterAdunitPlaceAssignment | size=NUM_PLACE_TYPES |
| double[] SymptPlaceTypeWithdrawalProp | size=NUM_PLACE_TYPES |
| double CaseAbsenteeismDuration |  |
| double CaseAbsenteeismDelay |  |
| double PlaceCloseRoundHousehold | Default 1 (close places around a household), 0 (off) |
| int AbsenteeismPlaceClosure | Default 0 (off), 1 (on) track place closures in more detail |
| int MaxAbsentTime | In days.  Max number of days absent, range [0, MAX_ABSENT_TIME].  Default 0 if !P.AbsenteeismPlaceClosure, otherwise MAX_ABSENT_TIME |
| double CaseAbsentChildPropAdultCarers |  |
| double[] RelativeTravelRate | size=NUM_AGE_GROUPS |
| double[] RelativeSpatialContact | size=NUM_AGE_GROUPS |
| double[] AgeSusceptibility | size=NUM_AGE_GROUPS |
| double[] AgeInfectiousness | size=NUM_AGE_GROUPS |
| double[] InitialImmunity | size=NUM_AGE_GROUPS |
| double[][] WAIFW_Matrix | dims=NUM_AGE_GROUPS * NUM_AGE_GROUPS |
| double HotelPropLocal |  |
| double[] JourneyDurationDistrib | size=MAX_TRAVEL_TIME |
| double[] LocalJourneyDurationDistrib | size=MAX_TRAVEL_TIME |
| double MeanJourneyTime |  |
| double MeanLocalJourneyTime |  |
| int[] InvJourneyDurationDistrib | size=1025 | 
| int[] InvLocalJourneyDurationDistrib | size=1025 |
| double HouseholdTrans |  |
| double[][] HouseholdSizeDistrib | dims=MAX_ADUNITS * MAX_HOUSEHOLD_SIZE |
| double HouseholdTransPow |  |
| double[] HouseholdDenomLookup | size=MAX_HOUSEHOLD_SIZE |
| int[] PlaceTypeAgeMin | size=NUM_PLACE_TYPES |
| int[] PlaceTypeAgeMax | size=NUM_PLACE_TYPES |
| int[] PlaceTypeMaxAgeRead | size=NUM_PLACE_TYPES |
| int[] PlaceTypeAgeMin2 | size=NUM_PLACE_TYPES |
| int[] PlaceTypeAgeMax2 | size=NUM_PLACE_TYPES |
| int[] PlaceTypeAgeMin3 | size=NUM_PLACE_TYPES |
| int[] PlaceTypeAgeMax3 | size=NUM_PLACE_TYPES |
| int[] PlaceTypeNearestNeighb | size=NUM_PLACE_TYPES |
| int[] PlaceTypeKernelType | size=NUM_PLACE_TYPES |
| double[] PlaceTypePropAgeGroup | size=NUM_PLACE_TYPES |
| double[] PlaceTypePropAgeGroup2 | size=NUM_PLACE_TYPES |
| double[] PlaceTypePropAgeGroup3 | size=NUM_PLACE_TYPES |
| double[] PlaceTypeKernelShape | size=NUM_PLACE_TYPES |
| double[] PlaceTypeKernelScale | size=NUM_PLACE_TYPES |
| double[] PlaceTypeKernelP3 | size=NUM_PLACE_TYPES |
| double[] PlaceTypeKernelP4 | size=NUM_PLACE_TYPES |
| double[] PlaceTypeTrans | size=NUM_PLACE_TYPES |
| double[] PlaceTypeMeanSize | size=NUM_PLACE_TYPES |
| double[] PlaceTypePropBetweenGroupLinks | size=NUM_PLACE_TYPES |
| double[] PlaceTypeSizeSD | size=NUM_PLACE_TYPES 'for lognormal distribution' |
| double[] PlaceTypeSizePower | size=NUM_PLACE_TYPES |
| double[] PlaceTypeSizeOffset | size=NUM_PLACE_TYPES |
| double[] PlaceTypeSizeMax | size=NUM_PLACE_TYPES |
| double[] PlaceTypeGroupSizeParam1 | size=NUM_PLACE_TYPES |
| double[][] PlaceExclusivityMatrix | size=(NUM_PLACE_TYPES*NUM_PLACE_TYPES) |
| double[] PlaceTypePropBetweenGroupLinks | size=NUM_PLACE_TYPES |
| double[][] PropAgeGroup | dims=MAX_ADUNITS * NUM_AGE_GROUPS |
| double[][] PopByAdunit | dims=MAX_ADUNITS * 2 |
| double[][] InvLifeExpecDist | dims=MAX_ADUNITS * 1001 |
| double PlaceCloseTimeStart |  |
| double PlaceCloseTimeStart2 |  |
| double PlaceCloseDurationBase |  |
| double PlaceCloseDuration |  |
| double PlaceCloseDuration2 |  |
| double PlaceCloseDelayMean |  |
| double PlaceCloseRadius |  |
| double PlaceCloseRadius2 |  |
| double[] PlaceCloseEffect | size=NUM_PLACE_TYPES |
| double[] PlaceClosePropAttending | size=NUM_PLACE_TYPES |
| double PlaceCloseSpatialRelContact |  |
| double PlaceCloseHouseholdRelContact |  |
| double PlaceCloseCasePropThresh |  |
| double PlaceCloseAdunitPropThresh |  |
| double PlaceCloseFracIncTrig |  |
| int DoHolidays |  |
| int NumHolidays |  |
| double[] HolidayEffect | size=NUM_PLACE_TYPES |
| double[] HolidayStartTime | size=DAYS_PER_YEAR |
| double[] HolidayDuration | size=DAYS_PER_YEAR |
| double ColourPeriod |  |
| double[] BoundingBox | size=4 |
| double BitmapScale |  |
| double TreatSuscDrop |  |
| double TreatInfDrop |  |
| double TreatDeathDrop |  |
| double TreatSympDrop |  |
| double TreatDelayMean |  |
| double TreatTimeStart |  |
| double TreatPlaceGeogDuration |  |
| double TreatProphCourseLength |  |
| double TreatCaseCourseLength |  |
| double TreatPropRadial |  |
| double TreatRadius |  |
| double TreatRadius2 |  |
| double TreatCellIncThresh |  |
| double CaseIsolation_CellIncThresh |  |
| double HHQuar_CellIncThresh |  |
| double DigitalContactTracing_CellIncThresh |  |
| double TreatPropCases |  |
| double DigitalContactTracing_CellIncThresh |  |
| double TreatPropCaseHouseholds |  |
| double TreatHouseholdsDuration |  |
| double[] TreatPlaceProbCaseId | size=NUM_PLACE_TYPES |
| double[] TreatPlaceTotalProp | size=NUM_PLACE_TYPES |
| double TreatMaxCoursesBase |  |
| double TreatNewCoursesRate |  |
| double TreatNewCoursesStartTime |  |
| double TreatMaxCourses |  |
| double VaccSuscDrop |  |
| double VaccSuscDrop2 |  |
| double VaccInfDrop |  |
| double VaccMortDrop |  |
| double VaccSympDrop |  |
| double VaccDelayMean |  |
| double VaccTimeStart |  |
| double VaccTimeEfficacySwitch |  |
| double VaccTimeStartGeo |  |
| double VaccTimeToEfficacy |  |
| double VaccProp |  |
| double VaccRadius |  |
| double VaccRadius2 |  |
| double VaccMinRadius |  |
| double VaccMinRadius2 |  |
| double VaccPropCaseHouseholds |  |
| double VaccHouseholdsDuration |  |
| double VaccMaxCoursesBase |  |
| double VaccNewCoursesRate |  |
| double VaccNewCoursesStartTime |  |
| double VaccMaxCourses |  |
| double VaccNewCoursesEndTime |  |
| double VaccEfficacyDecay |  |
| double VaccCellIncThresh |  |
| double VaccCampaignInterval |  |
| double VaccCoverageIncreasePeriod |  |
| int VaccDosePerDay |  |
| double PreAlertControlPropCasesId |  |
| double PostAlertControlPropCasesId |  |
| double ControlPropCasesId |  |
| double MoveRestrRadius |  |
| double MoveRestrRadius2 |  |
| double MoveDelayMean |  |
| double MoveRestrEffect |  |
| double MoveRestrDuration |  |
| double MoveRestrTimeStart |  |
| double AirportCloseTimeStart |  |
| double AirportCloseDuration |  |
| double AirportCloseEffectiveness |  |
| double CaseIsolationDuration |  |
| double CaseIsolationEffectiveness |  |
| double CaseIsolationHouseEffectiveness |  |
| double CaseIsolationDelay |  |
| double CaseIsolationPolicyDuration |  |
| double CaseIsolationProp |  |
| double HQuarantineTimeStart |  |
| double HQuarantineDelay |  |
| double HQuarantineHouseDuration |  |
| double HQuarantinePolicyDuration |  |
| double HQuarantinePropIndivCompliant |  |
| double HQuarantinePropHouseCompliant |  |
| double[] HQuarantinePlaceEffect | size=NUM_PLACE_TYPE |
| double HQuarantineSpatialEffect |  |
| double HQuarantineHouseEffect |  |
| int EnhancedSocDistClusterByHousehold |  |
| double SocDistTimeStart |  |
| double SocDistDuration |  |
| double SocDistHouseholdEffect |  |
| double[] SocDistPlaceEffect | size=NUM_PLACE_TYPES |
| double SocDistSpatialEffect |  |
| double EnhancedSocDistHouseholdEffect |  |
| double[] EnhancedSocDistPlaceEffect | size=NUM_PLACE_TYPES |
| double EnhancedSocDistSpatialEffect |  |
| double[] EnhancedSocDistProportionCompliant | size=NUM_AGE_GROUPS |
| double SocDistChangeDelay |  |
| double SocDistDuration2 |  |
| double SocDistHouseholdEffect2 |  |
| double[] SocDistPlaceEffect2 | size=NUM_PLACE_TYPES |
| double SocDistSpatialEffect2 |  |
| double EnhancedSocDistSpatialEffect |  |
| double EnhancedSocDistHouseholdEffect2 |  |
| double[] EnhancedSocDistPlaceEffect2 | size=NUM_PLACE_TYPES |
| double EnhancedSocDistSpatialEffect2 |  |
| double SocDistDurationCurrent |  |
| double SocDistHouseholdEffectCurrent |  |
| double[] SocDistPlaceEffectCurrent | size=NUM_PLACE_TYPES |
| double SocDistSpatialEffectCurrent |  |
| double EnhancedSocDistHouseholdEffectCurrent |  |
| double[] EnhancedSocDistPlaceEffectCurrent | size=NUM_PLACE_TYPES |
| double EnhancedSocDistSpatialEffectCurrent |  |
| double SocDistRadius |  |
| double SocDistRadius2 |  |


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