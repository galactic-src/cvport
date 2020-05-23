#How Data Setup works for the sim

This file is to build a commentary on how the code in SetupModel.cpp works.
I will build it in the order the code executes, and build it far enough to understand the meaning of variables that I am populating in state_map.md.
I expect it will be most straightforward to add features to this project in a similar order to the initialisation process, since that will take into account any dependencies/references, however they are stored.

### Relevant Snippets before SetupModel()
- P.cwidth, used for both cell width and height, is read from param file 'Grid size', defaulting to 1/120.
- P.SpatialBoundingBox (4 values) is read from param file 'Spatial domain for simulation', defaulting on both axes to 0.0->1.0

### SetupModel() Part 1

##### Init Random number generation

- Allocates storage arrays for ranf (random number generation?) with enough space for MAX_NUM_THREADS * CACHE_LINE_SIZE longs.
- Calls setall() with setupSeed1 and setupSeed2.

##### Setup simulation area IF using population density data

- if doing admin unit boundaries, update SpatialBoundingBox to precisely incorporate all population and update admin unit counts
- set cell height = cell width
- temporarily normalise P.SpatialBoundingBox to a whole number of cells, now their size is established (SpatialBoundingBox = [x_min, y_min, x_max, y_max])
- temporarily set P.width and P.height according to calculated SpatialBoundingBox deltas
- set P.ncw and P.nch. Divides number of cells by 4, rounds up and multiplies by 4. This ensures the total number of cells vertically and horizontally is a multiple of 4. Unclear why.
- update P.width and P.height according to the normalised number of cells.
- update P.SpatialBoundingBox to reflect the new P.width and P.height
- set P.NC, total number of cells from ncw and nch

##### Setup simulation area IF assuming constant population density

This looks broken - we immediately read P.NC even though it has been set to -1 in CovidSim.cpp?
Need to run something without a population file and check logging (added to README.md)

##### Propagate normalised values

- set microcell totals (NMC, nmcw, nmch, mcwidth, mcheight) according to NMCL and NC
- set bitmap specs (bwidth, bheight, bheight2, scalex, scaley, bminx, bminy) according to width, height, BitmapScale, BitmapAspectScale and BoundingBox[]
- normalise LocationInitialInfection values according to SpatialBoundingBox
- set KernelDelta (max distance/NKR (kernel resolution)). Max distance is either maximum of diagonal, straight up height or straight across width, using dist2_raw (quartered if DoPeriodicBoundaries).

##### call SetupPopulation(DensityFile, SchoolFile, RegDemogFile)

- allocate for `Cells` (Cell[P.NC]), `Mcells` (Microcell[P.NMC]), `mcell_num` (int[P.NMC]), `mcell_dens` (double[P.NMC]), `mcell_country` (int[P.NMC]), `mcell_adunits` (int[P.NMC])
- if not handling varied population density, all mcell_dens set to 1.0 and all Mcells[].country set to 1
- otherwise use density file data (in `BF` of length `P.BinFileLength`):
  - iterate over entries
  - each x/y coordinate is resolved to a microcell. The pop field is averaged by keeping a total density and a tally of how many entries resolved to the microcell. This is then divided to calculate an average.
  - maxd holds the total of all densities, once averaging is complete.
  - then for each microcell, the resolved density is divided by the total density to give a proportion of the population to assign to the microcell.
  - t holds the proportion of the population remaining to assign. m tracks how many people have been assigned.
  - s is a probability for a binary sample. It is the ratio of remaining population to assign to total proportion of the population that should be in the microcell. So if 10% of the population should be in the microcell and 15% remains to be allocated, each remaining person gets a 10/15 = 2/3 chance of being allocated to the microcell.
  - s is capped at 1.0 (could be greater if we happened to overallocate significantly). In that case, every remaining person is allocated to this microcell (e.g. 10% of people should be in the microcell and only 9% of the population remains to be allocated).
- all but the last microcell are iterated over, then the final one is assigned the remainder of the population.
- total person-containing cells tallied in P.NMCP. Number of people assigned stored in Mcells[i].n
- 
