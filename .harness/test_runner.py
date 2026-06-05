import os
import sys
import subprocess

def run_tests():
    harness_dir = os.path.dirname(os.path.abspath(__file__))
    root_dir = os.path.dirname(harness_dir)
    
    print("========================================")
    print("RUNNING HARNESS TEST RUNNER & VALIDATOR")
    print("========================================")
    
    # 1. Run validator.py
    validator_path = os.path.join(harness_dir, 'validator.py')
    print("Step 1: Running Specification Validator...")
    val_result = subprocess.run([sys.executable, validator_path], capture_output=False)
    
    # 2. Run unit tests if they exist
    print("\nStep 2: Running Unit Tests...")
    test_files = [f for f in os.listdir(root_dir) if f.startswith('test_') and f.endswith('.py')]
    unit_tests_failed = False
    
    if not test_files:
        print("No unit tests found (test_*.py in root). Skipping.")
    else:
        for tf in test_files:
            print(f"Running unit test: {tf}...")
            test_path = os.path.join(root_dir, tf)
            res = subprocess.run([sys.executable, test_path], capture_output=False)
            if res.returncode != 0:
                print(f"[FAIL] Unit test {tf} failed with return code {res.returncode}")
                unit_tests_failed = True
            else:
                print(f"[PASSED] Unit test {tf}")

    print("========================================")
    if val_result.returncode == 0 and not unit_tests_failed:
        print("[SUCCESS] All tests and specifications passed.")
        return True
    else:
        print("[FAIL] Test runner failed. Please fix code errors.")
        return False

if __name__ == '__main__':
    success = run_tests()
    if success:
        sys.exit(0)
    else:
        sys.exit(1)
