import numpy as np
from loops import remove_loops

if __name__ == "__main__":
    points = np.array([[0, 0], [1, 0], [1, 1], [0, 1], [0, 0], [1, 0], [1, 1], [0, 1], [0, 0]])
    print(remove_loops(points))