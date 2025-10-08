import numpy as np
import matplotlib.pyplot as plt

def do_stuff():
    a = np.random.uniform(-1.0, 1.0, 1000)
    b = np.outer(a, a)
    _c = np.dot(b, b)
    _d = np.dot(b, a)

    plt.imshow(b)
    plt.show(block=False)

do_stuff()
