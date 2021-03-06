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
| int NKR | Size of kernel lookup table - arrays pre-populated by InitKernel |
| int NK_HR | Factor to expand hi-res kernel lookup table by |
| int MoveKernelType | Configurable: "Kernel type". |
| int AirportKernelType | |
| int KernelType | Used to init particular Kernel type, set from P.MoveKernelType, P.AirportKernelType and P.PlaceTypeKernelType[#place types] |
| unsigned int BinFileLen | Number of lines in density file |
| int DoBin | |
| int DoSaveSnapshot | |
| int DoLoadSnapshot | |
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
| int NMCP | Number of populated microcells |
| int ncw | number of cells across bounds |
| int nch | number of cells height of bounds |
| int nmcw | number of microcells across bounds (ncw * NMCL) |
| int nmch | number of microcells height of bounds (nch * NMCL) |
| int DoUTM_coords |  |
| int nsp |  |
| int DoSeasonality |  |
| int DoCorrectAgeDist |  |
| int DoPartialImmunity |  |
| int DoAdUnits |  |
| int NumAdunits | Number of Administrative Units |
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
| double LongitudeCutLine | Longitude to image earth is cut at to produce a flat map when parsing density file data.  Default -360 degrees (effectively -180).  Use to ensure countries have a contiguous boundary |
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
| double[][] HouseholdSizeDistrib | index 0 holds reference data of the proportion of households having n people. dims=MAX_ADUNITS * MAX_HOUSEHOLD_SIZE |
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
| int VaryEfficaciesOverTime |  |
| int Num_SD_ChangeTimes | must be at most MAX_NUM_INTERVENTION_CHANGE_TIMES |  
| double[] SD_ChangeTimes | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; change times for intensity of (enhanced) social distancing
| double[] SD_SpatialEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of SocDistSpatialEffectCurrent |
| double[] SD_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of SocDistHouseholdEffectCurrent |
| double[][] SD_PlaceEffects_OverTime | dims=MAX_NUM_INTERVENTION_CHANGE_TIMES * NUM_PLACE_TYPES; indexed by i) change time; ii) place type;  time-varying equivalent of SocDistPlaceEffectCurrent |
| double[] SD_CellIncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of SocDistCellIncThresh |
| double[] Enhanced_SD_SpatialEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of EnhancedSocDistSpatialEffectCurrent |
| double[] Enhanced_SD_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of EnhancedSocDistHouseholdEffectCurrent |
| double[][] Enhanced_SD_PlaceEffects_OverTime | dims=MAX_NUM_INTERVENTION_CHANGE_TIMES * NUM_PLACE_TYPES; indexed by i) change time; ii) place type;  time-varying equivalent of EnhancedSocDistPlaceEffectCurrent |
| int Num_CI_ChangeTimes | must be at most MAX_NUM_INTERVENTION_CHANGE_TIMES |
| double[] CI_ChangeTimes | change times for intensity of case isolation |
| double[] CI_SpatialAndPlaceEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of CaseIsolationEffectiveness |
| double[] CI_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of CaseIsolationHouseEffectiveness |
| double[] CI_Prop_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of CaseIsolationProp |
| double[] CI_CellIncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of CaseIsolation_CellIncThresh |
| int Num_HQ_ChangeTimes | max=MAX_NUM_INTERVENTION_CHANGE_TIMES
| double[] HQ_ChangeTimes | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; change times for intensity of household quarantine |
| double[] HQ_SpatialEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of HQuarantineSpatialEffect |
| double[] HQ_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of HQuarantineHouseEffect |
| double[][] HQ_PlaceEffects_OverTime | dims=MAX_NUM_INTERVENTION_CHANGE_TIMES * NUM_PLACE_TYPES; indexed by i) change time; ii) place type; time-varying equivalent of HQuarantinePlaceEffect |
| double[] HQ_Individual_PropComply_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of HQuarantinePropIndivCompliant |
| double[] HQ_Household_PropComply_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of HQuarantinePropHouseCompliant |
| double[] HQ_CellIncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of HHQuar_CellIncThresh |
| int Num_PC_ChangeTimes | max=MAX_NUM_INTERVENTION_CHANGE_TIMES |
| double[] PC_ChangeTimes | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; change times for intensity of place closure |
| double[] PC_SpatialEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseSpatialRelContact |
| double[] PC_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseHouseholdRelContact |
| double[][] PC_PlaceEffects_OverTime | dims=MAX_NUM_INTERVENTION_CHANGE_TIMES * NUM_PLACE_TYPES; indexed by i) change time; ii) place type; //// time-varying equivalent of PlaceCloseEffect |
| double[][] PC_PropAttending_OverTime | dims=MAX_NUM_INTERVENTION_CHANGE_TIMES * NUM_PLACE_TYPES |
| int[] PC_IncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseIncTrig / PlaceCloseIncTrig1 |
| double[] PC_FracIncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseFracIncTrig |
| int[] PC_CellIncThresh_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseCellIncThresh |
| double[] PC_Durs_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of PlaceCloseDuration |
| int Num_DCT_ChangeTimes | max=MAX_NUM_INTERVENTION_CHANGE_TIMES | 
| double[] DCT_ChangeTimes | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; change times for intensity of digital contact tracing |
| double[] DCT_SpatialAndPlaceEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of DCTCaseIsolationEffectiveness |
| double[] DCT_HouseholdEffects_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of DCTCaseIsolationHouseEffectiveness |
| double[] DCT_Prop_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES; time-varying equivalent of ProportionDigitalContactsIsolate |
| int DCT_MaxToTrace_OverTime | size=MAX_NUM_INTERVENTION_CHANGE_TIMES |
| double KeyWorkerProphTimeStart |  |
| double KeyWorkerProphDuration |  |
| double[] KeyWorkerPropInKeyPlaces | size=NUM_PLACE_TYPES |
| double KeyWorkerHouseProp |  |
| double KeyWorkerProphRenewalDuration |  |
| double KeyWorkerProphRadius |  |
| double KeyWorkerProphRadius2 |  |
| double TreatTimeStartBase |  |
| double VaccTimeStartBase |  |
| double MoveRestrTimeStartBase |  |
| double PlaceCloseTimeStartBase |  |
| double PlaceCloseTimeStartBase2 |  |
| double PlaceCloseTimeStartPrevious |  |
| double AirportCloseTimeStartBase |  |
| double HQuarantineTimeStartBase |  |
| double CaseIsolationTimeStartBase |  |
| double SocDistTimeStartBase |  |
| double KeyWorkerProphTimeStartBase |  |
| double DigitalContactTracingTimeStartBase |  |
| double InfectionImportRate1 |  |
| double InfectionImportRate2 |  |
| double InfectionImportChangeTime |  |
| double[] ImportInfectionTimeProfile | size=MAX_DUR_IMPORT_PROFILE |
| double PreControlClusterIdTime |  |
| double PreControlClusterIdCalTime |  |
| double PreControlClusterIdHolOffset |  |
| double PreIntervIdCalTime |  |
| double PreIntervTime |  |
| double SeedingScaling |  |
| int PreControlClusterIdCaseThreshold |  |
| int PreControlClusterIdUseDeaths |  |
| int PreControlClusterIdDuration |  |
| int DoAlertTriggerAfterInterv |  |
| int AlertTriggerAfterIntervThreshold |  |
| int StopCalibration |  |
| int ModelCalibIteration |  |
| int DoPerCapitaTriggers |  |
| int DoGlobalTriggers |  |
| int DoAdminTriggers |  |
| int DoICUTriggers |  |
| int MoveRestrCellIncThresh |  |
| int DoHQretrigger |  |
| int PlaceCloseCellIncThresh |  |
| int PlaceCloseCellIncThresh1 |  |
| int PlaceCloseCellIncThresh2 |  |
| int TriggersSamplingInterval |  |
| int PlaceCloseIndepThresh |  |
| int SocDistCellIncThresh |  |
| int[] VaccPriorityGroupAge | size=2 |
| int PlaceCloseCellIncStopThresh |  |
| int SocDistCellIncStopThresh |  |
| int[] PlaceCloseAdunitPlaceTypes | size=NUM_PLACE_TYPES |
| int DoPlaceCloseOnceOnly |  |
| int DoSocDistOnceOnly |  |
| int DoMoveRestrOnceOnly |  |
| int DoKeyWorkerProphOnceOnly |  |
| int VaccMaxRounds |  |
| int VaccByAdminUnit |  |
| int VaccAdminUnitDivisor |  |
| int TreatByAdminUnit |  |
| int TreatAdminUnitDivisor |  |
| int MoveRestrByAdminUnit |  |
| int MoveRestrAdminUnitDivisor |  |
| int PlaceCloseByAdminUnit |  |
| int PlaceCloseAdminUnitDivisor |  |
| int KeyWorkerProphCellIncThresh |  |
| int KeyWorkerPopNum |  |
| int[] KeyWorkerPlaceNum | size=NUM_PLACE_TYPES |
| int KeyWorkerNum |  |
| int KeyWorkerIncHouseNum |  |
| int DoBlanketMoveRestr |  |
| int PlaceCloseIncTrig |  |
| int PlaceCloseIncTrig1 |  |
| int PlaceCloseIncTrig2 |  |
| int TreatMaxCoursesPerCase |  |
| int DoImportsViaAirports |  |
| int DoMassVacc |  |
| int DurImportTimeProfile |  |
| unsigned short int usHQuarantineHouseDuration |  |
| unsigned short int usVaccTimeToEfficacy |  |
| unsigned short int usVaccTimeEfficacySwitch | us = unsigned short versions of their namesakes, multiplied by P.TimeStepsPerDay |
| unsigned short int usCaseIsolationDuration |  |
| unsigned short int usCaseIsolationDelay |  |
| unsigned short int usCaseAbsenteeismDuration |  |
| unsigned short int usCaseAbsenteeismDelay |  |
| int DoRecordInfEvents | DoRecordInfEvents and MaxInfEvents give the user a choice as to whether to output infection events as a line list |
| int MaxInfEvents |  |
| int RecordInfEventsPerRun |  |
| double KernelPowerScale |  |
| double KernelOffsetScale |  |
| int LimitNumInfections |  |
| int MaxNumInfections |  |
| int DoDigitalContactTracing |  |
| int ClusterDigitalContactUsers |  |
| int NDigitalContactUsers |  |
| int NDigitalHouseholdUsers |  |
| int FindContactsOfDCTContacts |  |
| int DoDCTTest |  |
| double PropPopUsingDigitalContactTracing |  |
| double ScalingFactorSpatialDigitalContacts |  |
| double ScalingFactorPlaceDigitalContacts |  |
| double DigitalContactTracingDelay |  |
| double LengthDigitalContactIsolation |  |
| double ProportionDigitalContactsIsolate |  |
| double ProportionSmartphoneUsersByAge | size=NUM_AGE_GROUPS |
| double DelayFromIndexCaseDetectionToDCTIsolation |  |
| double DelayToTestIndexCase |  |
| double DelayToTestDCTContacts |  |
| double SpecificityDCT |  |
| double SensitivityDCT |  |
| double DigitalContactTracingPolicyDuration |  |
| double DCTCaseIsolationHouseEffectiveness |  |
| double DCTCaseIsolationEffectiveness |  |
| int OutputDigitalContactTracing |  |
| int OutputDigitalContactDist |  |
| int IncludeHouseholdDigitalContactTracing |  |
| int IncludePlaceGroupDigitalContactTracing |  |
| int DCTIsolateIndexCases |  |
| int RemoveContactsOfNegativeIndexCase |  |
| int MaxDigitalContactsToTrace |  |
| int DoOriginDestinationMatrix |  |
| int DoInterventionDelaysByAdUnit |  |
| int OutputAge |  |
| int OutputR0 |  |
| int OutputControls |  |
| int OutputCountry |  |
| int OutputAdUnitVar |  |
| int OutputHousehold |  |
| int OutputInfType |  |
| int OutputNonSeverity |  |
| int OutputSeverityAdminUnit |  |
| int OutputSeverityAge |  |
| int OutputNonSummaryResults |  |
| int MeanChildAgeGap | Average gap between ages of children in a household, in years |
| int MinAdultAge | The youngest age, in years, at which someone is considered to be an adult |
| int MaxMFPartnerAgeGap | The largest number of years older than a female partner that a male partner can be |
| int MaxFMPartnerAgeGap | The largest number of years older than a male partner that a female partner can be |
| int MinParentAgeGap | The minimum number of years older than a child that a parent must be |
| int MaxParentAgeGap | The maximum number of years older than a child that a parent can be |
| int MaxChildAge | The maximum age, in years, of a child |
| double OneChildTwoPersProb |  |
| double TwoChildThreePersProb |  |
| double OnePersHouseProbOld |  |
| double TwoPersHouseProbOld |  |
| double OnePersHouseProbYoung |  |
| double TwoPersHouseProbYoung |  |
| double OneChildProbYoungestChildUnderFive |  |
| double TwoChildrenProbYoungestUnderFive |  |
| double ProbYoungestChildUnderFive |  |
| double ZeroChildThreePersProb |  |
| double OneChildFourPersProb |  |
| double YoungAndSingleSlope |  |
| int YoungAndSingle |  |
| int NoChildPersAge |  |
| int OldPersAge |  |
| double ThreeChildFivePersProb |  |
| int OlderGenGap |  |


Cell
- Array: Cells (count = P.NC)
- Pointer Array: CellLookup (references to populated cells only)

| Field | Represents |
|----------|------------|
| int n | | see setupmodel -> setuppopulation
| int S | susceptible? |
| int L | latent? |
| int I | infected? |
| int R | recovered? |
| int D | died? |
| int cumTC |  |
| int S0 | initial number of susceptibles within cell |
| int tot_treat | total number of people treated within cell |
| int tot_vacc | total number of people vaccinated within cell |
| int *members |  |
| int *susceptible |  |
| int *latent |  |
| int *infected |  |
| int *InvCDF |  |
| float tot_prob |  |
| float *cum_trans |  |
| float *max_trans |  |
| short[] CurInterv | size=MAX_INTERVENTION_TYPES |

Microcell
Array: Mcells (count = P.NMC)
Pointer Array: McellLookup (references to populated microcells only)

| Field | Represents |
|----------|------------|
| int n | Number of people in microcell |
| int adunit | administrative unit membership |
| unsigned short int | country membership |
| int*[] places | size=NUM_PLACE_TYPES |
| unsigned short int[] np | size=NUM_PLACE_TYPES |
| unsigned short int moverest |  |
| unsigned short int placeclose |  |
| unsigned short int socdist |  |
| unsigned short int keyworkerproph |  |
| unsigned short int move_trig |  |
| unsigned short int place_trig |  |
| unsigned short int socdist_trig |  |
| unsigned short int keyworkerproph_trig |  |
| unsigned short int move_start_time |  |
| unsigned short int move_end_time |  |
| unsigned short int place_end_time |  |
| unsigned short int socdist_end_time |  |
| unsigned short int keyworkerproph_end_time |  |
| unsigned short int treat |  |
| unsigned short int vacc |  |
| unsigned short int treat_trig |  |
| unsigned short int vacc_trig |  |
| unsigned short int treat_start_time |  |
| unsigned short int treat_end_time |  |
| unsigned short int vacc_start_time |  |
| IndexList* AirportList |  |


BinFile (used for density file)
File parsed as series of lines, all of form: x y pop cnt as
- Array BF (count = P.BinFileLen)
Note that multiple entries may end up aggregated into the same microcell

| Field | Represents |
|----------|------------|
| double x | x coordinate |
| double y | y coordinate |
| double pop | population density |
| int cnt | country |
| int ad | admin unit |

PopVar - Global model state
Instance: State
Per-Thread Instance: StateT
Includes several current totals and cumulative totals, including breakdowns by admin unit and age group.
Also some "queues" and "member arrays".

| Field | Represents |
|----------|------------|
| int S |  |
| int L |  |
| int I | infections |
| int R |  |
| int D |  |
| int cumI | cumulative infections |
| int cumR |  |
| int cumD |  |
| int cumC |  |
| int cumTC |  |
| int cumFC |  |
| int cumDC | cumulative detected cases |
| int trigDC |  |
| int cumH | Cumulative hospitalisations |
| int cumCT | cumulative contact tracing |
| int cumCC |  |
| int DCT | digital contact tracing |
| int cumDCT | cumulative digital contact tracing |
| int[] cumC_country | cumulative cases by country; size=MAX_COUNTRIES |
| int cumHQ |  |
| int cumAC |  |
| int cumAA |  |
| int cumAH |  |
| int cumACS |  |
| int cumAPC |  |
| int cumAPA |  |
| int cumAPCS |  |
| int[] cumIa | cumulative infections by age group; size=NUM_AGE_GROUPS |
| int[] cumCa | size=NUM_AGE_GROUPS |
| int[] cumDa | size=NUM_AGE_GROUPS |
| int[] cumI_adunit | size=MAX_ADUNITS |
| int[] cumC_adunit | size=MAX_ADUNITS |
| int[] cumD_adunit | size=MAX_ADUNITS |
| int[] cumT_adunit | size=MAX_ADUNITS |
| int[] cumH_adunit | cumulative hospitalisations per admin unit; size=MAX_ADUNITS |
| int[] cumDC_adunit | cumulative detected cases per admin unit; size=MAX_ADUNITS |
| int[] cumCT_adunit | cumulative contact tracing per admin unit; size=MAX_ADUNITS |
| int[] cumCC_adunit | size=MAX_ADUNITS |
| int[] trigDC_adunit | size=MAX_ADUNITS |
| int[] cumDCT_adunit | cumulative digital contact tracing per admin unit; size=MAX_ADUNITS |
| int[] DCT_adunit | overall digital contact tracing per admin unit; size=MAX_ADUNITS |
| int[] cumCT_adunit | size=MAX_ADUNITS |
| int[] cumItype | size=INFECT_TYPE_MASK |
| int[] cumI_keyworker | size=2 |
| int[] cumC_keyworker | size=2 |
| int[] cumT_keyworker | size=2 |
| Infection*[] inf_queue | "the queue (i.e. list) of infections. 1st index is thread, 2nd is person."; size=MAX_NUM_THREADS |
| int[] n_queue | number of infections in inf_queue; size=MAX_NUM_THREADS |
| int*[] p_queue | actual place queue, 1st index is place type, 2nd is place(i.e. list of places); size=NUM_PLACE_TYPES |
| int*[] pg_queue | actual place-group queue; size=NUM_PLACE_TYPES |
| int[] np_queue | number of places in place queue (by place type); size=NUM_PLACE_TYPES |
| int[] NumPlacesClosed | size=NUM_PLACE_TYPES |
| int n_mvacc |  | 
| int mvacc_cum |  | 
| float* cell_inf | List of spatial infectiousnesses by person within cell |
| double sumRad2 |  |
| double maxRad2 |  |
| double cumT |  |
| double cumV |  |
| double cumVG |  |
| double cumUT |  |
| double cumTP |  |
| double cumV_daily |  |
| double cumVG_daily |  |
| int* CellMemberArray |  |
| int* CellSuscMemberArray | An int for each person size=P.POP_SIZE |
| int** InvAgeDist |  |
| int* mvacc_queue |  |
| int[] nct_queue | queue for contact tracing; size=MAX_ADUNITS |
| ContactEvent*[] dct_queue | queues for digital contact tracing; size=MAX_ADUNITS |
| int[] ndct_queue | queues for digital contact tracing; size=MAX_ADUNITS |
| int[] contact_dist | contact distribution; size=MAX_CONTACTS+1 |
| double*[] origin_dest | intermediate storage for calculation of origin-destination matrix; size=MAX_ADUNITS |
| int Mild |  |
| int ILI |  |
| int SARI |  |
| int Critical |  |
| int CritRecov |  |
| int cumMild |  |
| int cumILI |  |
| int cumSARI |  |
| int cumCritical |  |
| int cumCritRecov |  |
| int[] Mild_adunit | size=MAX_ADUNITS |
| int[] ILI_adunit | size=MAX_ADUNITS |
| int[] SARI_adunit | size=MAX_ADUNITS |
| int[] Critical_adunit | size=MAX_ADUNITS |
| int[] CritRecov_adunit | size=MAX_ADUNITS |
| int[] cumMild_adunit | size=MAX_ADUNITS |
| int[] cumILI_adunit | size=MAX_ADUNITS |
| int[] cumSARI_adunit | size=MAX_ADUNITS |
| int[] cumCritical_adunit | size=MAX_ADUNITS |
| int[] cumCritRecov_adunit | size=MAX_ADUNITS |
| int[] Mild_age | size=NUM_AGE_GROUPS |
| int[] ILI_age | size=NUM_AGE_GROUPS |
| int[] SARI_age | size=NUM_AGE_GROUPS |
| int[] Critical_age | size=NUM_AGE_GROUPS |
| int[] CritRecov_age | size=NUM_AGE_GROUPS |
| int[] cumMild_age | size=NUM_AGE_GROUPS |
| int[] cumILI_age | size=NUM_AGE_GROUPS |
| int[] cumSARI_age | size=NUM_AGE_GROUPS |
| int[] cumCritical_age | size=NUM_AGE_GROUPS |
| int[] cumCritRecov_age | size=NUM_AGE_GROUPS |
| int cumDeath_ILI | cumulative deaths from ILI severity |
| int cumDeath_SARI | cumulative deaths from SARI severity |
| int cumDeath_Critical | cumulative deaths from Critical severity |
| int[] cumDeath_ILI_adunit | cumulative deaths from ILI severity by admin unit; size=MAX_ADUNITS |
| int[] cumDeath_SARI_adunit | cumulative deaths from SARI severity by admin unit; size=MAX_ADUNITS |
| int[] cumDeath_Critical_adunit | cumulative deaths from Critical severity by admin unit; size=MAX_ADUNITS |
| int[] cumDeath_ILI_age | cumulative deaths from ILI severity by age group; size=NUM_AGE_GROUPS |
| int[] cumDeath_SARI_age | cumulative deaths from SARI severity by age group; size=NUM_AGE_GROUPS |
| int[] cumDeath_Critical_age | cumulative deaths from Critical severity by age group; size=NUM_AGE_GROUPS |
