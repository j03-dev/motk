# MotK Project: Mot Clé (Keyword) Finder

The MotK project is a Rust library that enables users to find keywords (mots clés) or lexical fields (champs lexicaux) for given text. The library provides a Python wrapper using the PyO3 library, allowing users to utilize MotK functionality in Python applications as well.

## Purpose

The purpose of the MotK library is to help users analyze and understand the content of texts by identifying relevant keywords or words belonging to specific lexical fields. This can be useful in various natural language processing tasks, information retrieval, content analysis, and more.

## Installation and Usage Guide

To utilize the MotK library effectively, follow the step-by-step guide below to build and install the Rust library within a Python virtual environment and run the Python example script.

### Step-by-Step Guide:

Step 1: Clone the Repository
git clone <repository_url>
cd motk-project

Step 2: Create a New Virtual Environment
python -m venv myenv

Step 3: Activate the Virtual Environment
- On Windows:
  myenv\Scripts\activate
- On macOS and Linux:
  source myenv/bin/activate

Step 4: Install maturin
pip install maturin

Step 5: Build the Rust Library
cd path/to/motk_code  # Change to the directory containing the pyo3 library code
maturin build --release

Step 6: Install the Rust Library
cd path/to/motk_project  # Change back to the main project directory
pip install path/to/motk-0.1.0-cp38-cp38-win_amd64.whl
Replace path/to/motk-0.1.0-cp38-cp38-win_amd64.whl with the actual path to the built wheel file.

Step 7: Prepare Data Files
Place the required data files (DEM.json and french_stopwords.txt) in the appropriate location. For example, you can create a data folder in the main project directory and put the files there.

Step 8: Python Example Code
Create a Python script to use the installed MotK library and read the data files. For example, you can create a file named example.py in the main project directory with the content provided below.

Step 9: Run the Python Example
python example.py

Ensure you have the correct file paths in the example script based on the actual location of your data files.

By following these steps, you will build and install the MotK Rust library within a Python virtual environment and use it in the Python example script to find keywords from the provided input text.
