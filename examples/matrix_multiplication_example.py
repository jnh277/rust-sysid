import numpy as np
import rust_sysid

from time import perf_counter

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