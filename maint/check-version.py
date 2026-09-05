# Upgrade self's version
import toml

data = toml.load("Cargo.toml")

members = data["workspace"]["members"]
rust_pt_crates = [m for m in members if m.startswith("rust_pt/")]

root_version = data["workspace"]["package"]["version"]
deps = data["workspace"]["dependencies"]
pt_dep = {m: deps[m]["version"] for m in deps if m.startswith("pt_")}
for name, version in pt_dep.items():
    if root_version != version:
        raise ValueError(
            f"different version between:  {name}: {version} vs {root_version}"
        )
