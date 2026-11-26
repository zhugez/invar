# Ordering fixture

report.py promises alphabetically sorted names. conventional_check.py checks membership and count. It deliberately omits ordering. Reversing the sort still passes those conventional checks.

Invar records the actual command output, so it detects the ordering change after approval. Adding an ordering assertion to the conventional test would also catch it. This fixture demonstrates complementary observed coverage, not testing superiority.

From the repository root, run python3 scripts/demo.py --invar target/debug/invar after cargo build. The demo copies the fixture into a clean temporary project, checks the candidate's bytes before explicit approval, detects the regression, repairs it, and removes the temporary project.
