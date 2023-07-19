# MotK Project: Mot Clé (Keyword) Finder

The MotK project is a Python library written in Rust that enables users to find keywords (mots clés) or lexical fields (champs lexicaux) for given text. The library provides a Python wrapper using the PyO3 library, allowing users to utilize MotK functionality in Python applications as well.

## Purpose

The purpose of the MotK library is to help users analyze and understand the content of texts by identifying relevant keywords or words belonging to specific lexical fields. This can be useful in various natural language processing tasks, information retrieval, content analysis, and more.

## Key Features

- Fast and Efficient: Built on Rust, MotK provides high-speed keyword extraction, making it well-suited for large-scale text analysis.
- Python Integration: The library offers a Python wrapper using PyO3, enabling seamless usage in Python applications.
- Customizable: Users can control the number of keywords to extract and tailor the library to specific use cases.
- Data-driven: Utilizes JSON data files to identify lexical fields, enhancing the accuracy of keyword extraction.
- Virtual Environment Support: Easy setup with Python virtual environments for clean development and deployment.

## Installation and Usage Guide

To utilize the MotK library effectively, follow the step-by-step guide below to build and install the Rust library within a Python virtual environment and run the Python example script.

### Step-by-Step Guide:

**Step 1: Clone the Repository**
```
git clone <repository_url>
cd motk-project
```

Replace `<repository_url>` with the URL of the Git repository containing the MotK project.

**Step 2: Create a New Virtual Environment**
```
python -m venv myenv
```

**Step 3: Activate the Virtual Environment**
- On Windows:
  ```
  myenv\Scripts\activate
  ```
- On macOS and Linux:
  ```
  source myenv/bin/activate
  ```

**Step 4: Install maturin**
```
pip install maturin
```

**Step 5: Build the Rust Library**
```
cd path/to/motk_code  # Change to the directory containing the pyo3 library code
maturin build --release
```

**Step 6: Install the Rust Library**
```
cd path/to/motk_project  # Change back to the main project directory
pip install path/to/motk-0.1.0-cp38-cp38-win_amd64.whl
```

Replace `path/to/motk-0.1.0-cp38-cp38-win_amd64.whl` with the actual path to the built wheel file.

**Step 7: Prepare Data Files**
Place the required data files (`DEM.json` and `french_stopwords.txt`) in the appropriate location. For example, you can create a `data` folder in the main project directory and put the files there.

**Step 8: Python Example Code**
Create a Python script to use the installed MotK library and read the data files. For example, you can create a file named `example.py` in the main project directory with the content provided below:

```python
import motk
import time

# Step 3: Load the MotK library
motk_finder = motk.MotKFinder("data/DEM.json", "data/french_stopwords.txt")

# Step 4: Replace the input_text with your desired text
input_text = """
Your input text goes here
"""

n = 5  # Number of top keywords to find

start = time.time()
keywords = motk_finder.find_keywords(input_text, n)
print("Keywords:", keywords)
end = time.time()
print("Time taken for processing:", end - start)
```

**Step 9: Run the Python Example**
```
python example.py
```

Ensure you have the correct file paths in the example script based on the actual location of your data files.

By following these steps, you will build and install the MotK Rust library within a Python virtual environment and use it in the Python example script to find keywords from the provided input text.