ERQL-CLI 🦀

    ERQL-CLI (Education, Reporting & Query Language CLI) is a lightweight, high-performance command-line tool designed to automate LMS administration, scrape SCORM data, and run analytical queries.

Originally built in Python with a Tkinter GUI, this complete Rust rewrite delivers blazing speed, zero runtime dependencies, and a streamlined terminal interface.
🎯 The Mission & Motivation

Working as an LMS administrator in secure, restricted, or low-performance network environments highlights two massive roadblocks:

    The "IT Approval" Wall: Getting approval for heavy data visualization suites (like Power BI) is notoriously slow, and license costs are prohibitive for budget-constrained units.

    The Environment Struggle: Installing Python, managing virtual environments, and downloading massive libraries (pandas, scikit-learn) on restricted government computers is often impossible.

ERQL-CLI solves this by being:

    Zero-Dependency: It compiles down to a single, statically linked binary. Drag, drop, and run. No Python installation, no registry edits, no hassle.

    Intuitively Simple: Features a human-readable query language designed for fast data extraction and analysis without the steep learning curve.

    Air-Gap Friendly: Designed to work flawlessly offline on older or heavily restricted systems.

✨ Features

    🗣️ Human-Readable Query Language: Run powerful queries, filter datasets, and manipulate files using simple, expressive syntax.

    🌐 LMS & SCORM Scraping: Securely extract student progress, completion rates, and SCORM data from military and educational portals.

    📊 Fast Data Analysis: Perform built-in statistical analysis (regressions, clustering, and descriptive stats) directly from your terminal.

    📁 File & Data Scripting: Automate tedious file manipulation tasks and pipeline data without writing complex bash scripts.

    ⚡ Blazing Rust Performance: Sub-millisecond startup times and highly parallelized scraping/parsing, leaving the old Python implementation in the dust.
