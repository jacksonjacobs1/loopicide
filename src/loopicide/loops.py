import numpy as np
from shapely.geometry import LineString

def segments_intersect(s1, s2):
    return LineString(s1).intersects(LineString(s2))

def get_segments_from_points(points):
    segments = []
    for i in range(len(points)-1):
        segments.append(np.array([points[i], points[i+1]]))
    return segments

def get_points_from_segments(segments):
    points = []
    for s in segments:
        points.append(s[0])
    points.append(segments[-1][1])
    return np.array(points)

def remove_loops(points):
    stack = []
    loops = []

    points_copy = np.copy(points)

    segments = get_segments_from_points(points)
    for i, s1 in enumerate(segments):
        for j, s2 in enumerate(segments):
            if segments_intersect(s1, s2): 
                if j > i + 1:   # if the segments are not adjacent we add the first segment to the stack
                    stack.append(i)
                elif j < i - 1:
                    loopstart = stack.pop()
                
                    if len(stack) == 0:
                        loops.append((loopstart, i))

    
    for i in reversed(loops):
        points_copy = np.delete(points_copy, slice(i[0]+1, i[1]+1), axis=0) # not very efficient but it works

    return points_copy