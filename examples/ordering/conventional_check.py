from report import names

result = names()
assert len(result) == 3
assert set(result) == {"amber", "birch", "cedar"}
print("Conventional membership/count checks passed.")
