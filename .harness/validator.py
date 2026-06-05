import os
import json
import sys

def run_validation():
    harness_dir = os.path.dirname(os.path.abspath(__file__))
    root_dir = os.path.dirname(harness_dir)
    spec_path = os.path.join(harness_dir, 'spec.json')

    if not os.path.exists(spec_path):
        print(f"[FAIL] Spec file not found at {spec_path}")
        return False

    with open(spec_path, 'r', encoding='utf-8') as f:
        spec = json.load(f)

    print(f"Validating project: {spec.get('project_name', 'Unknown')}")
    all_passed = True

    for req in spec.get('requirements', []):
        req_id = req.get('id')
        desc = req.get('description')
        file_path = req.get('file_path')
        check_type = req.get('check_type')
        full_path = os.path.join(root_dir, file_path)

        print(f"Checking {req_id}: {desc}...", end=" ")

        if check_type == 'exists':
            if os.path.exists(full_path):
                print("PASSED")
            else:
                print("FAILED (File missing)")
                all_passed = False

        elif check_type == 'contains':
            if not os.path.exists(full_path):
                print("FAILED (File missing)")
                all_passed = False
                continue
            pattern = req.get('pattern')
            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()
            if pattern in content:
                print("PASSED")
            else:
                print(f"FAILED (Content pattern '{pattern}' not found)")
                all_passed = False

        elif check_type == 'function_returns':
            if not os.path.exists(full_path):
                print("FAILED (File missing)")
                all_passed = False
                continue
            func_name = req.get('function_name')
            expected = req.get('expected_value')
            try:
                # Add root to path to import dynamically
                sys.path.insert(0, root_dir)
                module_name = os.path.splitext(os.path.basename(file_path))[0]
                if module_name in sys.modules:
                    import importlib
                    importlib.reload(sys.modules[module_name])
                    module = sys.modules[module_name]
                else:
                    module = __import__(module_name)
                func = getattr(module, func_name)
                result = func()
                if result == expected:
                    print("PASSED")
                else:
                    print(f"FAILED (Got '{result}', expected '{expected}')")
                    all_passed = False
            except Exception as e:
                print(f"FAILED (Error: {e})")
                all_passed = False

    return all_passed

if __name__ == '__main__':
    success = run_validation()
    if success:
        print("[SUCCESS] All specifications validated successfully.")
        sys.exit(0)
    else:
        print("[FAIL] Validation failed. Codebase violates specifications.")
        sys.exit(1)
