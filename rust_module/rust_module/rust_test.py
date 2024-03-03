import random
import time
import psutil
import math
# import rust_module

from rust_module import sum_as_string
from rust_module import create_facts_table
from rust_module import get_value
from rust_module import set_value

def convert_size(size_bytes):
    size_bytes = abs(size_bytes)
    
    if size_bytes == 0:
        return "0B"
    
    size_name = ("B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB")
    i = int(math.floor(math.log(size_bytes, 1024)))
    p = math.pow(1024, i)
    s = round(size_bytes / p, 2)

    return "%s %s" % (s, size_name[i])

def generate_address(dim_count: int):
    addr = list()

    for i in range(0, dim_count):
      value = random.randint(1, 10000)
      addr.append(value)

    return tuple(addr)

def fill_with_data(dim_count: int, cell_count: int, table_id: int):
  
  for i in range(1, cell_count+1):
    cell_value = random.randint(1, 100000) / 100000
    addr = generate_address(dim_count);
    set_value(table_id, addr, cell_value)

    if i % 10000000 == 0:
      print(f"Processed {i} values")


table = create_facts_table(5)
print(table)
print("Starting...")

initially_used_memory = psutil.Process().memory_info().rss

start = time.time()
fill_with_data(5, 500 * 1000000, table)
duration = time.time() - start

actual_memory_consumption = round(psutil.Process().memory_info().rss - initially_used_memory, 0)

print(f"Completed in {duration}. Memory: {convert_size(actual_memory_consumption)}")

  
