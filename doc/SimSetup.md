#How Data Setup works for the sim

This file is to build a commentary on how the code in SetupModel.cpp works.
I will build it in the order the code executes, and build it far enough to understand the meaning of variables that I am populating in state_map.md.
I expect it will be most straightforward to add features to this project in a similar order to the initialisation process, since that will take into account any dependencies/references, however they are stored.

### Relevant Snippets before SetupModel()
- P.cwidth, used for both cell width and height, is read from param file 'Grid size', defaulting to 1/120.
- P.SpatialBoundingBox (4 values) is read from param file 'Spatial domain for simulation', defaulting on both axes to 0.0->1.0

### SetupModel() Part 1

#####Init Random number generation

- Allocates storage arrays for ranf (random number generation?) with enough space for MAX_NUM_THREADS * CACHE_LINE_SIZE longs.
- Calls setall() with setupSeed1 and setupSeed2.

#####Setup simulation area IF using population density data

- set cell height = cell width
- temporarily normalise P.SpatialBoundingBox to a whole number of cells, now their size is established (SpatialBoundingBox = [x_min, y_min, x_max, y_max])
- temporarily set P.width and P.height according to calculated SpatialBoundingBox deltas
- set P.ncw and P.nch. Divides number of cells by 4, rounds up and multiplies by 4. This ensures the total number of cells vertically and horizontally is a multiple of 4. Unclear why.
- update P.width and P.height according to the normalised number of cells.
- update P.SpatialBoundingBox to reflect the new P.width and P.height
- set P.NC, total number of cells from ncw and nch

#####Setup simulation area IF assuming constant population density

