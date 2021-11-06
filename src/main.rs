use std::process::{Command, Stdio};
use std::io::{Write, BufReader, BufRead};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
struct Test {
    inputs: Vec<String>,
    outputs: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct TestFile {
    executable: String,
    arguments: Vec<String>,
    tests: Vec<Test>,
}

struct TestResult {
    total_tests: u16,
    successful_tests: u16,
    failed_tests: u16
}

impl TestResult {

    pub fn new() -> Self {
        Self {
            total_tests: 0,
            successful_tests: 0,
            failed_tests: 0
        }
    }

    pub fn add_successful_test(&mut self) {
        self.total_tests += 1;
        self.successful_tests += 1;
    }

    pub fn add_failed_test(&mut self) {
        self.total_tests += 1;
        self.failed_tests += 1;
    }
}

fn main() {

    let contents = check_test_file();
    let test_file = parse_test(&contents);
    let tests = test_file.tests;

    let mut result = TestResult::new();

    for t in tests {
        let outputs = run_child_with_input(&test_file.executable, &test_file.arguments, &construct_input_string(&t.inputs));
        check_test_case(&t, &outputs, &mut result);
    }

    println!("{} / {} tests successful!", result.successful_tests, result.total_tests);
}

fn check_test_case(test: &Test, outputs: &Vec<String>, result: &mut TestResult) -> bool {
    if &test.outputs == outputs {
        println!("Test case {:?} was correct!", test);
        result.add_successful_test();
        true
    } else {
        println!("Test case {:?} was wrong!", test);
        result.add_failed_test();
        false
    }
}

fn spawn_child(executable: &str, arguments: &Vec<String>) -> std::process::Child {
    let mut child = Command::new(executable);
    for arg in arguments {
        child.arg(arg);
    };
    child
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start process")
}

fn run_child_with_input(executable: &str, arguments: &Vec<String>, stdin_input: &str) -> Vec<String> {
    let mut child = spawn_child(executable, arguments);

    let stdin = child.stdin.as_mut().expect("failed to get stdin");
    let stdout = child.stdout.take().expect("failed to get stdout!");

    let reader = BufReader::new(stdout);
    
    let output_reader_thread = std::thread::spawn(move || {
        let mut outputs: Vec<String> = Vec::new();
        reader.lines().for_each(|line|  outputs.push(line.unwrap()));
        outputs
    });

    stdin.write_all((stdin_input).as_bytes()).expect("failed to write to stdin");

    child.wait_with_output().expect("Error with child process!");
    output_reader_thread.join().unwrap()
}

fn construct_input_string(inputs: &Vec<String>) -> String {
    inputs.join("\n")
}

fn parse_test(test_string: &str) -> TestFile {
    let tests: TestFile = serde_json::from_str(test_string).unwrap();
    tests
}

fn check_test_file() -> String {
    let filename : &str = "example_config.json";
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(string) => string,
        _ => panic!("Could not read config file!"),
    }
}
