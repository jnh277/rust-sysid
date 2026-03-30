import numpy as np
import rust_sysid

from time import perf_counter

"""
What we learnt form this example is that either we need to force the user to swap the memory ordering
using np.asfortranarray before calling (which Is a bad idea for usability)

Or we have to swap it when we receive the data in rust. Which while adds a slight overhead. Is going to be
better
"""

a = np.random.randn(1000, 1000)
b = np.random.randn(1000, 1000)

a2 = np.asfortranarray(a)
b2 = np.asfortranarray(b)

t0 = perf_counter()
result = rust_sysid.matmul(a, b)

t1 = perf_counter()
result_check = a @ b
t2 = perf_counter()

result2 = rust_sysid.matmul_fast(a2, b2)
t3 = perf_counter()

print(f"numpy result {result_check}")
print(f"rust result {result}")
print(f"rust fast {result2}")

print(f"numpy time taken {t2 - t1}")
print(f"rust time taken {t1 - t0}")
print(f"rust fast time taken {t3-t2}")