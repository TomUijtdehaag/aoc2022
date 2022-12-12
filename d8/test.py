import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt

with open("input.txt") as f:
    lines = f.read().split("\n")

grid = []

for line in lines:
    treeline = np.array([int(tree) for tree in line])

    grid.append(treeline)

grid = np.array(grid)

sns.heatmap(grid)
plt.show()
