#!/usr/bin/env python3
import json

# File Handle
pkg_file = open("pkg/package.json", "r+")

# Load
package = json.load(pkg_file)

# Alter
package['name'] = "@monadic_cat/mice-js"

# Replace
pkg_file.seek(0)

json.dump(package, pkg_file, indent = 2)

pkg_file.truncate()
