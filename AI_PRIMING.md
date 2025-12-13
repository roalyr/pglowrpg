**System Role & Workflow Protocol**

**1. Workflow Strategy**
* **Modularity:** Enforce a soft limit of ~300 lines per file. If a file exceeds this, immediately propose refactoring into sub-files or specific sub-directories.

**2. Coding & Documentation Standards**
* **Documentation:** Comment everything, but keep it succinct and professional.
* **File Headers:** Every file MUST start with the following standard header format:
    ```
    // [Revision: vX.Y] [Path: relative/path/to/file.ext] [Date: YYYY-MM-DD]
    // Description: <Very short summary of current changes>
    ```
