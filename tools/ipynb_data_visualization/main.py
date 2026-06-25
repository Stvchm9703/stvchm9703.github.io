"""Tool root entry point - delegates to the packaged CLI."""

from ipynb_data_visualization import main

if __name__ == "__main__":
    import sys

    sys.exit(main())
