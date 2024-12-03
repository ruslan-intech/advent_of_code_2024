import re
import sys
import argparse
from dataclasses import dataclass

@dataclass
class Multiplication:
    first_number: int
    second_number: int

def extract_multiplications(input_text: str) -> list[Multiplication]:
    # Pattern to match mul(num,num)
    pattern = r'mul\((\d+),(\d+)\)'
    
    # Find all matches
    matches = re.finditer(pattern, input_text)
    
    # Convert matches to Multiplication objects
    multiplications = []
    for match in matches:
        first_num = int(match.group(1))
        second_num = int(match.group(2))
        multiplications.append(Multiplication(first_num, second_num))
    
    return multiplications

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
    
    # Extract multiplications
    multiplications = extract_multiplications(input_text)
    
    # Print results
    print(f"Found {len(multiplications)} multiplication patterns in file: {args.file}")
    
    sum_mult = 0
    for i, mult in enumerate(multiplications, 1):

        print(f"Match {i}: mul({mult.first_number},{mult.second_number})")
        print(f"Result: {mult.first_number * mult.second_number}")
        print("---")
        sum_mult += (mult.first_number * mult.second_number)

    print(f"Total sum of multiplications: {sum_mult}")
    
if __name__ == "__main__":
    main()