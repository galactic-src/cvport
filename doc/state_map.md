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
| int NRactual | |
| int NRactE | |
| int NRactNE | |
| int UpdatesPerSample | Number of time steps between samples |
| int NumSamples | Total number of samples that will be made |
| int KernelType | |
| int NKR | Size of kernel lookup table |
| int NK_HR | Factor to expand hi-res kernel lookup table by |
| int MoveKernelType | |
| int AirportKernelType | |
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