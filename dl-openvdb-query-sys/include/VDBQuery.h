#ifndef __VDBQuery_h
#define __VDBQuery_h

#include "dlInterface.h"

#include "stddef.h"

#ifdef  __cplusplus
extern "C" {
#endif

/*
	This returns the world space bounding box of the given file. It is written
	to bbox as: xmin ymin zmin xmax ymax zmax
*/
DL_INTERFACE bool DlVDBGetFileBBox(
	const char *filename,
	double *bbox );

DL_INTERFACE bool DlVDBGetGridNames(
	const char *filename,
	int *num_grids,
	const char *const **grid_names );

DL_INTERFACE void DlVDBFreeGridNames(
	const char *const *grid_names );

DL_INTERFACE void DlVDBGeneratePoints(
	const char *filename,
	const char *densitygrid,
	size_t *num_points,
	const float **points);

DL_INTERFACE void DlVDBFreePoints(
	const float *points);

#ifdef __cplusplus
}
#endif

#endif
