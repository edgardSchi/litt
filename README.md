# litt :fire:
A small testing framework written in Rust using stdin and stdout to test programs.
litt stands for "Language Independent Testing Tool" as it can be used to test any program.

## Test file
The config file for a test is a simple JSON file with the following structure:
```json=
{
    "executable": "python",
    "arguments": ["test.py"],
    "tests": [
        {
            "inputs" : ["3", "3"],
            "outputs": ["6"]
        },
        {
            "inputs" : ["4", "3"],
            "outputs": ["7"]
        },
        {
            "inputs" : ["7", "3"],
            "outputs": ["10"]
        },
        {
            "inputs" : ["1", "3"],
            "outputs": ["6"]
        }
    ]
}
```
This example file tests if the given program outputs the numbers from `outputs` with the specified numbers in `inputs` as input over stdin.

## Running litt
Clone the repo and run it with `cargo run <test_file>`.
