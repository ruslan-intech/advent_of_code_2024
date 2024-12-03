import re
import sys
import argparse
from dataclasses import dataclass

@dataclass
class Multiplication:
    first_number: int
    second_number: int
    position: int  # Added position to track order of instructions

def extract_multiplications(input_text: str) -> list[Multiplication]:
    # Pattern to match mul(num,num)
    mul_pattern = r'mul\((\d+),(\d+)\)'
    dont_pattern = r"don't\(\)"
    do_pattern = r'do\(\)'
    
    # Find all multiplications with their positions
    muls = [(int(m.group(1)), int(m.group(2)), m.start()) 
            for m in re.finditer(mul_pattern, input_text)]
    
    # Find all don't() positions
    dont_positions = [m.start() for m in re.finditer(dont_pattern, input_text)]
    
    # Find all do() positions
    do_positions = [m.start() for m in re.finditer(do_pattern, input_text)]
    
    # Combine all state changes (do/don't) and sort by position
    state_changes = [(pos, False) for pos in dont_positions] + [(pos, True) for pos in do_positions]
    state_changes.sort()
    
    # Start with multiplications enabled
    enabled = True
    valid_muls = []
    
    # Process each multiplication
    for first, second, pos in muls:
        # Update state based on any do/don't instructions before this position
        while state_changes and state_changes[0][0] < pos:
            _, new_state = state_changes.pop(0)
            enabled = new_state
            
        if enabled:
            valid_muls.append(Multiplication(first, second, pos))
    
    return valid_muls

def read_input_file(file_path: str) -> str:
    try:
        with open(file_path, 'r') as file:
            return file.read()
    except FileNotFoundError:
        print(f"Error: File '{file_path}' not found.")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading file: {e}")
        sys.exit(1)

def main():
    # Set up argument parser
    parser = argparse.ArgumentParser(description='Extract multiplication patterns from file')
    parser.add_argument('-f', '--file', required=True, help='Input file path')
    
    # Parse arguments
    args = parser.parse_args()
    
    # Read input file
    input_text = read_input_file(args.file)
    
    # Extract valid multiplications
    multiplications = extract_multiplications(input_text)
    
    # Print results
    print(f"Found {len(multiplications)} valid multiplication patterns in file: {args.file}")
    
    sum_mult = 0
    for i, mult in enumerate(multiplications, 1):
        print(f"Match {i}: mul({mult.first_number},{mult.second_number})")
        print(f"Result: {mult.first_number * mult.second_number}")
        print("---")
        sum_mult += (mult.first_number * mult.second_number)

    print(f"Total sum of multiplications: {sum_mult}")
    
if __name__ == "__main__":
    main()