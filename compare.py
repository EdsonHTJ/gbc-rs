import re

def extract_values(line):
    # Extracts both "ticks" and "PC" hexadecimal values from lines starting with "TICKS:"
    if not line.startswith("TICKS:"):
        return None, None
    ticks_match = re.search(r'TICKS: ([0-9A-Fa-f]+)', line)
    pc_match = re.search(r'PC: ([0-9A-Fa-f]+)', line)
    ticks = int(ticks_match.group(1), 16) if ticks_match else None
    pc = int(pc_match.group(1), 16) if pc_match else None
    return ticks, pc

def compare_files(file_path1, file_path2):
    with open(file_path1, 'r') as file1, open(file_path2, 'r') as file2:
        line_number1 = 1
        line_number2 = 1
        while True:
            line1 = file1.readline()
            line2 = file2.readline()

            ticks1, pc1 = extract_values(line1)
            ticks2, pc2 = extract_values(line2)

            # Skip to the next line for any file where the line doesn't start with "TICKS:"
            while (ticks1 is None or pc1 is None) and line1:
                line1 = file1.readline()
                line_number1 += 1
                ticks1, pc1 = extract_values(line1)

            while (ticks2 is None or pc2 is None) and line2:
                line2 = file2.readline()
                line_number2 += 1
                ticks2, pc2 = extract_values(line2)

            # If either file has ended, stop the loop
            if not line1 or not line2:
                break

            # Check for mismatch in ticks or PC
            if ticks1 != ticks2:
                print(f"Mismatch in TICKS at line {line_number1} (File 1) and line {line_number2} (File 2):")
                print(f"File 1 PC: {hex(pc1)}, File 2 PC: {hex(pc2)}")
                print(f"File 1 TICKS: {hex(ticks1)}, File 2 TICKS: {hex(ticks2)}")
                return

            if pc1 != pc2:
                print(f"Mismatch in PC at line {line_number1} (File 1) and line {line_number2} (File 2):")
                print(f"File 1 PC: {hex(pc1)}, File 2 PC: {hex(pc2)}")
                print(f"File 1 TICKS: {hex(ticks1)}, File 2 TICKS: {hex(ticks2)}")

                return

            line_number1 += 1
            line_number2 += 1

        else:
            print("No mismatches found.")
# Example usage
file_path1 = '../LLD_gbemu/part10/out3.txt'
file_path2 = './out_test.txt'
compare_files(file_path1, file_path2)
