import unittest
from dummy_app import get_status

class TestDummyApp(unittest.TestCase):
    def test_status(self):
        self.assertEqual(get_status(), "Harness Active")

if __name__ == '__main__':
    unittest.main()
