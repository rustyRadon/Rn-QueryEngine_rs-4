import struct
import random

def generate_relational_data(num_rows=1000000):
    print(f"Generating {num_rows} rows with Age, Dept, and Salary...")
    
    ids = []
    ages = []
    dept_ids = []
    salaries = []

    for i in range(num_rows):
        ids.append(i)
        ages.append(random.randint(22, 65)) # New Column
        dept_ids.append(random.randint(1, 5))
        salaries.append(random.uniform(30000, 150000))

    with open("id.bin", "wb") as f:
        for val in ids: f.write(struct.pack("i", val))

    with open("age.bin", "wb") as f: # New File
        for val in ages: f.write(struct.pack("i", val))

    with open("dept_id.bin", "wb") as f:
        for val in dept_ids: f.write(struct.pack("i", val))

    with open("salary.bin", "wb") as f:
        for val in salaries: f.write(struct.pack("d", val))

    print("Success: id.bin, age.bin, dept_id.bin, salary.bin created.")

if __name__ == "__main__":
    generate_relational_data()